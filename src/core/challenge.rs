use crate::entities::challenge::{Challenge, ChallengeType};
use sqlx::{PgPool, Result};
use uuid::Uuid;

use super::quiz::insert_translation;

#[tracing::instrument(skip(pool))]
pub async fn insert_challenge(pool: &PgPool, challenge: &Challenge) -> Result<Uuid> {
    let description_id = insert_translation(pool, &challenge.description, None).await?;
    sqlx::query_scalar!(
        r#"
        insert into "challenge" (type, goal, description) values ($1, $2, $3)
        returning id"#,
        challenge.r#type as _,
        challenge.goal,
        description_id
    )
    .fetch_one(pool)
    .await
}

#[tracing::instrument]
pub async fn get_challenge(pool: &PgPool, id: Uuid) -> Result<Option<Challenge>> {
    sqlx::query_as!(
        Challenge,
        r#"
            select
                challenge.id id,
                type as "type: ChallengeType",
                goal,
                content description
            from challenge challenge
            inner join translation
            on challenge.description = translation.id
            where challenge.id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}
