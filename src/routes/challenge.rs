use poem::web::Data;
use poem_openapi::{param::Path, payload::Json, ApiResponse, OpenApi};
use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

use crate::{core, entities::challenge::Challenge, security::JWTAuthorization};

use super::ApiTags;

pub struct ChallengeAPI;

#[OpenApi]
impl ChallengeAPI {
    #[oai(path = "/challenge", method = "post", tag = "ApiTags::Challenge")]
    #[tracing::instrument(skip(self, pool, auth))]
    async fn create_challenge(
        &self,
        pool: Data<&PgPool>,
        req: Json<Challenge>,
        auth: JWTAuthorization,
    ) -> CreateChallengeResponse {
        match core::challenge::insert_challenge(&pool, &req.0).await {
            Ok(id) => CreateChallengeResponse::Ok(Json(id)),
            Err(e) => {
                error!("error while inserting challenge: {:?}", e);
                CreateChallengeResponse::Internal
            }
        }
    }

    #[oai(path = "/challenge/:id", method = "get", tag = "ApiTags::Challenge")]
    #[tracing::instrument(skip(self, pool, id, auth))]
    async fn get_challenge(
        &self,
        pool: Data<&PgPool>,
        id: Path<Uuid>,
        auth: JWTAuthorization,
    ) -> GetChallengeResponse {
        match core::challenge::get_challenge(&pool, id.0).await {
            Ok(Some(ch)) => GetChallengeResponse::Ok(Json(ch)),
            Ok(None) => GetChallengeResponse::NotFound,
            Err(e) => {
                error!("error while getting challenge: {:?}", e);
                GetChallengeResponse::Internal
            }
        }
    }
}

#[derive(ApiResponse, Debug)]
pub enum CreateChallengeResponse {
    #[oai(status = 201)]
    Ok(Json<Uuid>),

    #[oai(status = 500)]
    Internal,
}

#[derive(ApiResponse, Debug)]
pub enum GetChallengeResponse {
    #[oai(status = 201)]
    Ok(Json<Challenge>),

    #[oai(status = 500)]
    Internal,

    #[oai(status = 404)]
    NotFound,
}
