use crate::recipe::Recipe;
use crate::*;
use axum::extract::{Json, Path};
use utoipa_axum::routes;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "recipe-server", description = "A simple recipe server and API")
    )
)]
pub struct ApiDoc;

pub fn router() -> OpenApiRouter<Arc<RwLock<AppState>>> {
    OpenApiRouter::new()
        .routes(routes!(get_recipe_by_id))
        .routes(routes!(get_recipe_by_tag))
        .routes(routes!(get_recipe_random))
}

#[utoipa::path(
    get,
    path = "/recipe/{recipe_id}",
    responses(
        (status = 200, description = "Fetch a recipe by id", body = [Recipe]),
        (status = 404, description = "No matching recipe"),
        (status = 400, description = "User provided bad id"),
    )
)]
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

#[utoipa::path(
    get,
    path = "/recipe/with-tags",
    responses(
        (status = 200, description = "Fetch a joke with provided tag(s)", body = [Recipe]),
        (status = 404, description = "No matching recipe"),
    )
)]
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

#[utoipa::path(
    get,
    path = "/recipe/random",
    responses(
        (status = 200, description = "Fetch a random recipe", body = [Recipe]),
        (status = 404, description = "The database is empty"),
    )
)]
pub async fn get_recipe_random(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<response::Response, http::StatusCode> {
    let appstate = state.read().await;
    match database::random_recipe_id(&appstate.db).await {
        Ok(id) => match database::fetch_recipe(&appstate.db, id).await {
            Ok(recipe) => Ok(recipe.into_response()),
            Err(e) => {
                log::error!("api failed to fetch recipe id={}, err={}", id, e);
                Err(http::StatusCode::NOT_FOUND)
            }
        },
        Err(e) => {
            log::error!("api failed to fetch random recipe id, err={}", e);
            Err(http::StatusCode::NOT_FOUND) // Should this be 500?
        }
    }
}
