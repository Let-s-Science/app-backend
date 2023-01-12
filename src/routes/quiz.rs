use crate::{entities::quiz::APIQuiz, security::JWTAuthorization};

use super::ApiTags;
use poem::web::Data;
use poem_openapi::{
    payload::{Json, PlainText},
    OpenApi,
};
use sqlx::PgPool;

pub struct QuizAPI;

#[OpenApi]
impl QuizAPI {
    #[oai(path = "/quiz", method = "post", tag = "ApiTags::Quiz")]
    #[tracing::instrument(skip(self, pool, auth))]
    async fn create_quiz(
        &self,
        auth: JWTAuthorization,
        pool: Data<&PgPool>,
        req: Json<APIQuiz>,
    ) -> PlainText<&'static str> {
        todo!()
    }
}
