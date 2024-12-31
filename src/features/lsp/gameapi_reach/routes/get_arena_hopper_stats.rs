use axum::response::IntoResponse;
use blf_lib::blf::BlfFile;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_end_of_file;
use blf_lib::blf::versions::haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_arena_hopper_stats;
use blf_lib::blf_file;

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