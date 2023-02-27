use crate::entities::challenge::{Challenge, ChallengeType, UserChallenge};
use sqlx::{PgPool, Result};
use uuid::Uuid;

use super::quiz::insert_translation;

#[tracing::instrument(skip(pool))]
pub async fn insert_challenge(pool: &PgPool, challenge: &Challenge) -> Result<Uuid> {
    let description_id = insert_translation(pool, &challenge.description, None).await?;
    sqlx::query_scalar!(
        r#"
        insert into "challenge" (type, goal, description, title, category) values ($1, $2, $3, $4, $5)
        returning id"#,
        challenge.r#type as _,
        challenge.goal,
        description_id,
        challenge.title,
        challenge.category
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
                title,
                category,
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

#[tracing::instrument]
pub async fn add_progress(
    pool: &PgPool,
    user_id: Uuid,
    challenge_id: Uuid,
    progress: i32,
) -> Result<Option<UserChallenge>> {
    sqlx::query_as!(
        UserChallenge,
        r#"
            insert into user_challenge (user_id, challenge_id, progress)
            values ($1, $2, $3)
            on conflict on constraint one_user_per_challenge
            do update set progress = user_challenge.progress + EXCLUDED.progress
            returning *
        "#,
        user_id,
        challenge_id,
        progress
    )
    .fetch_optional(pool)
    .await
}

#[tracing::instrument]
pub async fn get_user_challenges(
    pool: &PgPool,
    user_id: Uuid,
    challenge_id: Option<Uuid>,
) -> Result<Vec<UserChallenge>> {
    if let Some(challenge_id) = challenge_id {
        sqlx::query_as!(
            UserChallenge,
            r#"
                select * from "user_challenge"
                where challenge_id = $1 and user_id = $2
            "#,
            challenge_id,
            user_id
        )
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as!(
            UserChallenge,
            r#"
                select * from "user_challenge"
                where user_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await
    }
}

#[tracing::instrument]
pub async fn get_challenges(pool: &PgPool) -> Result<Vec<Challenge>> {
    Ok(sqlx::query!(
        r#"
        select 
            challenge.id id,
            type as "type: ChallengeType",
            goal,
            title,
            category,
            content
        from "challenge"
        inner join translation
        on challenge.description = translation.id
   "#
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|record| Challenge {
        id: record.id,
        r#type: record.r#type,
        title: record.title,
        category: record.category,
        goal: record.goal,
        description: record.content,
    })
    .collect())
}

#[tracing::instrument]
pub async fn delete_progress(
    pool: &PgPool,
    user_id: Uuid,
    challenge_id: Uuid,
) -> Result<Option<UserChallenge>> {
    sqlx::query_as!(
        UserChallenge,
        r#"
            delete from user_challenge
            where user_id = $1 and challenge_id = $2
            returning *
        "#,
        user_id,
        challenge_id
    )
    .fetch_optional(pool)
    .await
}

#[cfg(test)]
mod tests {

    use super::*;

    #[sqlx::test]
    async fn insert_challenge(pool: PgPool) -> sqlx::Result<()> {
        let res = super::insert_challenge(&pool, &Challenge::default()).await;
        assert!(res.is_err());
        Ok(())
    }
}
