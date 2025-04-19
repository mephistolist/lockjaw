mod config;
mod db;
mod utils;
mod spider;
use crate::config::Config;
use crate::spider::run_crawl;
#[tokio::main]
async fn main() {
    let config = Config::parse();
    if let Err(err) = run_crawl(config).await {
        eprintln!("Error: {}", err);
    }
}
