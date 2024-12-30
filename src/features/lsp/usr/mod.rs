mod routes;

use axum::Router;
use axum::routing::get;
use crate::features::common::title_server::APIFeature;
use routes::halo3::get_machine::get_machine;
use routes::halo3::get_recent_players::get_recent_players;
use routes::halo3::get_user::get_user;
use crate::features::lsp::usr::routes::halo_reach::get_reach_machine::get_reach_machine;
use crate::features::lsp::usr::routes::halo_reach::get_reach_recent_players::get_reach_recent_players;
use crate::features::lsp::usr::routes::halo_reach::get_reach_user::get_reach_user;

pub struct UserStorageServer {

}

impl APIFeature for UserStorageServer {
    fn get_router(&self) -> Router {
        Router::new()
            .route("/storage/user/{unk1}/{unk2}/{unk3}/{xuid}/recent_players.bin", get(get_recent_players))
            .route("/storage/user/{unk1}/{unk2}/{unk3}/{xuid}/user.bin", get(get_user))
            .route("/storage/machine/{unk1}/{unk2}/{unk3}/{machine_id}/machine.bin", get(get_machine))
            .route("/storage/user/{title}/{unk1}/{unk2}/{unk3}/{xuid}/recent_players.bin", get(get_reach_recent_players))
            .route("/storage/user/{title}/{unk1}/{unk2}/{unk3}/{xuid}/user.bin", get(get_reach_user))
            .route("/storage/machine/{title}/{unk1}/{unk2}/{unk3}/{machine_id}/machine.bin", get(get_reach_machine))
    }

    fn get_name(&self) -> &str {
        "LSP - usr"
    }
}

