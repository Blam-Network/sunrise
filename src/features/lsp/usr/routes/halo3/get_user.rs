use axum::extract::Path;
use axum::response::IntoResponse;
use blf_lib::blf::BlfFile;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_player_data, s_blf_chunk_service_record, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;
use blf_lib::types::byte_order_mark::byte_order_mark;
use crate::features::common::database::{get_connection_pool, try_get_connection_pool};
use crate::features::lsp::usr::db::halo3::player_data::get_player_data;
use crate::features::lsp::usr::db::halo3::service_record::get_service_record_by_xuid;

blf_file!(
    struct user_file {
        _blf: s_blf_chunk_start_of_file,
        athr: s_blf_chunk_author,
        fupd: s_blf_chunk_player_data,
        srid: s_blf_chunk_service_record,
        // fubh
        // filq
        _eof: s_blf_chunk_end_of_file
    }
);

#[axum::debug_handler]
pub async fn get_user(Path((_, _, _, xuid)): Path<(String, String, String, String)>) -> impl IntoResponse {
    // Parse xuid from hex string to a u64 number
    let player_xuid = u64::from_str_radix(&xuid, 16).unwrap();

    let pool = try_get_connection_pool().await;

    // If we can't connect to the database, return a generic user file.
    if pool.is_err() {
        let mut user = user_file {
            _blf: s_blf_chunk_start_of_file::new("halo3 user", byte_order_mark::default()),
            athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
            fupd: s_blf_chunk_player_data::default(),
            srid: s_blf_chunk_service_record::default(),
            _eof: s_blf_chunk_end_of_file::default()
        };

        user.write()
    } else {
        let pool = pool.unwrap();
        let mut user = user_file {
            _blf: s_blf_chunk_start_of_file::new("halo3 user", byte_order_mark::default()),
            athr: s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>(),
            fupd: get_player_data(&pool, player_xuid).await.unwrap_or(s_blf_chunk_player_data::default()),
            srid: get_service_record_by_xuid(&pool, player_xuid).await.unwrap_or(s_blf_chunk_service_record::default()),
            _eof: s_blf_chunk_end_of_file::default()
        };

        user.write()
    }
}