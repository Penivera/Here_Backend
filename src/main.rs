pub mod routes;
pub mod schemas;
pub mod core;
pub mod utils;
use here::run;
use dotenv::dotenv;
#[allow(unused_imports)]
use core::configs::{CONFIG,init_logging};

#[tokio::main]
async fn main() {
    init_logging();
    dotenv().ok();
    run().await
}
