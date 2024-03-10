use sqlx::FromRow;
use serde::{ Serialize, Deserialize };
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub about: String,
    pub account_status: String,
    pub registration_date: NaiveDateTime,
    pub last_login_date: NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct InsertUser {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct SavedUser {
    pub id: String,
    pub username: String,
    pub email: String,
    pub last_login_date: NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub about: Option<String>
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Article {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub status: String,
    pub creation_date: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct InsertArticle {
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateArticleStatus {
    pub id: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateArticle {
    pub id: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ReturnArticle {
    pub id: String,
    pub author: String,
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub status: String,
    pub creation_date: NaiveDateTime
}
