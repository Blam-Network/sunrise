mod routes;
use axum::Router;
use axum::routing::{get, post};
use crate::features::common::title_server::APIFeature;
use crate::features::lsp::gameapi_reach::routes::get_arena_hopper_stats::get_arena_hopper_stats;
use crate::features::lsp::gameapi_reach::routes::get_bnet_subscription::get_bnet_subscription;
use crate::features::lsp::gameapi_reach::routes::get_rewards_file::get_rewards_file;

pub struct GameAPIReach {

}

impl APIFeature for GameAPIReach {
    fn get_router(&self) -> Router {
        Router::new()
            .route("/gameapi_omaha/ArenaGetSeasonStats.ashx", get(get_arena_hopper_stats))
            .route("/gameapi_omaha/UserUpdateRewards.ashx", post(get_rewards_file))
            .route("/gameapi_omaha/UserGetBnetSubscription.ashx", get(get_bnet_subscription))
    }

    fn get_name(&self) -> &str {
        "LSP - gameapi_omaha"
    }
}

