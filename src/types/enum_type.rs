use core::fmt;

use crate::LearnStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "word_status", rename_all = "lowercase")]
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
