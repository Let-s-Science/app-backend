use poem_openapi::{
    payload::{Json, OpenApi},
    ApiResponse,
};
use sqlx::PgPool;

use crate::{entities::challenge::Challenge, security::JWTAuthorization};

pub struct ChallengeAPI;

#[OpenApi]
impl ChallengeAPI {
    #[oai(path = "/challenge", method = "post", tags = "ApiTags::Challenge")]
    #[tracing::instrument]
    async fn create_challenge(
        &self,
        pool: Data<&PgPool>,
        req: Json<Challenge>,
        auth: JWTAuthorization,
    ) -> CreateChallengeResponse {
        todo!()
    }
}

#[derive(APIResponse)]
pub enum CreateChallengeResponse {
    #[oai(status = 201)]
    Ok(Json<Challenge>),

    #[oai(status = 500)]
    Internal,
}
