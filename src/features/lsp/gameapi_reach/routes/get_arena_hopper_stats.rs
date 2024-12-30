use axum::response::IntoResponse;
use blf_lib::blf::BlfFile;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_machine_network_statistics, s_blf_chunk_player_data, s_blf_chunk_start_of_file};
use blf_lib::blf_file;
use crate::features::lsp::gameapi_reach::blf::s_blf_chunk_arena_hopper_stats::s_blf_chunk_arena_hopper_stats;

blf_file!(
    struct arena_stats_file {
        arhs: s_blf_chunk_arena_hopper_stats,
        _eof: s_blf_chunk_end_of_file
    }
);

#[axum::debug_handler]
pub async fn get_arena_hopper_stats() -> impl IntoResponse {
    let mut arena = arena_stats_file {
        arhs: s_blf_chunk_arena_hopper_stats::default(),
        _eof: s_blf_chunk_end_of_file::default()
    };

    arena.write()
}