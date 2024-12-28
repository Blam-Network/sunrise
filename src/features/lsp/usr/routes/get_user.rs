use axum::extract::Path;
use axum::response::IntoResponse;
use blf_lib::blf::BlfFile;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_player_data, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;
use blf_lib::types::byte_order_mark::byte_order_mark;

blf_file!(
    struct user_file {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        fupd: s_blf_chunk_player_data,
        // srid
        // fubh
        // filq
        _eof: s_blf_chunk_end_of_file
    }
);

#[axum::debug_handler]
pub async fn get_user(Path((_, _, _, xuid)): Path<(String, String, String, String)>) -> impl IntoResponse {
    let mut user = user_file {
        _blf: s_blf_chunk_start_of_file::new("halo3 user", byte_order_mark::default()),
        athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
        fupd: s_blf_chunk_player_data::default(),
        _eof: s_blf_chunk_end_of_file::default()
    };

    user.write()
}