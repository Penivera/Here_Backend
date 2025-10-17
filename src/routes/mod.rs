use axum::{routing::get, Router};
pub mod auth;

pub fn create_routes()->Router<()>{
    return Router::new().route("/",get(||async {"Hello world!!"}));
}