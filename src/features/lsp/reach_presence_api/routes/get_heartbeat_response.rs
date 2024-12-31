use blf_lib::blf::versions::haloreach::v12065_11_08_24_1738_tu1actual::s_blf_chunk_player_heartbeat_response;
use axum::response::IntoResponse;
use blf_lib::blf::BlfFile;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_end_of_file, s_blf_chunk_start_of_file};
use blf_lib::blf_file;

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