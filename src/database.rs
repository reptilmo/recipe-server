use crate::error::RecipeServerError;
use crate::recipe::{Recipe, read_recipes_json};

use sqlx::{SqlitePool, migrate::MigrateDatabase, sqlite};
use tokio_stream::StreamExt;

pub fn get_uri(db_uri: Option<&str>) -> String {
    if let Some(db_uri) = db_uri {
        db_uri.to_string()
    } else if let Ok(db_uri) = std::env::var("RECIPE_SERVER_DB_URI") {
        db_uri
    } else {
        "sqlite://db/recipe-server.db".to_string()
    }
}

fn extract_dir(db_uri: &str) -> Result<&str, RecipeServerError> {
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

pub async fn init(
    init_from: Option<std::path::PathBuf>,
    db_uri: &str,
) -> Result<SqlitePool, RecipeServerError> {
    if !sqlite::Sqlite::database_exists(db_uri).await? {
        let db_dir = extract_dir(db_uri)?;
        std::fs::create_dir_all(db_dir)?;
        sqlite::Sqlite::create_database(db_uri).await?
    }

    let db = SqlitePool::connect(db_uri).await?;
    sqlx::migrate!().run(&db).await?;

    if let Some(path) = init_from {
        let recipes = read_recipes_json(path)?;
        'next_recipe: for recipe in recipes {
            let mut tx = db.begin().await?;
            let prep = recipe.preparation.join("|");
            let insert = sqlx::query!(
                "INSERT INTO recipes (id, name, preparation, source) VALUES ($1, $2, $3, $4);",
                recipe.id,
                recipe.name,
                prep,
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
            tx.commit().await?;
        }
    }

    Ok(db)
}

pub async fn random_recipe_id(db: &SqlitePool) -> Result<i64, RecipeServerError> {
    let recipe_id = sqlx::query_scalar!("SELECT id FROM recipes ORDER BY RANDOM() LIMIT 1;")
        .fetch_one(db)
        .await?;
    Ok(recipe_id)
}

pub async fn fetch_recipe(db: &SqlitePool, recipe_id: i64) -> Result<Recipe, RecipeServerError> {
    struct SqlRecipe {
        id: i64,
        name: String,
        preparation: String,
        source: String,
    }

    let sql_recipe = sqlx::query_as!(SqlRecipe, "SELECT * FROM recipes WHERE id = $1;", recipe_id)
        .fetch_one(db)
        .await?;

    let mut sql_ingredients = sqlx::query_scalar!(
        "SELECT ingredient FROM ingredients WHERE recipe_id = $1;",
        sql_recipe.id
    )
    .fetch(db);
    let mut ingredients: Vec<String> = Vec::new();
    while let Some(ingred) = sql_ingredients.next().await {
        match ingred {
            Ok(ingredient) => ingredients.push(ingredient),
            Err(e) => {
                log::error!(
                    "failed to fetch ingredients: recipe={}, error={}",
                    sql_recipe.id,
                    e
                );
                return Err(RecipeServerError::SqlxError(e));
            }
        }
    }

    let mut sql_tags =
        sqlx::query_scalar!("SELECT tag FROM tags WHERE recipe_id = $1;", sql_recipe.id).fetch(db);
    let mut tags: Vec<String> = Vec::new();
    while let Some(tag) = sql_tags.next().await {
        match tag {
            Ok(tag) => tags.push(tag),
            Err(e) => {
                log::error!(
                    "failed to fetch tags: recipe={}, error={}",
                    sql_recipe.id,
                    e
                );
                return Err(RecipeServerError::SqlxError(e));
            }
        }
    }

    Ok(Recipe {
        id: sql_recipe.id as u32,
        name: sql_recipe.name,
        ingredients,
        preparation: sql_recipe
            .preparation
            .split("|") //TODO: May be a bad choice of line separating character.
            .map(|s| s.to_string())
            .collect(),
        source: sql_recipe.source,
        tags,
    })
}
