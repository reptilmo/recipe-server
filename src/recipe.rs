use std::path::Path;

use crate::error::RecipeServerError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Recipe {
    pub id: u32,
    pub name: String,
    pub ingredients: Vec<String>,
    pub preparation: Vec<String>,
    pub source: String,
    pub tags: Vec<String>,
}

pub fn read_recipes_json<P: AsRef<Path>>(json_path: P) -> Result<Vec<Recipe>, RecipeServerError> {
    let json_file = std::fs::File::open(json_path.as_ref())?;
    let recipes = serde_json::from_reader(json_file)?;
    Ok(recipes)
}
