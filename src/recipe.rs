use crate::error::RecipeServerError;
use crate::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Recipe {
    pub id: u32,
    pub name: String,
    pub ingredients: Vec<String>,
    pub preparation: Vec<String>,
    pub source: String,
    pub tags: Vec<String>,
}

impl axum::response::IntoResponse for &Recipe {
    fn into_response(self) -> axum::response::Response {
        (http::StatusCode::OK, axum::Json(&self)).into_response()
    }
}

pub fn read_recipes_json<P: AsRef<Path>>(json_path: P) -> Result<Vec<Recipe>, RecipeServerError> {
    let json_file = std::fs::File::open(json_path.as_ref())?;
    let recipes = serde_json::from_reader(json_file)?;
    Ok(recipes)
}
