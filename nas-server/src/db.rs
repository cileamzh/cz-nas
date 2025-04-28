use sqlx::{Executor, SqlitePool, query, query_as};

use crate::types::User;

pub async fn add_user(
    user: User,
    pool: &SqlitePool,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    pool.execute(
        query("INSERT INTO users(username,password,avator) values(?,?,?)")
            .bind(user.username)
            .bind(user.password)
            .bind(user.avator),
    )
    .await
}

pub async fn delete_user(
    username: &str,
    pool: &SqlitePool,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    pool.execute(query("DELETE FROM users WHERE username=?").bind(username))
        .await
}

pub async fn query_user(username: &str, pool: &SqlitePool) -> Result<User, sqlx::Error> {
    query_as("SELECT * FROM users WHERE username=?")
        .bind(username)
        .fetch_one(pool)
        .await
}

pub async fn username_exist(username: &str, pool: &SqlitePool) -> Result<(bool,), sqlx::Error> {
    query_as("SELECT EXISTS(SELECT 1 FROM users WHERE username=?)")
        .bind(username)
        .fetch_one(pool)
        .await
}

pub async fn update_username(
    lusername: &str,
    nusername: &str,
    pool: &SqlitePool,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    pool.execute(
        query("UPDATE users SET username=? WHERE username=?")
            .bind(nusername)
            .bind(lusername),
    )
    .await
}

pub async fn upadta_avator(
    avator: &str,
    username: &str,
    pool: &SqlitePool,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    pool.execute(
        query("UPDATE users SET avator=? WHERE username=?")
            .bind(avator)
            .bind(username),
    )
    .await
}

pub async fn update_bg(
    bg: &str,
    username: &str,
    pool: &SqlitePool,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    pool.execute(
        query("UPDATE users SET bg=? WHERE username=?")
            .bind(bg)
            .bind(username),
    )
    .await
}

pub async fn update_password(
    password: &str,
    username: &str,
    pool: &SqlitePool,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    pool.execute(
        query("UPDATE users SET password=? WHERE username=?")
            .bind(password)
            .bind(username),
    )
    .await
}
