use async_trait::async_trait;
use sqlx::{types::Uuid, PgPool};

use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
      AnswersDaoImpl {
        db
      }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let uuid = Uuid::parse_str(&answer.question_uuid)
          .map_err(|err| {
            DBError::InvalidUUID(err.to_string())
          })?;

        let record = sqlx::query!("INSERT INTO answers (question_uuid, content) VALUES ($1, $2) RETURNING *", uuid, answer.content)
          .fetch_one(&self.db)
          .await
          .map_err(|err: sqlx::Error| match err {
            sqlx::Error::Database(db_err) => {
              if db_err.is_foreign_key_violation() {
                DBError::InvalidUUID(db_err.to_string())
              } else {
                DBError::Other(Box::new(db_err))
              }
            },
            err => {
              DBError::Other(Box::new(err))
            }
          })?;

        Ok(AnswerDetail {
          answer_uuid: record.answer_uuid.to_string(),
          question_uuid: record.question_uuid.to_string(),
          content: record.content,
          created_at: record.created_at.to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let uuid = Uuid::parse_str(&answer_uuid)
          .map_err(|err| {
            DBError::InvalidUUID(err.to_string())
          })?;

        sqlx::query!("DELETE FROM answers WHERE answer_uuid = $1", uuid)
          .execute(&self.db)
          .await
          .map_err(|err: sqlx::Error| { DBError::Other(Box::new(err)) })?;

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid = Uuid::parse_str(&question_uuid)
          .map_err(|err| {
            DBError::InvalidUUID(err.to_string())
          })?;

        let records = sqlx::query!("SELECT * FROM answers WHERE question_uuid = $1", uuid)
          .fetch_all(&self.db)
          .await
          .map_err(|err: sqlx::Error| { DBError::Other(Box::new(err)) })?;

        let answers = records
          .into_iter()
          .map(|record| {
            AnswerDetail {
              answer_uuid: record.answer_uuid.to_string(),
              question_uuid: record.question_uuid.to_string(),
              content: record.content,
              created_at: record.created_at.to_string(),
            }
          })
          .collect();

        Ok(answers)
    }
}
