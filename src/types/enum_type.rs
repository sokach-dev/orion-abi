use core::fmt;

use crate::{LearnStatus, WordClassification};

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "learn_status", rename_all = "lowercase")]
pub enum OrionLearnStatus {
    New,
    Easy,
    Difficult,
    Learned,
}

impl From<OrionLearnStatus> for LearnStatus {
    fn from(value: OrionLearnStatus) -> Self {
        match value {
            OrionLearnStatus::New => LearnStatus::New,
            OrionLearnStatus::Easy => LearnStatus::Easy,
            OrionLearnStatus::Difficult => LearnStatus::Difficult,
            OrionLearnStatus::Learned => LearnStatus::Learned,
        }
    }
}

impl fmt::Display for LearnStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LearnStatus::New => write!(f, "new"),
            LearnStatus::Easy => write!(f, "easy"),
            LearnStatus::Difficult => write!(f, "difficult"),
            LearnStatus::Learned => write!(f, "learned"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "word_classification")] // word_classification是数据库中的类型名称
pub enum OrionWordClassification {
    #[sqlx(rename = "CET-4")]
    CET4,
    #[sqlx(rename = "CET-6")]
    CET6,
    #[sqlx(rename = "junior")]
    Junior,
    #[sqlx(rename = "senior")]
    Senior,
    #[sqlx(rename = "graduate")]
    Graduate,
    #[sqlx(rename = "TOEFL")]
    TOEFL,
    #[sqlx(rename = "SAT")]
    SAT,
    #[sqlx(rename = "unknown")]
    Unknown,
}

impl From<OrionWordClassification> for WordClassification {
    fn from(value: OrionWordClassification) -> Self {
        match value {
            OrionWordClassification::CET4 => WordClassification::Cet4,
            OrionWordClassification::CET6 => WordClassification::Cet6,
            OrionWordClassification::Graduate => WordClassification::Graduate,
            OrionWordClassification::Junior => WordClassification::Junior,
            OrionWordClassification::Senior => WordClassification::Senior,
            OrionWordClassification::SAT => WordClassification::Sat,
            OrionWordClassification::TOEFL => WordClassification::Toefl,
            OrionWordClassification::Unknown => WordClassification::Unkown,
        }
    }
}

impl fmt::Display for WordClassification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WordClassification::Cet4 => write!(f, "CET-4"),
            WordClassification::Cet6 => write!(f, "CET-6"),
            WordClassification::Graduate => write!(f, "graduate"),
            WordClassification::Junior => write!(f, "junior"),
            WordClassification::Senior => write!(f, "senior"),
            WordClassification::Sat => write!(f, "SAT"),
            WordClassification::Toefl => write!(f, "TOEFL"),
            WordClassification::Unkown => write!(f, "unknown"),
        }
    }
}
