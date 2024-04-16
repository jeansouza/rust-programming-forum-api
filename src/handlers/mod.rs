use crate::models::*;
use axum::{response::IntoResponse, Json};

// ---- CRUD for Questions ----

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    todo!()
}

pub async fn read_questions() -> impl IntoResponse {
    todo!()
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) {
    todo!()
}

// ---- CRUD for Answers ----

pub async fn create_answer(Json(answer): Json<Answer>) -> impl IntoResponse {
  todo!()
}

pub async fn read_answers(Json(question_id): Json<QuestionId>) -> impl IntoResponse {
  todo!()
}

pub async fn delete_answer(Json(answer_id): Json<AnswerId>) {
  todo!()
}
