mod routes;
pub mod blf;

use axum::Router;
use axum::routing::{get, post};
use crate::features::common::title_server::APIFeature;
use crate::features::lsp::reach_presence_api::routes::get_heartbeat_response::get_heartbeat_response;

pub struct ReachPresenceAPI {

}

impl APIFeature for ReachPresenceAPI {
    fn get_router(&self) -> Router {
        Router::new()
            .route("/ReachPresenceApi/heartbeat.ashx", post(get_heartbeat_response))
    }

    fn get_name(&self) -> &str {
        "LSP - Reach Presence API"
    }
}

