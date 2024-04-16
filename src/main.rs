use std::env;
use sqlx::postgres::{PgPoolOptions};

use axum::{
  routing::{delete, get, post},
  Router,
};

mod handlers;
mod models;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

use handlers::*;

#[tokio::main]
async fn main() {
  pretty_env_logger::init();
  dotenvy::dotenv().unwrap();

  let url = env::var("DATABASE_URL").unwrap();

  let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&url).await.unwrap();

  let recs = sqlx::query!("SELECT * FROM questions")
    .fetch_all(&pool)
    .await
    .unwrap();

  info!("********* Question Records *********");
  info!("{:?}", recs);

  let app = Router::new()
      .route("/question", post(create_question))
      .route("/questions", get(read_questions))
      .route("/question", delete(delete_question))
      .route("/answer", post(create_answer))
      .route("/answers", get(read_answers))
      .route("/answer", delete(delete_answer));

  let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
      .await
      .unwrap();

  axum::serve(listener, app).await.unwrap();
}
