mod opt;
mod model;

use warp::{Filter, trace, fs};
use tracing_subscriber::{fmt::format::FmtSpan};
use tracing;
use opt::{Config};
use clap::Clap;
use std::net::SocketAddr;

#[macro_use]
extern crate serde;

#[derive(Clap, Debug)]
#[clap(name = "warp-api-template", rename_all = "kebab-case", rename_all_env = "screaming-snake")]
pub struct Args {
    #[clap(short, long)]
    debug: bool,
    #[clap(default_value = "postgres://scorpio:zhuyan@localhost/chubby",short = 'D', long, env)]
    database_url: String,
    #[clap(default_value = "localhost:5678",short = 'R', long, env)]
    redis_url: String,
    #[clap(default_value = "0.0.0.0:8080", env)]
    host: SocketAddr,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv();
    let args: Args = Args::parse();
    let conf = Config::new(&args).await?;
    let conf = warp::any().map(move || conf.clone());
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());
    tracing_subscriber::fmt().with_env_filter(filter).with_span_events(FmtSpan::CLOSE).init();
    let root = warp::path::end().map(|| "hello world!");
    let hello = warp::path!("hello" / String)
        .map(|name| {
            tracing::info!("saying hello...");
            format!("Hello, {}!", name)
        });
    let assets = warp::path("static").and(fs::dir("./assets"));
    let route = warp::any().and(root.or(assets).or(hello)).with(trace::request());
    Ok(warp::serve(route).run((args.host.ip(), args.host.port())).await)
}