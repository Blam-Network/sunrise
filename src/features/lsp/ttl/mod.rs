mod routes;

use axum::Router;
use axum::routing::{get, get_service};
use crate::features::common::title_server::APIFeature;
use tower_http::services::ServeDir;
use crate::features::lsp::ttl::routes::dynamic_matchmaking_nightmap::generate_image;

pub struct TitleStorageServer {

}

impl APIFeature for TitleStorageServer {
    fn get_router(&self) -> Router {
        Router::new()
            .route("/storage/title/tracked/{title_version}/{hoppers_directory}/dynamic_matchmaking_nightmap.jpg", get(generate_image))
            .nest_service("/storage/title", get_service(ServeDir::new("./title_storage")))
    }

    fn get_name(&self) -> &str {
        "LSP - ttl (Title Storage)"
    }
}

