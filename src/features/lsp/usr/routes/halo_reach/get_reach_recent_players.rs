use axum::extract::Path;
use axum::Json;
use axum::response::IntoResponse;
use blf_lib::blf::BlfFile;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_recent_players, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;
use blf_lib::types::byte_order_mark::byte_order_mark;
use uuid::Uuid;

blf_file!(
    struct recent_players {
        // not sure if this file actually needs the _blf or athr
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        furp: s_blf_chunk_recent_players,
        _eof: s_blf_chunk_end_of_file
    }
);

#[axum::debug_handler]
pub async fn get_reach_recent_players(Path((title, _, _, _, xuid)): Path<(String, String, String, String, String)>) -> impl IntoResponse {
    let mut recent_players = recent_players {
        _blf: s_blf_chunk_start_of_file::new("omaha recent players", byte_order_mark::default()),
        athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
        furp: s_blf_chunk_recent_players::create(),
        _eof: s_blf_chunk_end_of_file::default()
    };

    recent_players.write()
}