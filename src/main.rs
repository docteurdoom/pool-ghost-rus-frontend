#[macro_use] extern crate log;
pub const CRATE_NAME: &str = module_path!();
pub const JSON_PATH: &str = "ghost.json";
mod serve;
mod logger;
mod pages;
mod parse;

#[tokio::main]
async fn main() {
    use futures::future;
    logger::init();

    future::join(
        serve::http(),
        serve::https()
    ).await;
}


