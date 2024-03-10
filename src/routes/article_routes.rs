use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use serde::{ Deserialize, Serialize };

use crate::{ db, models::{self, UpdateArticle} };
use db::article_table_helper;
use models::InsertArticle;

pub fn article_scopes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/articles")
            .service(index)
            .service(latest_articles_handler)
            .service(create_article)
            .service(user_articles_handler)
            .service(update_article_status_handler)
            .service(delete_article_handler)
    );
}

#[derive(Serialize, Deserialize, Debug)]
struct Info { user_id: String }

#[post("/new")]
async fn create_article(article: web::Json<InsertArticle>) -> impl Responder {
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

#[get("/{user_id}/{type}")]
async fn user_articles_handler(path: web::Path<(String, String)>) -> impl Responder {
    let articles = article_table_helper::get_articles_by_user_id(path.0.clone(), path.1.clone()).await;
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "articles": articles
    }))
}

#[put("/update")]
async fn update_article_status_handler(data: web::Json<UpdateArticle>) -> impl Responder {
    let result = article_table_helper::update_article(data.into_inner()).await;

    match result {
        Ok(_) => {
            HttpResponse::Ok()
                .json(json!({
                    "status": "ok",
                    "message": "Article status updated successfully"
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

#[delete("/delete/{id}")]
async fn delete_article_handler(id: web::Path<String>, data: web::Query<Info>) -> impl Responder {
    let data = data.into_inner();
    let result = article_table_helper::delete_article(id.into_inner(), data.user_id).await;

    match result {
        Ok(_) => {
            HttpResponse::Ok()
                .json(json!({
                    "status": "ok",
                    "message": "Article deleted successfully"
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
