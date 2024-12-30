use axum::response::IntoResponse;
use blf_lib::blf::BlfFile;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_machine_network_statistics, s_blf_chunk_player_data, s_blf_chunk_start_of_file};
use blf_lib::blf_file;
use crate::features::lsp::gameapi_reach::blf::s_blf_chunk_arena_hopper_stats::s_blf_chunk_arena_hopper_stats;
use crate::features::lsp::reach_presence_api::blf::s_blf_chunk_player_heartbeat_response::s_blf_chunk_player_heartbeat_response;

blf_file!(
    struct heartbeat_response_file {
        _blf: s_blf_chunk_start_of_file,
        phbr: s_blf_chunk_player_heartbeat_response,
        _eof: s_blf_chunk_end_of_file
    }
);

#[axum::debug_handler]
pub async fn get_heartbeat_response() -> impl IntoResponse {
    let mut arena = heartbeat_response_file {
        _blf: s_blf_chunk_start_of_file::default(),
        phbr: s_blf_chunk_player_heartbeat_response::default(),
        _eof: s_blf_chunk_end_of_file::default()
    };

    arena.write()
}