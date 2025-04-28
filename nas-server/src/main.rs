use std::rc::Rc;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    App, HttpServer,
    web::{Data, scope},
};
use amid::AuthMiddleware;
use json::read_json;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use sqlx::SqlitePool;
use svces::{
    delete_file, login, mkdir, open, registry, rename_file, upload_avator, upload_bg, upload_file,
    user_dir, userinfo,
};
use types::{Config, TokenBody};
pub mod amid;
pub mod db;
pub mod gnid;
pub mod json;
pub mod svces;
pub mod time;
pub mod types;
pub mod ws;
#[actix_web::main]
async fn main() {
    // 加载服务器配置文件
    let config: Config = read_json("config.json").expect("failed to serialize config.json");
    // 初始化数据库连接池
    let pool = SqlitePool::connect("sqlite:server.db").await.unwrap();
    let appstate = AppState {
        tokensecret: config.jwt.secret.clone(),
        store_path: config.server.store_path.clone(),
    };
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(appstate.clone()))
            .wrap(Cors::permissive())
            .service(
                // user请求,无需token
                scope("/unauth")
                    // login 请求
                    .service(login), // 注册用户
            )
            .service(
                // api请求,token鉴定权限
                scope("/auth")
                    .wrap(AuthMiddleware::new(Rc::new(appstate.clone())))
                    .service(registry)
                    // 上传用户头像
                    .service(upload_avator)
                    .service(user_dir)
                    .service(open)
                    .service(userinfo)
                    .service(rename_file)
                    .service(delete_file)
                    .service(upload_file)
                    .service(mkdir)
                    .service(upload_bg),
            )
            .service(
                Files::new("/", "C:\\Users\\cilea\\Desktop\\workspace\\nas-web\\dist")
                    .index_file("index.html"),
            )
    })
    .bind(config.server.host.clone())
    .unwrap();
    println!("Server Running in http://{}", config.server.host);
    server.run().await.unwrap();
}

// 服务器共享资源
#[derive(Clone)]
pub struct AppState {
    // token 的密钥
    tokensecret: String,
    pub(crate) store_path: String,
}

// 为appstate加入token支持
impl AppState {
    // 解析token
    pub fn decodetoken(
        &self,
        token: &str,
    ) -> Result<jsonwebtoken::TokenData<TokenBody>, jsonwebtoken::errors::Error> {
        let r = decode(
            token,
            &DecodingKey::from_secret(self.tokensecret.as_ref()),
            &Validation::default(),
        );
        r
    }
    // 加密token
    pub fn encodetoken(&self, tkb: TokenBody) -> String {
        let r = encode(
            &Header::default(),
            &tkb,
            &EncodingKey::from_secret(&self.tokensecret.as_ref()),
        )
        .unwrap();
        r
    }
}
