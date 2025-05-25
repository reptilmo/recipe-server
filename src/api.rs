use crate::*;
use axum::extract::{Json, Path};

pub async fn get_recipe_by_id(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(recipe_id): Path<String>,
) -> Result<response::Response, http::StatusCode> {
    let appstate = state.read().await;
    if let Ok(id) = recipe_id.parse::<i64>() {
        match database::fetch_recipe(&appstate.db, id).await {
            Ok(recipe) => Ok(recipe.into_response()),
            Err(e) => {
                log::error!("api failed to fetch recipe id={}, err={}", id, e);
                Err(http::StatusCode::NOT_FOUND)
            }
        }
    } else {
        log::error!("api failed to parse recipe id={}", recipe_id);
        Err(http::StatusCode::BAD_REQUEST)
    }
}

pub async fn get_recipe_by_tag(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(tags): Json<Vec<String>>,
) -> Result<response::Response, http::StatusCode> {
    log::info!("tags: {:?}", tags);
    let appstate = state.read().await;
    match database::fetch_recipe_id(&appstate.db, tags).await {
        Ok(Some(id)) => match database::fetch_recipe(&appstate.db, id).await {
            Ok(recipe) => Ok(recipe.into_response()),
            Err(e) => {
                log::error!("api failed to fetch recipe id={}, err={}", id, e);
                Err(http::StatusCode::NOT_FOUND)
            }
        },
        Ok(None) => Err(http::StatusCode::NOT_FOUND),
        Err(e) => {
            log::error!("api failed to fetch recipe id by tag, err={}", e);
            Err(http::StatusCode::NOT_FOUND) // Should this be 500?
        }
    }
}
