use axum::{self, routing};
use tokio::net;
use tower_http::{services, trace};

const DEFAULT_BIND_ADDR: &str = "127.0.0.1:8888";

async fn serv() -> Result<(), Box<dyn std::error::Error>> {
    let mime_favicon = "image/vnd.microsoft.icon".parse().unwrap();

    let app = axum::Router::new()
        .route("/", routing::get("Hello, World!"))
        .route_service(
            "/favicon.ico",
            services::ServeFile::new_with_mime("assets/static/favicon.ico", &mime_favicon),
        );

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
