mod database;
mod error;
mod recipe;
mod templates;

use crate::templates::IndexTemplate;

use axum::{self, extract::State, response, routing};
use clap::Parser;
use sqlx::SqlitePool;
use tokio::{net, sync::RwLock};
use tower_http::{services, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

struct AppState {
    db: SqlitePool,
}

async fn server_response(State(state): State<Arc<RwLock<AppState>>>) -> response::Html<String> {
    let appstate = state.read().await;
    let recipe = database::fetch_recipe(&appstate.db).await;

    match recipe {
        Ok(recipe) => {
            let recipe = IndexTemplate::recipe(recipe);
            response::Html(recipe.to_string())
        }
        Err(_) => response::Html("Internal Error".to_string()), //TODO:
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

    // the server
    let app = axum::Router::new()
        .route("/", routing::get(server_response))
        .route_service(
            "/recipe.css",
            services::ServeFile::new_with_mime("assets/static/recipe.css", &mime::TEXT_CSS_UTF_8),
        )
        .route_service(
            "/favicon.ico",
            services::ServeFile::new_with_mime("assets/static/favicon.ico", &mime_favicon),
        )
        .layer(trace_layer)
        .with_state(state);

    println!("recipe-service is listening on \x1b[91m{}\x1b[0m", addr);

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
