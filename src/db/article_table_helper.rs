use dotenv::dotenv;
use uuid::Uuid;
use std::env;
use sqlx::{ Pool, Postgres };
use sqlx::postgres::PgQueryResult;

use crate::models::{
    Article, InsertArticle, ReturnArticle, UpdateArticle
};
use crate::db::user_table_helper::get_user_by_id;


pub async fn insert_article(article: InsertArticle) -> Result<PgQueryResult, String> {
    let pool = establish_connection().await;
    let article_id = Uuid::new_v4().hyphenated().to_string();

    if let Some(user) = get_user_by_id(article.user_id.clone()).await {
        if user.account_status != "active" {
            return Err("User account is not active".to_string());
        }

    } else {
        return Err("User not found".to_string());
    }

    let query = if article.status.is_none() {
        sqlx::query!(
            r#"INSERT INTO articles (id, user_id, title, content)
            VALUES ($1, $2, $3, $4)"#,
            article_id, article.user_id, article.title, article.content)
    } else {
        sqlx::query!(
            r#"INSERT INTO articles (id, user_id, title, content, status)
            VALUES ($1, $2, $3, $4, $5)"#,
            article_id, article.user_id, article.title, article.content, article.status)
    };

    let result = query.execute(&pool).await;

    pool.close().await;

    match result {
        Ok(res) => Ok(res),
        Err(e) => Err(e.to_string())
    }
}

pub async fn update_article(article: UpdateArticle) -> Result<PgQueryResult, String> {
    let pool = establish_connection().await;

    if let None = get_article_by_id(article.id.clone()).await {
        return Err("Article not found".to_string());
    }

    let mut params: Vec<String> = Vec::new();
    let mut params_index = 1;
    let mut query = String::from("UPDATE articles SET");

    if let Some(title) = article.title {
        query.push_str(format!(" title = ${}", params_index).as_str());
        params_index += 1;
        query.push_str(",");
        params.push(title);
    }

    if let Some(content) = article.content {
        query.push_str(format!(" content = ${}", params_index).as_str());
        params_index += 1;
        query.push_str(",");
        params.push(content);
    }

    if let Some(status) = article.status {
        query.push_str(format!(" status = ${}", params_index).as_str());
        params_index += 1;
        query.push_str(",");
        params.push(status);
    }

    query.pop();
    query.push_str(format!(" WHERE id = ${};", params_index).as_str());
    params.push(article.id);
    println!("{}", query);
    println!("{:?}", params);

    let mut sql = sqlx::query(&query);

    for param in params {
        sql = sql.bind(param);
    }

    let result = sql
        .execute(&pool)
        .await;

    match result {
        Ok(res) => Ok(res),
        Err(e) => Err(e.to_string())
    }
}

pub async fn get_article_by_id(id: String) -> Option<Article> {
    let pool = establish_connection().await;

    sqlx::query_as!(
        Article,
        r#"
        SELECT id, user_id, title, content, status, creation_date
        FROM articles
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(&pool)
    .await
    .ok()
}

pub async fn delete_article(id: String, user_id: String) -> Result<PgQueryResult, String> {
    let pool = establish_connection().await;
    let article = get_article_by_id(id.clone()).await;

    if article.is_none() {
        return Err("Article not found".to_string());
    }

    let article = article.unwrap();

    if article.user_id != user_id {
        return Err("Unauthorized".to_string());
    }

    if article.status == "published" {
        return Err("Cannot delete published article".to_string());
    }

    let result = sqlx::query!(
        r#"DELETE FROM articles WHERE id = $1"#,
        id
    )
    .execute(&pool)
    .await;
    println!("{:?}", result);

    pool.close().await;

    match result {
        Ok(res) => Ok(res),
        Err(e) => Err(e.to_string())
    }
}

pub async fn get_articles_by_user_id(user_id: String, type_: String) -> Vec<ReturnArticle> {
    let pool = establish_connection().await;

    if type_ == "all" {
        return sqlx::query_as!(
            ReturnArticle,
            r#"
            SELECT articles.id, username as author, title,
            users.id as user_id, content, status, creation_date
            FROM articles
            INNER JOIN users ON articles.user_id = users.id
            WHERE user_id = $1
            "#, user_id)
            .fetch_all(&pool)
            .await
            .expect("Error retrieving records from table");
    } else {
        return sqlx::query_as!(
            ReturnArticle,
            r#"
            SELECT articles.id, username as author, title,
            users.id as user_id, content, status, creation_date
            FROM articles
            INNER JOIN users ON articles.user_id = users.id
            WHERE user_id = $1 AND status = $2
            "#,
            user_id, type_
            )
            .fetch_all(&pool)
            .await
            .expect("Error retrieving records from table");
    }
}

pub async fn get_latest_articles() -> Vec<ReturnArticle> {
    let pool = establish_connection().await;

    let result = sqlx::query_as!(
        ReturnArticle,
        r#"
        SELECT articles.id, username as author, title,
        users.id as user_id, content, status, creation_date
        FROM articles
        INNER JOIN users ON articles.user_id = users.id
        WHERE status = 'published'
        ORDER BY creation_date DESC
        LIMIT 10
        "#)
        .fetch_all(&pool)
        .await
        .expect("Error retrieving records from table");

    pool.close().await;

    result
}

#[allow(dead_code)]
pub async fn get_article(article_id: String) -> Option<ReturnArticle> {
    let pool = establish_connection().await;

    let result = sqlx::query_as!(
        ReturnArticle,
        r#"SELECT articles.id, username as author, title, content,
            users.id as user_id, status, creation_date
            FROM articles
            INNER JOIN users ON articles.user_id = users.id
            WHERE articles.id = $1"#,
        article_id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(res) => Some(res),
        Err(_) => None
    }
}

pub async fn get_all_articles() -> Vec<ReturnArticle> {
    let pool = establish_connection().await;

    let result = sqlx::query_as!(
        ReturnArticle,
        r#"
        SELECT articles.id, username as author, title, content,
        users.id as user_id, status, creation_date
        FROM articles
        INNER JOIN users ON articles.user_id = users.id
        WHERE status = 'published'
        ORDER BY creation_date;
        "#)
        .fetch_all(&pool)
        .await
        .expect("Error retrieving records from table");

    pool.close().await;

    result
}

async fn establish_connection() -> Pool<Postgres> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    sqlx::postgres::PgPoolOptions::new()
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres")
}
