use derivative::Derivative;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Object, Clone, Derivative, Serialize, Deserialize)]
#[derivative(Debug, Default)]
pub struct Quiz {
    #[oai(read_only)]
    pub id: Uuid,
    pub content: String,
}
