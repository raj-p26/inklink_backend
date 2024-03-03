use actix_web::{ get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder };
use serde_json::json;

mod models;
mod db;

use models::{ InsertUser, LoginUser };

#[get("/")]
async fn root_handler() -> impl Responder {
    if let Ok(users) = db::fetch_all_users().await {
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

#[post("/users/signup")]
async fn new_user_handler(data: web::Json<InsertUser>) -> impl Responder {
    match db::insert_user(data.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json(json!({ "status": "ok" })),
        Err(e) => {
            HttpResponse::Conflict()
                .json(json!({
                    "status": "failed",
                    "message": e.to_string()
                }))
        }
    }
}

#[post("/users/login")]
async fn login_user_handler(data: web::Json<LoginUser>) -> impl Responder {
    match db::get_user_info_by_credentials(data.into_inner()).await {
        Some(user) => {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    println!("Server started Successfully");

    HttpServer::new(move || {
        App::new()
            .service(root_handler)
            .service(new_user_handler)
            .service(login_user_handler)
            .wrap(Logger::default())
    })
    .bind("192.168.102.215:4000")?
    .run()
    .await?;

    Ok(())
}
