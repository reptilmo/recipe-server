mod database;
mod error;
mod recipe;
mod templates;

use crate::database::*;
use crate::recipe::get_recipe;
use crate::templates::IndexTemplate;

use axum::{self, response, routing};
use clap::Parser;
use tokio::net;
use tower_http::{services, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

async fn response_recipe() -> response::Html<String> {
    let recipe = IndexTemplate::recipe(get_recipe());
    response::Html(recipe.to_string())
}

async fn serve(bind_addr: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mime_favicon = "image/vnd.microsoft.icon".parse().unwrap();
    let addr = match bind_addr {
        Some(addr) => addr,
        None => DEFAULT_BIND_ADDR.to_string(),
    };

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
        .route("/", routing::get(response_recipe))
        .route_service(
            "/recipe.css",
            services::ServeFile::new_with_mime("assets/static/recipe.css", &mime::TEXT_CSS_UTF_8),
        )
        .route_service(
            "/favicon.ico",
            services::ServeFile::new_with_mime("assets/static/favicon.ico", &mime_favicon),
        )
        .layer(trace_layer);

    println!("recipe-service is listening on \x1b[91m{}\x1b[0m", addr);

    let listener = net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let db_uri = get_database_uri(args.db_uri.as_deref());
    if let Err(err) = init_database(args.init_from, &db_uri).await {
        eprintln!("recipe-server error: {}", err);
        std::process::exit(1);
    }
    if let Err(err) = serve(args.bind_addr).await {
        eprintln!("recipe-server error: {}", err);
        std::process::exit(1);
    }
}
