use crate::error::RecipeServerError;
use crate::recipe::{Recipe, read_recipes_json};

use sqlx::{SqlitePool, migrate::MigrateDatabase, sqlite};

pub fn get_database_uri(db_uri: Option<&str>) -> String {
    if let Some(db_uri) = db_uri {
        db_uri.to_string()
    } else if let Ok(db_uri) = std::env::var("RECIPE_SERVER_DB_URI") {
        db_uri
    } else {
        "sqlite://db/recipe-server.db".to_string()
    }
}

fn extract_database_dir(db_uri: &str) -> Result<&str, RecipeServerError> {
    if db_uri.starts_with("sqlite://") && db_uri.ends_with(".db") {
        let start = db_uri.find(':').unwrap() + 3;
        let mut path = &db_uri[start..];
        if let Some(end) = path.rfind('/') {
            path = &path[..end];
        } else {
            path = "";
        }
        Ok(path)
    } else {
        Err(RecipeServerError::InvalidDbUri(db_uri.to_string()))
    }
}

pub async fn init_database(
    init_from: Option<std::path::PathBuf>,
    db_uri: &str,
) -> Result<(), RecipeServerError> {
    if !sqlite::Sqlite::database_exists(db_uri).await? {
        let db_dir = extract_database_dir(db_uri)?;
        std::fs::create_dir_all(db_dir)?;
        sqlite::Sqlite::create_database(db_uri).await?
    }

    let db = SqlitePool::connect(db_uri).await?;
    sqlx::migrate!().run(&db).await?;

    if let Some(path) = init_from {
        let recipes = read_recipes_json(path)?;
        'next_recipe: for recipe in recipes {
            let mut tx = db.begin().await?;
            let insert = sqlx::query!(
                "INSERT INTO recipes (id, name, preparation, source) VALUES ($1, $2, $3, $4);",
                recipe.id,
                recipe.name,
                recipe.preparation,
                recipe.source
            )
            .execute(&mut *tx)
            .await;
            if let Err(err) = insert {
                eprintln!("failed to insert recipe: {}: {}", recipe.id, err);
                tx.rollback().await?;
                continue;
            }

            for ingredient in recipe.ingredients {
                let insert = sqlx::query!(
                    "INSERT INTO ingredients (recipe_id, ingredient) VALUES ($1, $2);",
                    recipe.id,
                    ingredient
                )
                .execute(&mut *tx)
                .await;
                if let Err(err) = insert {
                    eprintln!("failed to insert ingredient: {}: {}", recipe.id, err);
                    tx.rollback().await?;
                    continue 'next_recipe;
                }
            }

            for tag in recipe.tags {
                let insert = sqlx::query!(
                    "INSERT INTO tags (recipe_id, tag) VALUES ($1, $2);",
                    recipe.id,
                    tag
                )
                .execute(&mut *tx)
                .await;
                if let Err(err) = insert {
                    eprintln!("failed to insert tag: {}: {}", recipe.id, err);
                    tx.rollback().await?;
                    continue 'next_recipe;
                }
            }
        }
    }

    Ok(())
}

pub async fn fetch_recipe(_db: &mut SqlitePool) -> Option<Recipe> {
    None
}
