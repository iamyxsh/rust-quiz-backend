use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSql, Validate, FromSql)]
pub struct Questions {
    #[validate(length(min = 6, max = 32))]
    question: String,

    options: Vec<String>,

    #[validate(length(min = 6, max = 32))]
    answers: String,
}

#[derive(Debug, Serialize, Deserialize, ToSql, FromSql)]
pub struct Quiz {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub questions: Questions,
    pub creator: i32,
}
