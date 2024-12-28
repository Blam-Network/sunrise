use axum::Router;
use crate::features::common::title_server::APIFeature;

pub struct TitleStorageServer {

}

impl APIFeature for TitleStorageServer {
    fn get_router(&self) -> Router {
        Router::new()
    }
}