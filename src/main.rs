mod api;
mod database;
mod error;
mod recipe;
mod templates;

use crate::templates::IndexTemplate;

use axum::{
    self,
    extract::{Query, State},
    http,
    response::{self, IntoResponse},
    routing,
};
use clap::Parser;
use serde::Deserialize;
use sqlx::SqlitePool;
use tokio::{net, sync::RwLock};
use tower_http::{services, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::{OpenApi, ToSchema};
use utoipa_axum::router::OpenApiRouter;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use std::sync::Arc;

const DEFAULT_BIND_ADDR: &str = "127.0.0.1:8888";

#[derive(Parser)]
struct Args {
    #[arg(short, long, name = "bind-addr")]
    bind_addr: Option<String>,
    #[arg(short, long, name = "init-from")]
    init_from: Option<std::path::PathBuf>,
    #[arg(short, long, name = "db-uri")]
    db_uri: Option<String>,
}

#[derive(Deserialize)]
struct WebQueryParams {
    id: Option<i64>,
    tags: Option<String>,
}

struct AppState {
    db: SqlitePool,
}

async fn web_response(
    State(state): State<Arc<RwLock<AppState>>>,
    Query(params): Query<WebQueryParams>,
) -> Result<response::Response, http::StatusCode> {
    let appstate = state.read().await;

    // Got a recipe id.
    if let WebQueryParams { id: Some(id), .. } = params {
        let result = match database::fetch_recipe(&appstate.db, id).await {
            Ok(recipe) => {
                let recipe = IndexTemplate::recipe(recipe);
                Ok(response::Html(recipe.to_string()).into_response())
            }
            Err(e) => {
                log::error!("failed to fetch recipe, err={}", e);
                Err(http::StatusCode::NOT_FOUND)
            }
        };
        return result;
    }

    // Got tag(s).
    if let WebQueryParams {
        tags: Some(tags), ..
    } = params
    {
        if !tags.is_empty() {
            let tags_vec = tags
                .split(",")
                .map(|t| t.to_string().trim_start().to_lowercase())
                .collect();
            match database::fetch_recipe_id(&appstate.db, tags_vec).await {
                Ok(Some(id)) => {
                    let uri = format!("/?id={}", id);
                    return Ok(response::Redirect::to(&uri).into_response());
                }
                Ok(None) => {
                    log::info!("no data for tags={:?}", tags);
                }
                Err(e) => {
                    log::error!("{}", e);
                }
            };
        }
    }

    // Got nothing.
    match database::random_recipe_id(&appstate.db).await {
        Ok(id) => {
            let uri = format!("/?id={}", id);
            Ok(response::Redirect::to(&uri).into_response())
        }
        Err(e) => {
            log::error!("failed to fetch random id, err={}", e);
            Err(http::StatusCode::NO_CONTENT)
        }
    }
}

async fn serve(
    db: SqlitePool,
    bind_addr: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mime_favicon = "image/vnd.microsoft.icon".parse().unwrap();
    let addr = match bind_addr {
        Some(addr) => addr,
        None => DEFAULT_BIND_ADDR.to_string(),
    };
    let state = Arc::new(RwLock::new(AppState { db }));

    // tracing registry and layer
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "recipe-server=debug,info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let trace_layer = trace::TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        //.on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO));

    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods([http::Method::GET])
        .allow_origin(tower_http::cors::Any);

    let (api_router, api) = OpenApiRouter::with_openapi(api::ApiDoc::openapi())
        .nest("/api/v1", api::router())
        .split_for_parts();

    let redoc_ui = Redoc::with_url("/redoc", api.clone());
    let swagger_ui = SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api);

    // the server
    let app = axum::Router::new()
        .route("/", routing::get(web_response))
        .route_service(
            "/recipe.css",
            services::ServeFile::new_with_mime("assets/static/recipe.css", &mime::TEXT_CSS_UTF_8),
        )
        .route_service(
            "/favicon.ico",
            services::ServeFile::new_with_mime("assets/static/favicon.ico", &mime_favicon),
        )
        .merge(redoc_ui)
        .merge(swagger_ui)
        .merge(api_router)
        .layer(cors)
        .layer(trace_layer)
        .with_state(state);

    println!(
        "\x1b[93mrecipe-server\x1b[0m is listening on \x1b[91m{}\x1b[0m",
        addr
    );

    let listener = net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let db_uri = database::get_uri(args.db_uri.as_deref());
    let db = match database::init(args.init_from, &db_uri).await {
        Ok(db) => db,
        Err(err) => {
            eprintln!("recipe-server error: {}", err);
            std::process::exit(1);
        }
    };
    if let Err(err) = serve(db, args.bind_addr).await {
        eprintln!("recipe-server error: {}", err);
        std::process::exit(1);
    }
}
