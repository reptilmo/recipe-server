use axum::{self, routing};
use tokio::net;
use tower_http::{services, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const DEFAULT_BIND_ADDR: &str = "127.0.0.1:8888";

async fn serv() -> Result<(), Box<dyn std::error::Error>> {
    let mime_favicon = "image/vnd.microsoft.icon".parse().unwrap();

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
        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO));

    // the server
    let app = axum::Router::new()
        .route("/", routing::get("Hello, World!"))
        .route_service(
            "/favicon.ico",
            services::ServeFile::new_with_mime("assets/static/favicon.ico", &mime_favicon),
        )
        .layer(trace_layer);

    println!(
        "recipe-service is listening on \x1b[91m{}\x1b[0m",
        DEFAULT_BIND_ADDR
    );

    let listener = net::TcpListener::bind(DEFAULT_BIND_ADDR).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = serv().await {
        eprintln!("recipe-server error: {}", err);
        std::process::exit(1);
    }
}
