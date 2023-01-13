use crate::{
    core::{self, user::User},
    security::{create_jwt, JWTAuthorization},
};

use super::ApiTags;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use derivative::Derivative;
use password_hash::{rand_core::OsRng, SaltString};
use poem::{
    session::Session,
    web::{
        cookie::{Cookie, CookieJar},
        Data,
    },
};
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi};
use sqlx::PgPool;
use tracing::error;
use unicode_normalization::UnicodeNormalization;
use uuid::Uuid;

pub struct AuthAPI;

#[OpenApi]
impl AuthAPI {
    #[oai(path = "/register", method = "post", tag = "ApiTags::User")]
    #[tracing::instrument(skip(self, pool, jar))]
    async fn register(
        &self,
        jar: &CookieJar,
        pool: Data<&PgPool>,
        req: Json<RegisterRequest>,
    ) -> RegisterResponse {
        let mut user = User {
            name: req.name.clone(),
            is_guest: req.is_guest,
            avatar_seed: req.avatar_seed.clone(),
            ..User::default()
        };
        user.name = req.name.clone();
        if !req.is_guest {
            let Some(password) = req.password.as_ref() else {
                return RegisterResponse::BadRequest;
            };
            let Ok(hash) = hash_password(password) else {
                return RegisterResponse::Internal;
            };
            let Some(email) = req.email.as_ref() else {
                return RegisterResponse::BadRequest;
            };
            user.hash = Some(hash);
            user.email = Some(email.clone());
        }
        let db_user = match core::user::insert_user(&pool, &user).await {
            Ok(Some(u)) => u,
            Ok(None) => return RegisterResponse::UserAlreadyExists,
            Err(e) => {
                error!("database get user error {:?}", e);
                return RegisterResponse::Internal;
            }
        };
        if create_session(jar, db_user).is_err() {
            return RegisterResponse::Internal;
        }
        RegisterResponse::Ok(Json(db_user))
    }

    #[oai(path = "/login", method = "post", tag = "ApiTags::User")]
    #[tracing::instrument(skip(self, pool))]
    async fn login(&self, pool: Data<&PgPool>, req: Json<LoginRequest>) -> LoginResponse {
        let db_user = match core::user::get_user_by_email(&pool, &req.email).await {
            Ok(Some(u)) => u,
            Ok(None) => return LoginResponse::Unauthorized,
            Err(e) => {
                error!("database get user error: {:?}", e);
                return LoginResponse::Internal;
            }
        };

        let Ok(hash) = hash_password(&req.password) else {
            error!("internal hashing error ");
            return LoginResponse::Internal;
        };
        if let Some(db_hash) = db_user.hash {
            if hash == db_hash {
                return LoginResponse::Ok(Json(db_user.id));
            }
        }
        LoginResponse::Unauthorized
    }

    #[oai(path = "/restricted", method = "get", tag = "ApiTags::User")]
    #[tracing::instrument(skip(self, _pool, auth))]
    async fn restricted(&self, _pool: Data<&PgPool>, auth: JWTAuthorization) -> RegisterResponse {
        println!("{:?}", auth);
        RegisterResponse::UserAlreadyExists
    }
}

fn create_session(jar: &CookieJar, user_id: Uuid) -> jsonwebtoken::errors::Result<()> {
    let jwt = create_jwt(user_id)?;
    let cookie = Cookie::new("X-SESSION-TOKEN", jwt);
    jar.add(cookie);
    Ok(())
}

#[derive(ApiResponse)]
pub enum RegisterResponse {
    #[oai(status = 201)]
    #[oai(header(name = "X-SESSION-TOKEN", type = "String"))]
    Ok(Json<Uuid>),

    #[oai(status = 409)]
    UserAlreadyExists,

    #[oai(status = 400)]
    BadRequest,

    #[oai(status = 500)]
    Internal,
}

#[derive(ApiResponse)]
pub enum LoginResponse {
    #[oai(status = 200)]
    #[oai(header(name = "X-SESSION-TOKEN", type = "String"))]
    Ok(Json<Uuid>),

    #[oai(status = 401)]
    Unauthorized,

    #[oai(status = 500)]
    Internal,
}

#[derive(Derivative, Object)]
#[derivative(Debug)]
pub struct LoginRequest {
    email: String,
    #[derivative(Debug = "ignore")]
    password: String,
}

#[derive(Derivative, Object)]
#[derivative(Debug)]
pub struct RegisterRequest {
    name: String,
    email: Option<String>,
    #[derivative(Debug = "ignore")]
    password: Option<String>,
    avatar_seed: String,
    is_guest: bool,
}

fn normalize(pass: &str) -> String {
    pass.nfkc().collect::<String>()
}

/// Creates a hash with the given password.
#[tracing::instrument(name = "Hash password", skip_all)]
pub fn hash_password(password: &str) -> Result<String, password_hash::Error> {
    let password = normalize(password);
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    let s = hash.serialize();
    Ok(s.as_str().to_owned())
}

/// Verifies that the given password results in the given hash.
#[tracing::instrument(name = "Validate password", skip_all)]
pub fn verify_password(password: &str, hash: &str) -> Result<bool, password_hash::Error> {
    let pass = normalize(password);
    let argon2 = Argon2::default();
    let result = argon2.verify_password(pass.as_bytes(), &PasswordHash::new(hash)?);
    match result {
        Ok(_) => Ok(true),
        Err(password_hash::Error::Password) => Ok(false),
        Err(e) => Err(e),
    }
}
