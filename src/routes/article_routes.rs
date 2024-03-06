use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;

use crate::{ db, models::InsertArticle };
use db::article_table_helper;

pub fn article_scopes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/articles")
            .service(index)
            .service(latest_articles_handler)
            .service(create_article)
    );
}

#[post("/new")]
async fn create_article(_request: HttpRequest, article: web::Json<InsertArticle>) -> impl Responder {
    let article = article.into_inner();
    let result = article_table_helper::insert_article(article).await;

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

#[get("/all")]
async fn index() -> impl Responder {
    let articles = article_table_helper::get_all_articles().await;
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "articles": articles
    }))
}

#[get("/latest")]
async fn latest_articles_handler() -> impl Responder {
    let articles = article_table_helper::get_latest_articles().await;
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "articles": articles
    }))
}
