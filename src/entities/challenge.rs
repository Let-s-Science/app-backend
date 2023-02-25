use chrono::{DateTime, Utc};
use poem_openapi::{Enum, Object};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

#[derive(Type, Enum, Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[sqlx(type_name = "challengetype", rename_all = "lowercase")]
pub enum ChallengeType {
    Counter,
    DailyChallenge,
}

impl Default for ChallengeType {
    fn default() -> Self {
        Self::Counter
    }
}

#[derive(Object, Debug, Clone, Default, Serialize, Deserialize)]
pub struct Challenge {
    #[oai(read_only)]
    pub id: Uuid,
    pub title: String,
    pub category: String,
    pub r#type: ChallengeType,
    pub goal: i32,
    pub description: String,
}

#[derive(Object, Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserChallenge {
    pub user_id: Uuid,
    pub challenge_id: Uuid,
    pub progress: i32,
    pub updated_at: Option<DateTime<Utc>>,
}
