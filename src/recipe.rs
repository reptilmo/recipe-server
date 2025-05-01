use std::path::Path;

use crate::error::RecipeServerError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Recipe {
    pub id: u32,
    pub name: String,
    pub ingredients: Vec<String>,
    pub preparation: String,
    pub source: String,
    pub tags: Vec<String>,
}

pub fn read_recipes_json<P: AsRef<Path>>(json_path: P) -> Result<Vec<Recipe>, RecipeServerError> {
    let json_file = std::fs::File::open(json_path.as_ref())?;
    let recipes = serde_json::from_reader(json_file)?;
    Ok(recipes)
}

//TODO: Remove!
pub fn get_recipe() -> Recipe {
    let mut ingreds = Vec::<String>::new();
    ingreds.push("4 apples".to_string());
    ingreds.push("1/2 cup water".to_string());
    ingreds.push("8 tablespoon sugar".to_string());
    ingreds.push("1/2 teaspoon cinnamon".to_string());
    ingreds.push("2 tablespoon butter".to_string());

    let mut tags = Vec::<String>::new();
    tags.push("Apples".to_string());
    tags.push("Non-Alcoholic".to_string());
    tags.push("Kid-Friendly".to_string());

    Recipe {
        id: 0u32,
        name: "Baked Apples".to_string(),
        ingredients: ingreds,
        preparation: "Select apples of uniform size.\n
Wash and core.\n
Place in a pan, cover the bottom with water.\n
Fill each cavity with sugar, a dash of powdered cinnamon and a tiny lump of butter.
Bake for thirty minutes, basting occasionally.\n
Serve around the platter of pork chops.\n"
            .to_string(),
        source: "dot com".to_string(),
        tags,
    }
}
