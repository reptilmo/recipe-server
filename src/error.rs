extern crate serde_json;
extern crate thiserror;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RecipeServerError {
    #[error("could not find recipe json: {0}")]
    JokesNotFound(#[from] std::io::Error),
    #[error("could not read recipe json: {0}")]
    JokeMisformat(#[from] serde_json::Error),
    #[error("invalid database uri: {0}")]
    InvalidDbUri(String),
    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("sqlx migrate error: {0}")]
    MigrateError(#[from] sqlx::migrate::MigrateError),
}
