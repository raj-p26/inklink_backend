#![allow(unused)]
use actix_web::{get, post, put, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde_json::json;

use crate::{ db, middleware as mw, models::{InsertArticle, User} };
use db::article_table_helper;

pub fn article_scopes(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(latest_articles_handler)
        .service(
        web::scope("/articles")
            .service(create_article)
            .wrap(mw::AuthMiddleware)
    );
}

#[post("/new")]
async fn create_article(request: HttpRequest, article: web::Json<InsertArticle>) -> impl Responder {
    let user_id = request
        .extensions()
        .get::<User>()
        .unwrap()
        .id
        .to_string();

    let result = db::article_table_helper::insert_article(article.into_inner(), user_id).await;

    match result {
        Ok(_) => {
            HttpResponse::Ok()
                .json(json!({
                    "status": "ok",
                    "message": "Article created successfully"
                }))
        },
        Err(e) => {
            println!("{:?}", &e);
            HttpResponse::InternalServerError()
                .json(json!({
                    "status": "failed",
                    "message": e.to_string()
                }))
        }
    }
}

#[get("/articles/all")]
async fn index() -> impl Responder {
    let articles = article_table_helper::get_all_articles().await;
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "articles": articles
    }))
}

#[get("/articles/latest")]
async fn latest_articles_handler() -> impl Responder {
    let articles = article_table_helper::get_latest_articles().await;
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "articles": articles
    }))
}
