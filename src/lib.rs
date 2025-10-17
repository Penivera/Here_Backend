pub mod routes;
pub mod schemas;
pub mod utils;
pub mod core;

use routes::create_routes;
use axum::Router;
use tokio::{net::TcpListener};

pub async fn run(){
    let app: Router= create_routes();
    let listener =TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}