use futures::future;
use sqlx::{PgPool, Result};
use uuid::Uuid;

use crate::entities::quiz::{DBQuiz, DBQuizQuestion};

#[tracing::instrument(skip(pool))]
pub async fn get_quiz(pool: &PgPool, id: Uuid) -> Result<Option<DBQuiz>> {
    let Some(row) = sqlx::query!(
        r#"
            select
                quiz.id id,
                created_at,
                created_by,
                content title
            from quiz quiz
            inner join translation
            on quiz.title = translation.id
            where quiz.id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await? else  {
        println!("Heeeeeeeeey");
        return Ok(None);
    };

    let questions = get_questions_by_quiz_id(pool, row.id).await?;
    Ok(Some(DBQuiz {
        id: row.id,
        title: row.title,
        created_at: row.created_at,
        created_by: row.created_by,
        questions,
    }))
}

pub async fn get_questions_by_quiz_id(pool: &PgPool, quiz_id: Uuid) -> Result<Vec<DBQuizQuestion>> {
    let row = sqlx::query!(
        r#"
            select 
                question.id,
                question.quiz_id,
                content question,
                data
            from question
            inner join translation
            on question.question = translation.id
            where question.quiz_id = $1
        "#,
        quiz_id
    )
    .fetch_all(pool)
    .await?;
    Ok(row
        .into_iter()
        .map(|record| DBQuizQuestion {
            id: record.id,
            quiz_id: record.quiz_id,
            question: record.question,
            data: serde_json::from_value(record.data).expect("Unable to parse json"),
        })
        .collect())
}

#[tracing::instrument(skip(pool))]
pub async fn insert_quiz(pool: &PgPool, quiz: &DBQuiz) -> Result<Uuid> {
    let conn = pool.begin().await?;

    let title_id = sqlx::query_scalar!(
        r#"insert into "translation" (language_code, content) values ($1, $2) returning id"#,
        "en-GB",
        quiz.title
    )
    .fetch_one(pool)
    .await?;

    let quiz_id = sqlx::query_scalar!(
        r#"insert into "quiz" (title, created_by) values ($1, $2) returning id"#,
        title_id,
        quiz.created_by
    )
    .fetch_one(pool)
    .await?;

    let fut_vec = quiz.questions.iter().map(|question| {
        let question = DBQuizQuestion {
            quiz_id,
            ..question.clone()
        };
        insert_question(pool, question)
    });
    future::join_all(fut_vec).await;

    conn.commit().await?;
    Ok(quiz_id)
}

#[tracing::instrument(skip(pool))]
pub async fn insert_question(pool: &PgPool, question: DBQuizQuestion) -> Result<Uuid> {
    let translation_id = insert_translation(pool, &question.question, None).await?;
    sqlx::query_scalar!(
        r#"insert into "question" (quiz_id, question, data) values ($1, $2, $3) returning id"#,
        question.quiz_id,
        translation_id,
        question.data as _
    )
    .fetch_one(pool)
    .await
}

#[tracing::instrument(skip(pool))]
pub async fn insert_translation(
    pool: &PgPool,
    content: &str,
    language_code: Option<String>,
) -> Result<Uuid> {
    let language_code = language_code.unwrap_or_else(|| "en-GB".to_owned());
    sqlx::query_scalar!(
        r#"insert into "translation" (language_code, content) values ($1, $2) returning id"#,
        language_code,
        content
    )
    .fetch_one(pool)
    .await
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::{
        core::{self, user::User},
        entities::quiz::DBQuiz,
    };

    #[sqlx::test]
    async fn require_created_by(pool: PgPool) -> sqlx::Result<()> {
        let res = super::insert_quiz(&pool, &DBQuiz::default()).await;
        assert!(res.is_err());
        Ok(())
    }

    #[sqlx::test]
    async fn insert_quiz(pool: PgPool) -> sqlx::Result<()> {
        let user_id = core::user::insert_user(&pool, &User::default())
            .await?
            .unwrap();
        let mut quiz = DBQuiz::default();
        quiz.created_by = user_id;
        super::insert_quiz(&pool, &quiz).await?;
        Ok(())
    }

    #[sqlx::test]
    async fn get_quiz(pool: PgPool) -> sqlx::Result<()> {
        let user_id = core::user::insert_user(&pool, &User::default())
            .await?
            .unwrap();
        let mut quiz = DBQuiz::default();
        quiz.created_by = user_id;
        let quiz_id = super::insert_quiz(&pool, &quiz).await?;

        let db_quiz = super::get_quiz(&pool, quiz_id)
            .await?
            .expect("Unable to retrive Quiz from DB");
        assert_eq!(db_quiz.title, quiz.title);
        assert_eq!(db_quiz.created_by, user_id);
        Ok(())
    }
}
