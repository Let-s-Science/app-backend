use chrono::offset::Utc;
use derivative::Derivative;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Object, Clone, Derivative, Serialize, Deserialize)]
#[derivative(Debug, Default)]
pub struct User {
    #[oai(read_only)]
    pub id: Uuid,
    #[oai(validator(max_length = 64))]
    pub name: String,
    pub email: Option<String>,
    #[derivative(Debug = "ignore")]
    #[oai(skip)]
    #[serde(skip)]
    pub hash: Option<String>,
    #[derivative(Default(value = "true"))]
    pub is_guest: bool,
    pub avatar_seed: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
    pub score: i32,
}

type Result<T> = sqlx::Result<T>;

// Inserts a new user into the database.
// Returns Ok(None) if a user with the specified E-Mail adress already exists
#[tracing::instrument(skip(pool))]
pub async fn insert_user(pool: &PgPool, user: &User) -> Result<Option<Uuid>> {
    match sqlx::query_scalar!(
        r#"insert into "user" (id, name, email, avatar_seed, hash, is_guest) values ($1, $2, $3, $4, $5, $6) returning id"#,
        Uuid::new_v4(),
        user.name,
        user.email,
        user.avatar_seed,
        user.hash,
        user.is_guest
    ).fetch_optional(pool)
    .await {
        Ok(u) => Ok(u),
        Err(sqlx::Error::Database(e)) => {
            match e.constraint() {
                Some("user_email_key") | Some("user_pkey") => Ok(None),
                _ => Err(sqlx::Error::Database(e))
                            }
        },
        Err(e) => Err(e)
    }
}

#[tracing::instrument(skip(pool))]
pub async fn get_user(pool: &PgPool, id: Uuid) -> Result<Option<User>> {
    sqlx::query_as!(User, r#"select * from "user" where "user".id = $1 "#, id)
        .fetch_optional(pool)
        .await
}

#[tracing::instrument(skip(pool))]
pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>> {
    sqlx::query_as!(
        User,
        r#"select * from "user" where "user".email = $1"#,
        email
    )
    .fetch_optional(pool)
    .await
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct UserPatch {
    name: Option<String>,
    email: Option<String>,
    avatar_seed: Option<String>,
    hash: Option<String>,
    is_guest: Option<bool>,
    score: Option<i32>,
}

#[tracing::instrument(skip(pool))]
pub async fn update_user(pool: &PgPool, id: Uuid, patch: &UserPatch) -> Result<Option<User>> {
    sqlx::query_as!(
        User,
        r#"
            update "user"
            set email = coalesce($1, "user".email),
                name = coalesce($2, "user".name),
                avatar_seed = coalesce($3, "user".avatar_seed),
                hash = coalesce($4, "user".hash),
                is_guest = coalesce($5, "user".is_guest),
                score = coalesce($6, "user".score)
            where id = $7
            returning *
        "#,
        patch.email,
        patch.name,
        patch.avatar_seed,
        patch.hash,
        patch.is_guest,
        patch.score,
        id
    )
    .fetch_optional(pool)
    .await
}

#[tracing::instrument(skip(pool))]
pub async fn increase_score(pool: &PgPool, id: Uuid, score: i32) -> Result<Option<()>> {
    let Some(user) = get_user(pool, id).await? else {
        return Ok(None);
    };
    let patch = UserPatch {
        score: Some(user.score + score),
        ..UserPatch::default()
    };

    Ok(update_user(pool, id, &patch).await?.map(|_| ()))
}

#[cfg(test)]
mod tests {
    use super::User;
    use sqlx::PgPool;

    fn verified_user() -> User {
        User {
            email: Some(String::from("test@example.com")),
            hash: Some(String::new()),
            is_guest: false,
            ..User::default()
        }
    }

    #[sqlx::test]
    async fn insert_user(pool: PgPool) -> sqlx::Result<()> {
        let user = super::insert_user(&pool, &verified_user()).await?;
        assert!(user.is_some());

        let user = super::insert_user(&pool, &verified_user()).await?;
        assert!(user.is_none());
        Ok(())
    }

    #[sqlx::test]
    async fn get_nonexistent_user(pool: PgPool) -> sqlx::Result<()> {
        let user = super::get_user(&pool, super::Uuid::new_v4()).await?;
        assert!(user.is_none());
        Ok(())
    }

    #[sqlx::test]
    async fn get_user_by_email(pool: PgPool) -> sqlx::Result<()> {
        let base_user = User {
            is_guest: false,
            email: Some("test@example.com".to_owned()),
            hash: Some(String::new()),
            ..User::default()
        };
        let id = super::insert_user(&pool, &base_user).await?;

        let user = super::get_user_by_email(&pool, "test@example.com").await?;
        assert!(user.is_some());
        assert_ne!(base_user.id, user.as_ref().unwrap().id);
        assert_eq!(id, user.map(|user| user.id));
        Ok(())
    }

    #[sqlx::test]
    async fn update_user(pool: PgPool) -> sqlx::Result<()> {
        let id = super::insert_user(&pool, &User::default())
            .await?
            .expect("Retrieved None id");
        let updated = super::update_user(
            &pool,
            id,
            &super::UserPatch {
                name: Some("updated".to_owned()),
                email: Some("updated".to_owned()),
                hash: Some("updated".to_owned()),
                avatar_seed: Some("updated".to_owned()),
                is_guest: Some(false),
                score: Some(133),
            },
        )
        .await?
        .expect("unable to update user");
        assert_eq!(updated.name, "updated");
        assert_eq!(updated.email.unwrap(), "updated");
        assert_eq!(updated.hash.unwrap(), "updated");
        assert_eq!(updated.avatar_seed, "updated");
        assert!(!updated.is_guest);
        assert_eq!(updated.score, 133);
        Ok(())
    }

    #[sqlx::test]
    async fn no_password_for_guest(pool: PgPool) -> sqlx::Result<()> {
        let id = super::insert_user(
            &pool,
            &User {
                hash: Some(String::new()),
                ..User::default()
            },
        )
        .await;
        assert!(id.is_err());
        Ok(())
    }

    #[sqlx::test]
    async fn default_score(pool: PgPool) -> sqlx::Result<()> {
        let user_id = super::insert_user(&pool, &User::default()).await?.unwrap();
        let user = super::get_user(&pool, user_id).await?.unwrap();
        assert_eq!(user.score, 0);
        Ok(())
    }
}
