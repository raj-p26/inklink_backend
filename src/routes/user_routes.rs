use actix_web::{web, get, Responder, HttpResponse, post, put};
use serde_json::json;

use crate::{ db, models };
use models::{ LoginUser, InsertUser, UpdateUser };
use db::user_table_helper;

pub fn user_scopes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(root_handler)
            .service(new_user_handler)
            .service(login_user_handler)
            .service(update_user_handler)
            .service(get_user_by_id_handler)
            .service(get_user_latest_articles)
    );
}

#[get("/")]
async fn root_handler() -> impl Responder {
    if let Ok(users) = user_table_helper::fetch_all_users().await {
        let response = json!({
            "status": "ok",
            "length": users.len(),
            "users": users
        });

        return HttpResponse::Ok().json(response);
    }

    return HttpResponse::InternalServerError()
        .json(json!({
            "status": "failed",
            "message": "something went wrong"
        }));
}

#[post("/signup")]
async fn new_user_handler(data: web::Json<InsertUser>) -> impl Responder {
    match user_table_helper::insert_user(data.into_inner()).await {
        Ok(user) => {
            HttpResponse::Ok()
                .json(json!({
                    "status": "ok",
                    "user": user,
                }))
        },
        Err(e) => {
            println!("{:?}", &e);
            HttpResponse::Conflict()
                .json(json!({
                    "status": "failed",
                    "message": e.to_string()
                }))
        }
    }
}

#[put("/update")]
async fn update_user_handler(data: web::Json<UpdateUser>) -> impl Responder {
    match user_table_helper::update_user(data.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "ok",
            "message": "User updated successfully"
        })),
        Err(e) => {
            HttpResponse::Conflict()
                .json(json!({
                    "status": "failed",
                    "message": e.to_string()
                }))
        }
    }
}

#[post("/login")]
async fn login_user_handler(data: web::Json<LoginUser>) -> impl Responder {
    match user_table_helper::get_user_info_by_credentials(data.into_inner()).await {
        Some(user) => {
            user_table_helper::update_last_login_date(&user.email)
                .await
                .unwrap();
            HttpResponse::Ok()
                .json(json!({
                    "status": "ok",
                    "user": user
                }))
        },
        None => {
            HttpResponse::BadRequest()
                .json(json!({
                    "status": "failed",
                    "message": "Invalid Credentials"
                }))
        }
    }
}

#[get("/{user_id}")]
async fn get_user_by_id_handler(path: web::Path<String>) -> impl Responder {
    let user = user_table_helper::get_user_by_id(path.into_inner()).await;
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "user": user
    }))
}

#[get("/{user_id}/latest")]
async fn get_user_latest_articles(path: web::Path<String>) -> impl Responder {
    let articles = user_table_helper::get_latest_articles_by_user_id(path.into_inner()).await;
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "articles": articles
    }))
}
