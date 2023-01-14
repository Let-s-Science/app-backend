use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use poem::Request;
use poem_openapi::{auth::ApiKey, SecurityScheme};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: Uuid,
    exp: u64,
    nbf: u64,
}

impl From<User> for AuthUser {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            exp: get_current_timestamp() + 10000,
            nbf: get_current_timestamp(),
        }
    }
}

impl From<Uuid> for AuthUser {
    fn from(value: Uuid) -> Self {
        Self {
            id: value,
            exp: get_current_timestamp() + 10000,
            nbf: get_current_timestamp(),
        }
    }
}

#[derive(SecurityScheme, Debug)]
#[oai(
    type = "api_key",
    key_name = "X-SESSION-TOKEN",
    in = "cookie",
    checker = "jwt_checker"
)]
pub struct JWTAuthorization(pub AuthUser);

async fn jwt_checker(_: &Request, key: ApiKey) -> Option<AuthUser> {
    // For some reason, JWT's get a %22 prefix
    verify_jwt(&key.key.trim_matches('"')).ok()
}

pub fn verify_jwt(s: &str) -> jsonwebtoken::errors::Result<AuthUser> {
    let key = DecodingKey::from_secret("secret".as_ref());
    decode::<AuthUser>(s, &key, &Validation::default()).map(|token| token.claims)
}

pub fn create_jwt(id: Uuid) -> jsonwebtoken::errors::Result<String> {
    let header = Header::default();
    let key = EncodingKey::from_secret("secret".as_ref());
    let claims: AuthUser = id.into();
    encode(&header, &claims, &key)
}

pub fn get_current_timestamp() -> u64 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
