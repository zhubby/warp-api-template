use warp::{Filter, trace,fs};
use tracing_subscriber::{fmt::format::FmtSpan};
use tracing;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    let assets = warp::path("static").and(fs::dir("../assets"));

    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let root = warp::path::end().map(|| "hello world!");
    let hello = warp::path!("hello" / String)
        .map(|name| {
            tracing::info!("saying hello...");
            format!("Hello, {}!", name)
        });


    let route = warp::get().and(root.or(assets).or(hello)).with(trace::request());
    warp::serve(route)
        .run(([0, 0, 0, 0], 8080))
        .await;
}