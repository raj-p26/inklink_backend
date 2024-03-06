use actix_web::body::BoxBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use actix_web::{http, HttpMessage};
use futures_util::future::{Ready, ready, LocalBoxFuture};
use futures_util::FutureExt;
use serde_json::json;
use std::rc::Rc;
use std::task::{ Context, Poll };

use crate::jwt;
use crate::db::user_table_helper;
use crate::models::User;

pub struct AuthMiddleware;

impl<S> actix_web::dev::Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<
        ServiceRequest,
        Response = ServiceResponse<BoxBody>,
        Error = actix_web::Error,
    > + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = actix_web::Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service: Rc::new(service) }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>
}

impl<S> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<
        ServiceRequest,
        Response = ServiceResponse<actix_web::body::BoxBody>,
        Error = actix_web::Error
    > + 'static
{
    type Response = ServiceResponse<actix_web::body::BoxBody>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, actix_web::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            let err_obj = json!({
                "status": "failed",
                "message": "token not found"
            });
            return Box::pin(ready(Err(ErrorUnauthorized(err_obj.to_string()))));
        }

        let user_id = match jwt::decode_token(token.unwrap()) {
            Ok(user_id) => user_id,
            Err(_) => {
                let err_obj = json!({
                    "status": "failed",
                    "message": "invalid token"
                });
                return Box::pin(ready(Err(ErrorUnauthorized(err_obj.to_string()))));
            }
        };

        let srv = Rc::clone(&self.service);

        async move {
            let user = user_table_helper::get_user_by_id(user_id).await;
            if user.is_none() {
                let err_obj = json!({
                    "status": "failed",
                    "message": "User not found"
                });
                return Err(ErrorInternalServerError(err_obj.to_string()));
            }

            req.extensions_mut().insert::<User>(user.unwrap());
            srv.call(req).await
        }.boxed_local()
    }
}
