use poem::web::Data;
use poem_openapi::{
    param::{Path, Query},
    payload::Json,
    ApiResponse, Object, OpenApi,
};
use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

use crate::{
    core,
    entities::challenge::{Challenge, UserChallenge},
    security::JWTAuthorization,
};

use super::ApiTags;

pub struct ChallengeAPI;

#[OpenApi]
impl ChallengeAPI {
    #[oai(path = "/api/challenge", method = "post", tag = "ApiTags::Challenge")]
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

    #[oai(
        path = "/api/challenge/:id",
        method = "get",
        tag = "ApiTags::Challenge"
    )]
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

    #[oai(
        path = "/api/challenge/:id/progress",
        method = "post",
        tag = "ApiTags::Challenge"
    )]
    #[tracing::instrument(skip(self, pool, id, auth))]
    async fn add_progress(
        &self,
        pool: Data<&PgPool>,
        id: Path<Uuid>,
        auth: JWTAuthorization,
        req: Json<AddProgressRequest>,
    ) -> AddProgressResponse {
        match core::challenge::add_progress(&pool, auth.0.id, id.0, req.progress).await {
            Ok(Some(ch)) => AddProgressResponse::Ok(Json(ch)),
            Ok(None) => AddProgressResponse::NotFound,
            Err(e) => {
                error!(
                    "error {:?} while adding progress {:?} to challenge {:?}",
                    e, req.progress, id.0
                );
                AddProgressResponse::Internal
            }
        }
    }

    #[oai(
        path = "/api/challenges/self",
        method = "get",
        tag = "ApiTags::Challenge"
    )]
    async fn get_user_challenges(
        &self,
        pool: Data<&PgPool>,
        challenge_id: Query<Option<Uuid>>,
        auth: JWTAuthorization,
    ) -> GetUserChallengesResponse {
        match core::challenge::get_user_challenges(&pool, auth.0.id, challenge_id.0).await {
            Ok(resp) => GetUserChallengesResponse::Ok(Json(resp)),
            Err(e) => {
                error!(
                    "error {:?} while retrieving challenges for user {:?}",
                    e, auth.0.id
                );
                GetUserChallengesResponse::Internal
            }
        }
    }

    #[oai(path = "/api/challenges", method = "get", tag = "ApiTags::Challenge")]
    async fn get_challenges(&self, pool: Data<&PgPool>) -> GetChallengesResponse {
        match core::challenge::get_challenges(&pool).await {
            Ok(resp) => GetChallengesResponse::Ok(Json(resp)),
            Err(e) => {
                error!("error {:?} while retrieving challenges", e);
                GetChallengesResponse::Internal
            }
        }
    }

    #[oai(
        path = "/api/challenge/:id/progress",
        method = "delete",
        tag = "ApiTags::Challenge"
    )]
    #[tracing::instrument(skip(self, pool, id, auth))]
    async fn delete_progress(
        &self,
        pool: Data<&PgPool>,
        id: Path<Uuid>,
        auth: JWTAuthorization,
    ) -> DeleteProgressResponse {
        match core::challenge::delete_progress(&pool, auth.0.id, id.0).await {
            Ok(Some(_)) => DeleteProgressResponse::Ok,
            Ok(None) => DeleteProgressResponse::NotFound,
            Err(e) => {
                error!(
                    "error {:?} while removing progress from challenge {:?}",
                    e, id.0
                );
                DeleteProgressResponse::Internal
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

#[derive(Object, Debug)]
pub struct AddProgressRequest {
    progress: i32,
}

#[derive(ApiResponse, Debug)]
pub enum AddProgressResponse {
    #[oai(status = 200)]
    Ok(Json<UserChallenge>),

    #[oai(status = 500)]
    Internal,

    #[oai(status = 404)]
    NotFound,
}

#[derive(ApiResponse)]
pub enum GetUserChallengesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<UserChallenge>>),

    #[oai(status = 500)]
    Internal,
}

#[derive(ApiResponse)]
pub enum GetChallengesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<Challenge>>),

    #[oai(status = 500)]
    Internal,
}

#[derive(ApiResponse)]
pub enum DeleteProgressResponse {
    #[oai(status = 200)]
    Ok,

    #[oai(status = 404)]
    NotFound,

    #[oai(status = 500)]
    Internal,
}
