use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseError {
  pub error: String,
}