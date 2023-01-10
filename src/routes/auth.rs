use super::ApiTags;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use password_hash::{rand_core::OsRng, SaltString};
use poem::Route;
use poem_openapi::{
    payload::{Json, PlainText},
    Object, OpenApi, OpenApiService,
};
use unicode_normalization::UnicodeNormalization;
use uuid::Uuid;

struct AuthApi;

pub fn routes() -> Route {
    let openapi_service =
        OpenApiService::new(AuthApi, "Auth API", "0.1").server("http://localhost:3000/api");
    let ui = openapi_service.rapidoc();
    Route::new().nest("/api", openapi_service).nest("/", ui)
}

#[OpenApi]
impl AuthApi {
    #[oai(path = "/register", method = "post", tag = "ApiTags::User")]
    #[tracing::instrument(skip(self))]
    async fn register(&self, req: Json<User>) -> PlainText<&'static str> {
        let _hash = hash_password(&req.password);
        PlainText("Hello, World!")
    }
}

#[derive(Object, Debug)]
struct User {
    #[oai(read_only)]
    id: Uuid,
    #[oai(validator(max_length = 64))]
    name: String,
    email: String,
    password: String,
    is_guest: bool,
    avatar_seed: String,
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
        Err(e) => Err(e.into()),
    }
}
