use dotenv::dotenv;
use uuid::Uuid;
use std::env;
use sqlx::{ Pool, Postgres, Result };
use sqlx::postgres::PgQueryResult;

use crate::models::{InsertUser, LoginUser, SavedUser, User};

pub async fn fetch_all_users() -> Result<Vec<User>, sqlx::Error> {
    let conn_pool = establish_connection().await;

    sqlx::query_as!(
        User,
        r#"SELECT first_name, last_name, username, email,
            about, account_status, registration_date,
            last_login_date FROM users;"#)
        .fetch_all(&conn_pool)
        .await
}

pub async fn insert_user(user: InsertUser) -> Result<PgQueryResult, String> {
    let pool = establish_connection().await;

    if is_email_taken(&pool, &user.email).await {
        return Err("User with same email already exists".to_string());
    }

    let user_id = Uuid::new_v4().hyphenated().to_string();

    let res = sqlx::query!(r#"
        INSERT INTO users (id, first_name, last_name, username, email, password,
        about) VALUES ($1, $2, $3, $4, $5, $6, $7);
        "#, user_id, user.first_name,
        user.last_name, user.username,
        user.email, hash_password(user.password), user.about)
        .execute(&pool)
        .await;
    pool.close().await;

    match res {
        Ok(res) => Ok(res),
        Err(e) => Err(e.to_string())
    }
}

async fn is_email_taken(pool: &Pool<Postgres>, email: &str) -> bool {
    sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email=$1);"
        , email)
        .fetch_one(pool)
        .await
        .expect("Error fetching record")
        .exists
        .unwrap_or(false)
}

pub async fn get_user_info_by_credentials(login_user: LoginUser) -> Option<SavedUser> {
    let pool = establish_connection().await;
    let res = sqlx::query_as!(
        SavedUser,
        r#"SELECT id, first_name, last_name, email
        FROM users WHERE email=$1;"#,
        login_user.email)
        .fetch_optional(&pool)
        .await
        .expect("Error fetching record");

    if let Some(user) = res {
        let stored_password = get_password_from_db(&user.email, &pool).await;

        if bcrypt::verify(&login_user.password, &stored_password).unwrap_or(false) {
            return Some(user)
        }
    }

    pool.close().await;
    return None;
}

async fn get_password_from_db(email: &str, pool: &Pool<Postgres>) -> String {
    let row = sqlx::query!(
        "SELECT password FROM users WHERE email=$1;", email)
        .fetch_one(pool)
        .await
        .expect("Error decoding");

    row.password
}

fn hash_password(password: String) -> String {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .expect("Unable to hash")
}

async fn establish_connection() -> Pool<Postgres> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL")
        .expect("db url not found in .env");

    Pool::connect(&db_url)
        .await
        .expect("Error connecting to db")
}
