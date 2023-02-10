use derive_more::{Display, Error};
use error_stack::{Report, Result};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use super::models::User;

#[derive(Debug, Display, Error)]
pub struct UserRepositoryError;

pub async fn exists(email: String, executor: &PgPool) -> Result<bool, UserRepositoryError> {
    match sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
        .bind(email)
        .fetch_one(executor)
        .await
    {
        Ok(row) => Ok(row.get(0)),
        Err(e) => Err(Report::new(UserRepositoryError).attach(e)),
    }
}

pub async fn create(
    name: String,
    email: String,
    password: String,
    role: Option<String>,
    executor: &PgPool,
) -> Result<User, UserRepositoryError> {
    let role = role.unwrap_or("user".into());

    let query_result = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email, password, role) values ($1, $2, $3, $4) RETURNING *",
        name,
        email,
        password,
        role,
    )
    .fetch_one(executor)
    .await;

    match query_result {
        Ok(user) => Ok(user),
        Err(e) => Err(Report::new(UserRepositoryError).attach(e)),
    }
}

pub async fn get_user_by_email(email: String, executor: &PgPool) -> Result<Option<User>, UserRepositoryError> {
    match sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
        .fetch_optional(executor)
        .await
    {
        Ok(user) => Ok(user),
        Err(e) => Err(Report::new(UserRepositoryError).attach(e)),
    }
}

pub async fn get_user_by_id(user_id: &Uuid, executor: &PgPool) -> Result<User, UserRepositoryError> {
    match sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(executor)
        .await
    {
        Ok(user) => Ok(user),
        Err(e) => Err(Report::new(UserRepositoryError).attach(e)),
    }
}
