use crate::*;
use axum::extract::Path;

pub async fn get_recipe_by_id(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(recipe_id): Path<String>,
) -> Result<response::Response, http::StatusCode> {
    let appstate = state.read().await;
    let id = recipe_id.parse::<i64>().unwrap(); // TODO:

    match database::fetch_recipe(&appstate.db, id).await {
        Ok(recipe) => Ok(recipe.into_response()),
        Err(e) => {
            log::error!("api failed to fetch recipe id={}, err={}", id, e);
            Err(http::StatusCode::NOT_FOUND)
        }
    }
}
