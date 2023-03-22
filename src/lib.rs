mod config;
mod error;
mod pb;
mod types;
mod utils;

pub use config::{Config, DbConfig};
pub use error::Error;
pub use pb::*;
use sqlx::{postgres::PgRow, FromRow, Row};
use types::enum_type::{OrionLearnStatus, OrionWordClassification};
pub use types::*;
pub use utils::*;

impl FromRow<'_, PgRow> for pb::Vocabulary {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");
        let updated_at: chrono::DateTime<chrono::Utc> = row.get("updated_at");

        Ok(Self {
            id: row.get("id"),
            word: row.get("word"),
            soundmark: row.get("soundmark"),
            roots: row.get("roots"),
            paraphrase: row.get("paraphrase"),
            collocations: row.get("collocations"),
            synonyms: row.get("synonyms"),
            examples: row.get("examples"),
            created_at: Some(convert_to_timestamp(&created_at)),
            updated_at: Some(convert_to_timestamp(&updated_at)),
        })
    }
}

impl FromRow<'_, PgRow> for pb::LearnWord {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");
        let updated_at: chrono::DateTime<chrono::Utc> = row.get("updated_at");
        let last_learned_at: chrono::DateTime<chrono::Utc> = row.get("last_learned_at");
        let next_learn_at: chrono::DateTime<chrono::Utc> = row.get("next_learn_at");

        let status: OrionLearnStatus = row.get("learn_status");

        Ok(Self {
            id: row.get("id"),
            word: row.get("word"),
            vocabulary_id: row.get("vocabulary_id"),
            word_list_id: row.get("word_list_id"),
            learn_count: row.get("learn_count"),
            learn_status: LearnStatus::from(status) as i32,
            last_learned_at: Some(convert_to_timestamp(&last_learned_at)),
            next_learn_at: Some(convert_to_timestamp(&next_learn_at)),
            created_at: Some(convert_to_timestamp(&created_at)),
            updated_at: Some(convert_to_timestamp(&updated_at)),
        })
    }
}

impl FromRow<'_, PgRow> for pb::WordList {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");
        let updated_at: chrono::DateTime<chrono::Utc> = row.get("updated_at");

        let class: OrionWordClassification = row.get("classification");

        Ok(Self {
            id: row.get("id"),
            word: row.get("word"),
            paraphrase: row.get("pharaphrase"),
            classification: WordClassification::from(class) as i32,
            created_at: Some(convert_to_timestamp(&created_at)),
            updated_at: Some(convert_to_timestamp(&updated_at)),
        })
    }
}

impl FromRow<'_, PgRow> for pb::Story {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at");
        let updated_at: chrono::DateTime<chrono::Utc> = row.get("updated_at");

        Ok(Self {
            id: row.get("id"),
            words: row.get("words"),
            content: row.get("content"),
            read_count: row.get("read_count"),
            created_at: Some(convert_to_timestamp(&created_at)),
            updated_at: Some(convert_to_timestamp(&updated_at)),
        })
    }
}
