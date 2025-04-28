use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use actix_files::NamedFile;
use actix_multipart::form::MultipartForm;
use actix_web::{
    HttpMessage, HttpRequest, HttpResponse, Responder,
    cookie::Cookie,
    get, post,
    web::{Data, Json},
};
use chrono::Utc;
use serde_json::json;
use sqlx::SqlitePool;
use tokio::fs::{self, read_dir};

use crate::{
    AppState,
    db::{
        add_user, delete_user, query_user, upadta_avator, update_bg, update_username,
        username_exist,
    },
    gnid::{get_pstr, is_in_udir},
    types::{
        CopyPayload, CreateDirPayload, DeletePayload, LoginReq, MoveFilePayload, RegistryReq,
        RenamePayload, SetAvatorPayload, SetBgPayload, TokenBody, UnameReq, UploadFileForm, User,
    },
};

#[post("/rguser")]
pub async fn registry(
    req: HttpRequest,
    body: Json<RegistryReq>,
    pool: Data<SqlitePool>,
) -> impl Responder {
    let ext = req.extensions();
    let tkb: &TokenBody = ext.get().unwrap();
    if tkb.identity != "manager" {
        return HttpResponse::BadRequest().json(json!({"msg":"Not a manager"}));
    }
    let r = username_exist(&body.username, &pool).await.unwrap().0;
    if r {
        return HttpResponse::BadRequest().json(json!({"msg":"user exists"}));
    };
    let user = User {
        username: body.username.clone(),
        password: body.password.clone(),
        identity: String::from("user"),
        avator: Some("/avators/default.jpg".to_string()),
        disksize: body.disksize,
        bg: Some("/bg.png".to_string()),
    };
    if let Err(_e) = add_user(user, &pool).await {
        return HttpResponse::BadRequest().json(json!({"msg":"fail to save user"}));
    }
    fs::create_dir(format!("store/{}", body.username))
        .await
        .expect("msg");
    HttpResponse::Ok().json(json!({"msg":"Success"}))
}

#[get("/rmuser/{username}")]
pub async fn rmuser(req: HttpRequest, pool: Data<SqlitePool>) -> impl Responder {
    let ext = req.extensions();
    let tkb: &TokenBody = ext.get().unwrap();

    let name = req.match_info().query("username");
    if tkb.username != name && tkb.identity != "manager" {
        return HttpResponse::BadRequest().json(json!({"msg":"No power"}));
    }
    let r = delete_user(name, &pool).await;
    if let Err(_) = r {
        return HttpResponse::BadRequest().json(json!({"msg":"Failed to remove"}));
    }
    HttpResponse::Ok().json(json!({"msg":"Success"}))
}

#[post("/login")]
pub async fn login(
    body: Json<LoginReq>,
    pool: Data<SqlitePool>,
    appstate: Data<AppState>,
) -> impl Responder {
    let r = username_exist(&body.username, &pool).await.unwrap().0;
    if !r {
        return HttpResponse::BadRequest().json(json!({"msg":"user doesn't exists"}));
    }
    let r = query_user(&body.username, &pool).await;
    if let Err(e) = r {
        println!("{:?}", e);
        return HttpResponse::BadRequest().json(json!({"msg":"get users err"}));
    }
    let r = r.unwrap();
    if &r.password != &body.password {
        return HttpResponse::BadRequest().json(json!({"msg":"wrong password"}));
    }
    let tkb = TokenBody {
        username: r.username,
        exp: (Utc::now() + Duration::from_secs(60 * 60 * 24 * 165)).timestamp() as usize,
        identity: r.identity,
        disksize: r.disksize,
    };
    let tkbstr = appstate.encodetoken(tkb);
    let cookie = Cookie::build("token", tkbstr).path("/").finish();
    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"msg":"Success","data":{
            "avator":r.avator
        }}))
}

#[get("/userinfo")]
pub async fn userinfo(req: HttpRequest, pool: Data<SqlitePool>) -> impl Responder {
    let ext = req.extensions();
    let tkb: &TokenBody = ext.get().unwrap();
    let user = query_user(&tkb.username, &pool).await;
    if let Err(e) = user {
        return HttpResponse::BadRequest().json(json!({"msg":e.to_string()}));
    }
    let user = user.unwrap();
    HttpResponse::Ok()
        .json(json!({"msg":"Success","data":{"username":user.username,"avator":user.avator,"disksize":user.disksize,"identity":user.identity,"bg":user.bg}}))
}

#[post("/upload_username")]
pub async fn upload_username(
    req: HttpRequest,
    body: Json<UnameReq>,
    pool: Data<SqlitePool>,
) -> impl Responder {
    let ext = req.extensions();
    let tkb: &TokenBody = ext.get().unwrap();
    if let Err(e) = update_username(&tkb.username, &body.username, &pool).await {
        return HttpResponse::BadRequest().json(json!({"msg":e.to_string()}));
    }
    return HttpResponse::Ok().json(json!({"msg":"Success"}));
}

#[post("/set_avator")]
pub async fn upload_avator(
    req: HttpRequest,
    data: Json<SetAvatorPayload>,
    pool: Data<SqlitePool>,
) -> HttpResponse {
    let ext = req.extensions();
    let tk: &TokenBody = ext.get().unwrap();
    let ap = data.avator_path.clone();
    let r = (|| {
        let pfs = vec![".jpep", ".png", ".jpg", ".webp"];
        for i in pfs {
            if ap.ends_with(i) {
                return true;
            }
        }
        return false;
    })();
    if !r {
        return HttpResponse::Ok().json(json!({"msg":"wrong file"}));
    }
    upadta_avator(&ap, &tk.username, &pool).await.unwrap();
    HttpResponse::Ok().json(json!({"msg":"Success"}))
}

#[post("/set_bg")]
pub async fn upload_bg(
    req: HttpRequest,
    data: Json<SetBgPayload>,
    pool: Data<SqlitePool>,
) -> HttpResponse {
    let ext = req.extensions();
    let tk: &TokenBody = ext.get().unwrap();
    let ap = data.path.clone();
    let r = (|| {
        let pfs = vec![".jpep", ".png", ".jpg", ".webp"];
        for i in pfs {
            if ap.ends_with(i) {
                return true;
            }
        }
        return false;
    })();
    if !r {
        return HttpResponse::Ok().json(json!({"msg":"wrong file"}));
    }
    update_bg(&ap, &tk.username, &pool).await.unwrap();
    HttpResponse::Ok().json(json!({"msg":"Success"}))
}

#[get("/user_dir")]
pub async fn user_dir(req: HttpRequest, app: Data<AppState>) -> impl Responder {
    let ext = req.extensions();
    let tk: &TokenBody = ext.get().unwrap();
    let path = format!("{}\\{}", app.store_path, tk.username);
    let pb = Path::new(&path);
    if !pb.exists() {
        fs::create_dir(pb).await.unwrap();
    }
    HttpResponse::Ok().json(json!({
        "msg":"Success",
        "user_dir":{
            "name":tk.username,
            "path":path,
            "is_dir":true
        }
    }))
}

#[get("/open/{path:.*}")]
pub async fn open(req: HttpRequest, app: Data<AppState>) -> impl Responder {
    let rpath = req.match_info().query("path");
    let ext = req.extensions();
    let tkb: &TokenBody = ext.get().unwrap();
    let sr = Path::new(&app.store_path).join(&tkb.username);
    if !is_in_udir(rpath, &sr) {
        return HttpResponse::BadRequest().json(json!({"msg":"invalid path"}));
    }
    let path = std::path::Path::new(rpath);
    if path.is_dir() {
        let mut r = read_dir(path).await.unwrap();
        let mut rows = Vec::new();
        while let Some(e) = r.next_entry().await.unwrap() {
            rows.push(json!({
                "name":e.file_name().to_string_lossy(),
                "is_dir":e.path().is_dir(),
                "path":get_pstr(e.path())
            }
            ));
        }
        return HttpResponse::Ok().json(json!({
            "msg":"Success",
            "rows":rows
        }));
    }
    let nf = NamedFile::open_async(path)
        .await
        .expect("Load User Dir Failed");
    nf.into_response(&req)
}

#[get("/sysinfo")]
pub async fn sysinfo(req: HttpRequest) -> impl Responder {
    let ext = req.extensions();
    let tkb: &TokenBody = ext.get().unwrap();
    if tkb.identity != "manager" {
        return HttpResponse::BadRequest().json(json!({"msg":"No Permission"}));
    }

    HttpResponse::Ok().json(json!({}))
}

#[post("/rename")]
async fn rename_file(
    data: Json<RenamePayload>,
    req: HttpRequest,
    app: Data<AppState>,
) -> HttpResponse {
    let ext = req.extensions();
    let tkb: &TokenBody = ext.get().unwrap();
    let upath = format!("{}\\{}", app.store_path, tkb.username);
    if !is_in_udir(&data.old_path, &upath) && tkb.identity != "manager" {
        return HttpResponse::BadRequest().json(json!({"msg":"No Permission"}));
    }
    let old_path = PathBuf::from(&data.old_path);
    let new_path = old_path
        .parent()
        .map(|parent| parent.join(&data.new_name))
        .unwrap_or_else(|| PathBuf::from(&data.new_name));
    if old_path.parent() != new_path.parent() && tkb.identity != "manager" {
        return HttpResponse::InternalServerError()
            .json(json!({ "msg": "Over Permission operation" }));
    }
    if let Err(e) = fs::rename(&old_path, &new_path).await {
        return HttpResponse::InternalServerError()
            .json(json!({ "msg": format!("Rename failed: {}", e) }));
    }
    HttpResponse::Ok().json(json!({ "msg": "Renamed successfully" }))
}

#[post("/move")]
pub async fn move_file(
    req: HttpRequest,
    data: Json<MoveFilePayload>,
    app: Data<AppState>,
) -> impl Responder {
    let ext = req.extensions();
    let tkb: &TokenBody = ext.get().unwrap();
    let upath = format!("{}\\{}", app.store_path, tkb.username);
    let op = data.old_path.clone();
    let np = data.new_path.clone();
    if !is_in_udir(&op, &upath) || !is_in_udir(&np, &upath) {
        return HttpResponse::InternalServerError()
            .json(json!({ "msg": "Over Permission operation" }));
    }
    let r = fs::rename(op, np).await;
    if let Err(e) = r {
        return HttpResponse::BadRequest().json(json!({"msg":e.to_string()}));
    }
    HttpResponse::Ok().json(json!({"msg":"Success"}))
}

#[post("/delete")]
async fn delete_file(
    data: Json<DeletePayload>,
    req: HttpRequest,
    app: Data<AppState>,
) -> HttpResponse {
    let ext = req.extensions();
    let tkb: &TokenBody = ext.get().unwrap();
    let upath = format!("{}\\{}", app.store_path, tkb.username);
    if !is_in_udir(&data.path, &upath) && tkb.identity != "manager" {
        return HttpResponse::BadRequest().json(serde_json::json!({ "msg": "No permission" }));
    }
    let path = PathBuf::from(&data.path);
    if !path.exists() {
        return HttpResponse::BadRequest().json(serde_json::json!({ "msg": "文件路径不存在" }));
    }
    let result = if path.is_dir() {
        fs::remove_dir_all(&path).await
    } else {
        fs::remove_file(&path).await
    };

    match result {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "msg": "删除成功" })),
        Err(e) => HttpResponse::InternalServerError()
            .json(serde_json::json!({ "msg": format!("删除失败: {}", e) })),
    }
}

#[post("/copy")]
pub async fn copy_file(
    data: Json<CopyPayload>,
    req: HttpRequest,
    app: Data<AppState>,
) -> impl Responder {
    let ext = req.extensions();
    let tkb: &TokenBody = ext.get().unwrap();
    let upath = format!("{}\\{}", app.store_path, tkb.username);
    if (!is_in_udir(&data.from, &upath) || !is_in_udir(&data.to, &upath))
        && tkb.identity != "manager"
    {
        return HttpResponse::BadRequest().json(json!({"msg":"No Permission"}));
    }
    let r = fs::copy(data.from.clone(), data.to.clone()).await;
    if let Err(e) = r {
        println!("{:?}", e);
        return HttpResponse::BadRequest().json(json!({"msg":"Failed to copy it"}));
    }
    HttpResponse::Ok().json(json!({"msg":"Success"}))
}

#[post("/upload")]
pub async fn upload_file(
    req: HttpRequest,
    payload: MultipartForm<UploadFileForm>,
    app: Data<AppState>,
) -> impl Responder {
    let ext = req.extensions();
    let tkb: &TokenBody = ext.get().unwrap();
    let upath = format!("{}\\{}", app.store_path, tkb.username);
    if (!is_in_udir(&*payload.path, &upath)) && tkb.identity != "manager" {
        return HttpResponse::BadRequest().json(json!({"msg":"No Permission"}));
    }
    let path = Path::new(&upath);
    if !path.is_dir() {
        return HttpResponse::BadRequest().json(json!({"msg":"Not a Dir"}));
    }
    let payload = payload.into_inner();
    let files = payload.files;
    for mut file in files {
        let fname = file.file_name;
        if let None = fname {
            return HttpResponse::BadRequest().json(json!({"msg":"No File Name"}));
        }
        let tmp = file.file.as_file_mut();
        let destpath = format!("{}\\{}", payload.path.to_string(), fname.unwrap());
        let mut destination = std::fs::File::create(destpath).unwrap();
        let r = std::io::copy(tmp, &mut destination);
        if let Err(e) = r {
            return HttpResponse::BadRequest().json(json!({"msg":e.to_string()}));
        }
    }
    HttpResponse::Ok().json(json!({"msg":"Success"}))
}

#[post("/mkdir")]
pub async fn mkdir(
    req: HttpRequest,
    data: Json<CreateDirPayload>,
    app: Data<AppState>,
) -> impl Responder {
    let ext = req.extensions();
    let tkb: &TokenBody = ext.get().unwrap();
    let upath = format!("{}\\{}", app.store_path, tkb.username);
    let path = Path::new(&data.path).parent();
    if path.is_none() {
        return HttpResponse::BadRequest().json(json!({"msg":"No Permission"}));
    }
    let fp = path.unwrap();
    if !fp.starts_with(&upath) || data.path.contains("../") {
        return HttpResponse::BadRequest().json(json!({"msg":"No Permission"}));
    }
    let r = fs::create_dir(&data.path).await;
    if let Err(e) = r {
        return HttpResponse::BadRequest().json(json!({"msg":e.to_string()}));
    }
    HttpResponse::Ok().json(json!({"msg":"Success"}))
}
