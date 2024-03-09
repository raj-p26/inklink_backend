use actix_web::{
    get, middleware::Logger,
    App, HttpResponse, HttpServer,
    Responder
};
use serde_json::json;
use std::env;

mod models;
mod routes;
mod db;

use crate::routes::{ user_routes, article_routes };

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix_web=info");
    }

    env_logger::init();
    println!("Server started Successfully");

    HttpServer::new(move || {
        App::new()
            .configure(user_routes::user_scopes)
            .configure(article_routes::article_scopes)
            .service(index)
            .wrap(Logger::default())
    })
    .bind("192.168.185.216:4000")?
        .run()
        .await?;

    Ok(())
}
