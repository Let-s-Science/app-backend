use crate::{
    core,
    entities::quiz::{APIQuiz, DBQuiz},
    security::JWTAuthorization,
};

use super::ApiTags;
use poem::web::{Data, Path, Query};
use poem_openapi::{payload::Json, ApiResponse, OpenApi};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

pub struct QuizAPI;

#[OpenApi]
impl QuizAPI {
    #[oai(path = "/quiz", method = "post", tag = "ApiTags::Quiz")]
    #[tracing::instrument(skip(self, pool, auth))]
    async fn create_quiz(
        &self,
        pool: Data<&PgPool>,
        req: Json<APIQuiz>,
        auth: JWTAuthorization,
    ) -> CreateQuizResponse {
        let mut db_quiz: DBQuiz = req.0.into();
        db_quiz.created_by = auth.0.id;
        let id = match core::quiz::insert_quiz(&pool, &db_quiz).await {
            Ok(id) => id,
            Err(e) => {
                error!("{:?}", e);
                return CreateQuizResponse::Internal;
            }
        };
        CreateQuizResponse::Ok(Json(id))
    }

    #[oai(path = "/quiz/:id", method = "get", tag = "ApiTags::Quiz")]
    #[tracing::instrument(skip(self, pool, auth))]
    async fn get_quiz(
        &self,
        pool: Data<&PgPool>,
        quiz_id: Path<Uuid>,
        locale_query: Query<LocaleQuery>,
        auth: JWTAuthorization,
    ) -> GetQuizResponse {
        let quiz = match core::quiz::get_quiz(&pool, quiz_id.0).await {
            Ok(Some(q)) => q,
            Ok(None) => return GetQuizResponse::NotFound,
            Err(e) => {
                error!("{:?}", e);
                return GetQuizResponse::Internal;
            }
        };
        GetQuizResponse::Ok(Json(quiz.into()))
    }
}

#[derive(Debug, Deserialize)]
pub struct LocaleQuery {
    lang_code: Option<String>,
}

#[derive(ApiResponse)]
pub enum GetQuizResponse {
    #[oai(status = 201)]
    Ok(Json<APIQuiz>),

    #[oai(status = 404)]
    NotFound,

    #[oai(status = 500)]
    Internal,
}

#[derive(ApiResponse)]
pub enum CreateQuizResponse {
    #[oai(status = 201)]
    Ok(Json<Uuid>),

    #[oai(status = 500)]
    Internal,
}
