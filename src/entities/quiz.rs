use chrono::{DateTime, Utc};
use derivative::Derivative;
use poem_openapi::{Object, Union};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Object, Clone, Debug, Default, Derivative, Serialize, Deserialize)]
pub struct Translation {
    #[oai(read_only)]
    pub id: Uuid,
    pub content: String,
}

#[derive(Object, Clone, Debug, Default, Serialize, Deserialize)]
pub struct MultipleChoiceQuestion {
    pub answers: Vec<String>,
    pub correct_answer: i32,
}

#[derive(Object, Clone, Debug, Default, Serialize, Deserialize)]
pub struct NumericQuestion {
    pub range_start: i32,
    pub range_end: i32,
}

#[derive(Object, Clone, Debug, Default, Serialize, Deserialize)]
pub struct TrueOrFalseQuestion {
    pub correct_answer: bool,
}

#[derive(Union, Clone, Debug, Serialize, Deserialize)]
#[oai(discriminator_name = "type")]
#[serde(tag = "type")]
pub enum QuestionType {
    MultipleChoice(MultipleChoiceQuestion),
    Numeric(NumericQuestion),
    TrueOrFalse(TrueOrFalseQuestion),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DBQuizQuestion {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub question: String,
    pub data: sqlx::types::Json<QuestionType>,
}

#[derive(Object, Clone, Debug, Serialize, Deserialize)]
pub struct APIQuizQuestion {
    #[oai(read_only)]
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub question: String,
    pub data: QuestionType,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DBQuiz {
    pub id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub questions: Vec<DBQuizQuestion>,
}

#[derive(Object, Clone, Debug, Default, Serialize, Deserialize)]
pub struct APIQuiz {
    #[oai(read_only)]
    pub id: Uuid,
    pub title: String,
    #[oai(read_only)]
    pub created_at: DateTime<Utc>,
    #[oai(read_only)]
    pub created_by: Uuid,
    pub questions: Vec<APIQuizQuestion>,
}

impl From<APIQuizQuestion> for DBQuizQuestion {
    fn from(value: APIQuizQuestion) -> Self {
        Self {
            id: value.id,
            quiz_id: value.quiz_id,
            question: value.question,
            data: sqlx::types::Json(value.data),
        }
    }
}

impl From<APIQuiz> for DBQuiz {
    fn from(value: APIQuiz) -> Self {
        Self {
            id: value.id,
            title: value.title,
            created_at: value.created_at,
            created_by: value.created_by,
            questions: value.questions.into_iter().map(|q| q.into()).collect(),
        }
    }
}

impl From<DBQuizQuestion> for APIQuizQuestion {
    fn from(value: DBQuizQuestion) -> Self {
        Self {
            id: value.id,
            quiz_id: value.quiz_id,
            question: value.question,
            data: value.data.0,
        }
    }
}

impl From<DBQuiz> for APIQuiz {
    fn from(value: DBQuiz) -> Self {
        Self {
            id: value.id,
            title: value.title,
            created_at: value.created_at,
            created_by: value.created_by,
            questions: value.questions.into_iter().map(|q| q.into()).collect(),
        }
    }
}
