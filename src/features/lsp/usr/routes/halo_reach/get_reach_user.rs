use axum::extract::Path;
use axum::response::IntoResponse;
use blf_lib::blf::BlfFile;
use blf_lib::blf::versions::haloreach::v12065_11_08_24_1738_tu1actual::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_player_data, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;
use blf_lib::types::byte_order_mark::byte_order_mark;

blf_file!(
    struct user_file {
        _blf: s_blf_chunk_start_of_file,
        fupd: s_blf_chunk_player_data,
        // srid
        // fubh
        // filq
        _eof: s_blf_chunk_end_of_file
    }
);

#[axum::debug_handler]
pub async fn get_reach_user(Path((title, _, _, _, xuid)): Path<(String, String, String, String, String)>) -> impl IntoResponse {
    let mut user = user_file {
        _blf: s_blf_chunk_start_of_file::new("omaha user", byte_order_mark::default()),
        fupd: s_blf_chunk_player_data::default(),
        _eof: s_blf_chunk_end_of_file::default()
    };

    user.write()
}