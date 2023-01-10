use chrono::offset::Utc;
use derivative::Derivative;
use poem_openapi::Object;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Object, Derivative)]
#[derivative(Debug)]
pub struct User {
    #[oai(read_only)]
    id: Uuid,
    #[oai(validator(max_length = 64))]
    name: String,
    email: String,
    #[derivative(Debug = "ignore")]
    #[oai(skip)]
    hash: String,
    is_guest: bool,
    avatar_seed: String,
    created_at: chrono::DateTime<Utc>,
    updated_at: Option<chrono::DateTime<Utc>>,
}

pub async fn get_user(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(User, r#"select * from "user" where "user".id = $1 "#, id)
        .fetch_optional(pool)
        .await
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"select * from "user" where "user".email = $1"#,
        email
    )
    .fetch_optional(pool)
    .await
}

pub struct UserPatch {
    email: Option<String>,
    avatar_seed: Option<String>,
    hash: Option<String>,
}

pub async fn update_user(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    Ok(None)
}
