use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Admin {
    pub admin_id: i32,
    pub username: String,
    pub password_hash: String,
    pub created_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Player {
    pub player_id: i32,
    pub username: String,
    pub password_hash: String,
    pub created_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Game {
    pub game_id: i32,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Question {
    pub question_id: i32,
    pub question_text: String,
    pub correct_answer: Answer,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct GameQuestionFull {
    pub question_id: i32,
    pub question_text: String,
    pub correct_answer: Answer,
    pub question_order: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Answer {
    True,
    False,
}

impl Answer {
    pub fn to_bool(&self) -> bool {
        match self {
            Answer::True => true,
            Answer::False => false,
        }
    }
}

impl PartialEq<Answer> for bool {
    fn eq(&self, other: &Answer) -> bool {
        match other {
            Answer::True => *self,
            Answer::False => !*self,
        }
    }
}

impl From<i8> for Answer {
    fn from(value: i8) -> Self {
        match value {
            0 => Answer::False,
            1 => Answer::True,
            _ => panic!("Invalid value for Answer"),
        }
    }
}

impl From<Answer> for i8 {
    fn from(value: Answer) -> Self {
        match value {
            Answer::False => 0,
            Answer::True => 1,
        }
    }
}

impl From<bool> for Answer {
    fn from(value: bool) -> Self {
        if value {
            Answer::True
        } else {
            Answer::False
        }
    }
}

impl From<Answer> for bool {
    fn from(value: Answer) -> Self {
        match value {
            Answer::False => false,
            Answer::True => true,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct GameQuestion {
    pub game_id: i32,
    pub question_id: i32,
    pub question_order: i32,
}
