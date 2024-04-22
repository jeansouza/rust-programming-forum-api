use crate::{
  models::{Answer, AnswerDetail, AnswerId, DBError, Question, QuestionDetail, QuestionId},
  persistance::{answers_dao::AnswersDao, questions_dao::QuestionsDao},
};

#[derive(Debug, PartialEq)]
pub enum HandlerError {
  BadRequest(String),
  InternalError(String),
}

impl HandlerError {
  pub fn default_internal_error() -> Self {
      HandlerError::InternalError("Something went wrong! Please try again.".to_owned())
  }
}

pub async fn create_question(
  question: Question,
  // We are using a trait object here so that inner handlers do not depend on concrete DAO implementations
  questions_dao: &(dyn QuestionsDao + Sync + Send),
) -> Result<QuestionDetail, HandlerError> {
  let question = questions_dao.create_question(question).await;

  match question {
      Ok(question) => Ok(question),
      Err(err) => {
          error!("Error to create question: {}", err);
          Err(HandlerError::default_internal_error())
      }
  }
}

pub async fn read_questions(
  questions_dao: &(dyn QuestionsDao + Sync + Send),
) -> Result<Vec<QuestionDetail>, HandlerError> {
  let questions = questions_dao.get_questions().await;

  match questions {
      Ok(questions) => Ok(questions),
      Err(err) => {
        error!("Error to list questions: {}", err);
        Err(HandlerError::default_internal_error())
      }
  }
}

pub async fn delete_question(
  question_uuid: QuestionId,
  questions_dao: &(dyn QuestionsDao + Sync + Send),
) -> Result<(), HandlerError> {
  let result = questions_dao.delete_question(question_uuid.question_uuid).await;

  if result.is_err() {
    error!("Error to delete question: {}", result.err().unwrap());
    return Err(HandlerError::default_internal_error());
  }

  Ok(())
}

pub async fn create_answer(
  answer: Answer,
  answers_dao: &(dyn AnswersDao + Send + Sync),
) -> Result<AnswerDetail, HandlerError> {
  let answer = answers_dao.create_answer(answer).await;

  match answer {
      Ok(answer) => Ok(answer),
      Err(err) => {
        error!("Error to create answer: {}", err);

          match err {
              DBError::InvalidUUID(s) => Err(HandlerError::BadRequest(s)),
              _ => Err(HandlerError::default_internal_error()),
          }
      }
  }
}

pub async fn read_answers(
  question_uuid: QuestionId,
  answers_dao: &(dyn AnswersDao + Send + Sync),
) -> Result<Vec<AnswerDetail>, HandlerError> {
  let answers = answers_dao.get_answers(question_uuid.question_uuid).await;

  match answers {
      Ok(answers) => Ok(answers),
      Err(err) => {
        error!("Error to list answers: {}", err);
        Err(HandlerError::default_internal_error())
      }
  }
}

pub async fn delete_answer(
  answer_uuid: AnswerId,
  answers_dao: &(dyn AnswersDao + Send + Sync),
) -> Result<(), HandlerError> {
  let result = answers_dao.delete_answer(answer_uuid.answer_uuid).await;

  if result.is_err() {
    error!("Error to delete answer: {}", result.err().unwrap());
    return Err(HandlerError::default_internal_error());
  }

  Ok(())
}

// ***********************************************************
//                           Tests
// ***********************************************************

#[cfg(test)]
mod tests {
  use super::*;

  use async_trait::async_trait;
  use tokio::sync::Mutex;

  struct QuestionsDaoMock {
      create_question_response: Mutex<Option<Result<QuestionDetail, DBError>>>,
      delete_question_response: Mutex<Option<Result<(), DBError>>>,
      get_questions_response: Mutex<Option<Result<Vec<QuestionDetail>, DBError>>>,
  }

  impl QuestionsDaoMock {
      pub fn new() -> Self {
          QuestionsDaoMock {
              create_question_response: Mutex::new(None),
              delete_question_response: Mutex::new(None),
              get_questions_response: Mutex::new(None),
          }
      }
      pub fn mock_create_question(&mut self, response: Result<QuestionDetail, DBError>) {
          self.create_question_response = Mutex::new(Some(response));
      }
      pub fn mock_delete_question(&mut self, response: Result<(), DBError>) {
          self.delete_question_response = Mutex::new(Some(response));
      }
      pub fn mock_get_questions(&mut self, response: Result<Vec<QuestionDetail>, DBError>) {
          self.get_questions_response = Mutex::new(Some(response));
      }
  }

  #[async_trait]
  impl QuestionsDao for QuestionsDaoMock {
      async fn create_question(&self, _: Question) -> Result<QuestionDetail, DBError> {
          self.create_question_response
              .lock()
              .await
              .take()
              .expect("create_question_response should not be None.")
      }
      async fn delete_question(&self, _: String) -> Result<(), DBError> {
          self.delete_question_response
              .lock()
              .await
              .take()
              .expect("delete_question_response should not be None.")
      }
      async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
          self.get_questions_response
              .lock()
              .await
              .take()
              .expect("get_questions_response should not be None.")
      }
  }

  struct AnswersDaoMock {
      create_answer_response: Mutex<Option<Result<AnswerDetail, DBError>>>,
      delete_answer_response: Mutex<Option<Result<(), DBError>>>,
      get_answers_response: Mutex<Option<Result<Vec<AnswerDetail>, DBError>>>,
  }

  impl AnswersDaoMock {
      pub fn new() -> Self {
          AnswersDaoMock {
              create_answer_response: Mutex::new(None),
              delete_answer_response: Mutex::new(None),
              get_answers_response: Mutex::new(None),
          }
      }
      pub fn mock_create_answer(&mut self, response: Result<AnswerDetail, DBError>) {
          self.create_answer_response = Mutex::new(Some(response));
      }
      pub fn mock_delete_answer(&mut self, response: Result<(), DBError>) {
          self.delete_answer_response = Mutex::new(Some(response));
      }
      pub fn mock_get_answers(&mut self, response: Result<Vec<AnswerDetail>, DBError>) {
          self.get_answers_response = Mutex::new(Some(response));
      }
  }

  #[async_trait]
  impl AnswersDao for AnswersDaoMock {
      async fn create_answer(&self, _: Answer) -> Result<AnswerDetail, DBError> {
          self.create_answer_response
              .lock()
              .await
              .take()
              .expect("create_answer_response should not be None.")
      }
      async fn delete_answer(&self, _: String) -> Result<(), DBError> {
          self.delete_answer_response
              .lock()
              .await
              .take()
              .expect("delete_answer_response should not be None.")
      }
      async fn get_answers(&self, _: String) -> Result<Vec<AnswerDetail>, DBError> {
          self.get_answers_response
              .lock()
              .await
              .take()
              .expect("get_answers_response should not be None.")
      }
  }

  #[tokio::test]
  async fn create_question_should_return_question() {
      let question = Question {
          title: "test title".to_owned(),
          description: "test description".to_owned(),
      };

      let question_detail = QuestionDetail {
          question_uuid: "123".to_owned(),
          title: question.title.clone(),
          description: question.description.clone(),
          created_at: "now".to_owned(),
      };

      let mut questions_dao = QuestionsDaoMock::new();

      questions_dao.mock_create_question(Ok(question_detail.clone()));

      let questions_dao: Box<dyn QuestionsDao + Send + Sync> = Box::new(questions_dao);

      let result = create_question(question, questions_dao.as_ref()).await;

      assert!(result.is_ok());
      assert_eq!(result.unwrap(), question_detail);
  }

  #[tokio::test]
  async fn create_question_should_return_error() {
      let question = Question {
          title: "test title".to_owned(),
          description: "test description".to_owned(),
      };

      let mut questions_dao = QuestionsDaoMock::new();

      questions_dao.mock_create_question(Err(DBError::InvalidUUID("test".to_owned())));

      let questions_dao: Box<dyn QuestionsDao + Send + Sync> = Box::new(questions_dao);

      let result = create_question(question, questions_dao.as_ref()).await;

      assert!(result.is_err());
      assert!(
          std::mem::discriminant(&result.unwrap_err())
              == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
      );
  }

  #[tokio::test]
  async fn read_questions_should_return_questions() {
      let question_detail = QuestionDetail {
          question_uuid: "123".to_owned(),
          title: "test title".to_owned(),
          description: "test description".to_owned(),
          created_at: "now".to_owned(),
      };

      let mut questions_dao = QuestionsDaoMock::new();

      questions_dao.mock_get_questions(Ok(vec![question_detail.clone()]));

      let questions_dao: Box<dyn QuestionsDao + Send + Sync> = Box::new(questions_dao);

      let result = read_questions(questions_dao.as_ref()).await;

      assert!(result.is_ok());
      assert_eq!(result.unwrap(), vec![question_detail]);
  }

  #[tokio::test]
  async fn read_questions_should_return_error() {
      let mut questions_dao = QuestionsDaoMock::new();

      questions_dao.mock_get_questions(Err(DBError::InvalidUUID("test".to_owned())));

      let questions_dao: Box<dyn QuestionsDao + Send + Sync> = Box::new(questions_dao);

      let result = read_questions(questions_dao.as_ref()).await;

      assert!(result.is_err());
      assert!(
          std::mem::discriminant(&result.unwrap_err())
              == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
      );
  }

  #[tokio::test]
  async fn delete_question_should_succeed() {
      let question_id = QuestionId {
          question_uuid: "123".to_owned(),
      };

      let mut questions_dao = QuestionsDaoMock::new();

      questions_dao.mock_delete_question(Ok(()));

      let questions_dao: Box<dyn QuestionsDao + Send + Sync> = Box::new(questions_dao);

      let result = delete_question(question_id, questions_dao.as_ref()).await;

      assert!(result.is_ok());
      assert_eq!(result.unwrap(), ());
  }

  #[tokio::test]
  async fn delete_question_should_return_error() {
      let question_id = QuestionId {
          question_uuid: "123".to_owned(),
      };

      let mut questions_dao = QuestionsDaoMock::new();

      questions_dao.mock_delete_question(Err(DBError::InvalidUUID("test".to_owned())));

      let questions_dao: Box<dyn QuestionsDao + Send + Sync> = Box::new(questions_dao);

      let result = delete_question(question_id, questions_dao.as_ref()).await;

      assert!(result.is_err());
      assert!(
          std::mem::discriminant(&result.unwrap_err())
              == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
      );
  }

  #[tokio::test]
  async fn create_answer_should_return_answer() {
      let answer = Answer {
          question_uuid: "123".to_owned(),
          content: "test content".to_owned(),
      };

      let answer_detail = AnswerDetail {
          answer_uuid: "456".to_owned(),
          question_uuid: answer.question_uuid.clone(),
          content: answer.content.clone(),
          created_at: "now".to_owned(),
      };

      let mut answers_dao = AnswersDaoMock::new();

      answers_dao.mock_create_answer(Ok(answer_detail.clone()));

      let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(answers_dao);

      let result = create_answer(answer, answers_dao.as_ref()).await;

      assert!(result.is_ok());
      assert_eq!(result.unwrap(), answer_detail);
  }

  #[tokio::test]
  async fn create_answer_should_return_bad_request_error() {
      let answer = Answer {
          question_uuid: "123".to_owned(),
          content: "test content".to_owned(),
      };

      let mut answers_dao = AnswersDaoMock::new();

      answers_dao.mock_create_answer(Err(DBError::InvalidUUID("test".to_owned())));

      let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(answers_dao);

      let result = create_answer(answer, answers_dao.as_ref()).await;

      assert!(result.is_err());
      assert!(
          std::mem::discriminant(&result.unwrap_err())
              == std::mem::discriminant(&HandlerError::BadRequest("".to_owned()))
      );
  }

  #[tokio::test]
  async fn create_answer_should_return_internal_error() {
      let answer = Answer {
          question_uuid: "123".to_owned(),
          content: "test content".to_owned(),
      };

      let mut answers_dao = AnswersDaoMock::new();

      answers_dao.mock_create_answer(Err(DBError::Other(Box::new(std::io::Error::new(
          std::io::ErrorKind::Other,
          "oh no!",
      )))));

      let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(answers_dao);

      let result = create_answer(answer, answers_dao.as_ref()).await;

      assert!(result.is_err());
      assert!(
          std::mem::discriminant(&result.unwrap_err())
              == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
      );
  }

  #[tokio::test]
  async fn read_answers_should_return_answers() {
      let answer_detail = AnswerDetail {
          answer_uuid: "456".to_owned(),
          question_uuid: "123".to_owned(),
          content: "test content".to_owned(),
          created_at: "now".to_owned(),
      };

      let question_id = QuestionId {
          question_uuid: "123".to_owned(),
      };

      let mut answers_dao = AnswersDaoMock::new();

      answers_dao.mock_get_answers(Ok(vec![answer_detail.clone()]));

      let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(answers_dao);

      let result = read_answers(question_id, answers_dao.as_ref()).await;

      assert!(result.is_ok());
      assert_eq!(result.unwrap(), vec![answer_detail]);
  }

  #[tokio::test]
  async fn read_answers_should_return_error() {
      let question_id = QuestionId {
          question_uuid: "123".to_owned(),
      };

      let mut answers_dao = AnswersDaoMock::new();

      answers_dao.mock_get_answers(Err(DBError::InvalidUUID("test".to_owned())));

      let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(answers_dao);

      let result = read_answers(question_id, answers_dao.as_ref()).await;

      assert!(result.is_err());
      assert!(
          std::mem::discriminant(&result.unwrap_err())
              == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
      );
  }

  #[tokio::test]
  async fn delete_answer_should_succeed() {
      let answer_id = AnswerId {
          answer_uuid: "123".to_owned(),
      };

      let mut answers_dao = AnswersDaoMock::new();

      answers_dao.mock_delete_answer(Ok(()));

      let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(answers_dao);

      let result = delete_answer(answer_id, answers_dao.as_ref()).await;

      assert!(result.is_ok());
      assert_eq!(result.unwrap(), ());
  }

  #[tokio::test]
  async fn delete_answer_should_return_error() {
      let answer_id = AnswerId {
          answer_uuid: "123".to_owned(),
      };

      let mut answers_dao = AnswersDaoMock::new();

      answers_dao.mock_delete_answer(Err(DBError::InvalidUUID("test".to_owned())));

      let answers_dao: Box<dyn AnswersDao + Send + Sync> = Box::new(answers_dao);

      let result = delete_answer(answer_id, answers_dao.as_ref()).await;

      assert!(result.is_err());
      assert!(
          std::mem::discriminant(&result.unwrap_err())
              == std::mem::discriminant(&HandlerError::InternalError("".to_owned()))
      );
  }
}
