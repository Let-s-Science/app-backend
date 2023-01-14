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
    pub id: Uuid,
    pub r#type: ChallengeType,
    pub goal: i32,
    pub description: String,
}

#[derive(Object, Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserChallenge {
    pub id: Uuid,
    pub challenge_id: Uuid,
    pub progress: i32,
    pub updated_at: DateTime<Utc>,
}
