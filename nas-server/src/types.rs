use actix_multipart::form::{tempfile::TempFile, text::Text};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
// All config
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub(crate) server: ServerConfig,
    pub(crate) jwt: JwtConfig,
}
// ServerConfig Struct
#[derive(Deserialize, Serialize)]
pub struct ServerConfig {
    pub(crate) host: String,
    pub(crate) store_path: String,
}

// Jwt Config Struct
#[derive(Deserialize, Serialize)]
pub struct JwtConfig {
    pub(crate) secret: String,
}

//Token Serialize Struct
#[derive(Deserialize, Serialize)]
pub struct TokenBody {
    pub(crate) username: String,
    pub(crate) exp: usize,
    pub(crate) identity: String,
    pub(crate) disksize: i32,
}

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) identity: String,
    pub(crate) avator: Option<String>,
    pub(crate) disksize: i32,
    pub(crate) bg: Option<String>,
}

// Registry Req Struct
#[derive(Deserialize, Serialize)]
pub struct RegistryReq {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) disksize: i32,
}

// LogIn Req Struct
#[derive(Deserialize, Serialize)]
pub struct LoginReq {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UnameReq {
    pub(crate) username: String,
}

#[derive(Deserialize)]
pub struct RenamePayload {
    pub(crate) old_path: String,
    pub(crate) new_name: String,
}

#[derive(Deserialize)]
pub struct DeletePayload {
    pub(crate) path: String,
}

#[derive(Deserialize)]
pub struct CopyPayload {
    pub(crate) from: String,
    pub(crate) to: String,
}

#[derive(Deserialize)]
pub struct SetAvatorPayload {
    pub(crate) avator_path: String,
}

#[derive(Deserialize)]
pub struct SetBgPayload {
    pub(crate) path: String,
}

#[derive(actix_multipart::form::MultipartForm)]
pub struct UploadFileForm {
    pub(crate) path: Text<String>,
    pub(crate) files: Vec<TempFile>,
}

#[derive(Deserialize)]
pub struct MoveFilePayload {
    pub(crate) old_path: String,
    pub(crate) new_path: String,
}

#[derive(Deserialize)]
pub struct CreateDirPayload {
    pub(crate) path: String,
}
