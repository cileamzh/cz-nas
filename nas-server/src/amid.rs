use actix_web::{
    Error as ActixError, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    http::header::COOKIE,
};
use futures::Future;
use futures::future::{Ready, ok};
use std::{
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

use crate::AppState;

// JWT 解析中间件
pub struct AuthMiddleware {
    app_state: Rc<AppState>,
}

impl AuthMiddleware {
    pub fn new(app_state: Rc<AppState>) -> Self {
        Self { app_state }
    }
}

// 让 `AuthMiddleware` 变成 Actix 的中间件
impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService {
            service: Rc::new(service),
            app_state: self.app_state.clone(),
        })
    }
}

// 处理请求
pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
    app_state: Rc<AppState>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let app_state = self.app_state.clone();

        Box::pin(async move {
            // 从 Cookie 获取 token
            let token = req
                .headers()
                .get(COOKIE)
                .and_then(|value| value.to_str().ok())
                .and_then(|cookie| {
                    cookie
                        .split("; ")
                        .find(|s| s.starts_with("token="))
                        .map(|s| s.trim_start_matches("token=").to_string())
                });

            if let Some(token) = token {
                // 解析 Token
                match app_state.decodetoken(&token) {
                    Ok(decoded) => {
                        // Token 验证成功，存入请求扩展数据
                        req.extensions_mut().insert(decoded.claims);
                    }
                    Err(_) => {
                        // Token 解析失败（过期或无效）
                        return Err(ErrorUnauthorized("Invalid or expired token"));
                    }
                }
            } else {
                return Err(ErrorUnauthorized("Missing token in cookie"));
            }
            service.call(req).await
        })
    }
}
