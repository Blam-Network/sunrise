use axum::extract::Path;
use axum::response::IntoResponse;
use blf_lib::blf::{BlfFile, BlfFileBuilder};
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_player_data, s_blf_chunk_service_record, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use blf_lib::blf_file;
use blf_lib::types::byte_order_mark::byte_order_mark;
use crate::features::common::database::{get_connection_pool, try_get_connection_pool};
use crate::features::lsp::usr::db::halo3::player_data::get_player_data;
use crate::features::lsp::usr::db::halo3::service_record::get_service_record_by_xuid;

#[axum::debug_handler]
pub async fn get_user(Path((_, _, _, xuid)): Path<(String, String, String, String)>) -> impl IntoResponse {
    let player_xuid = u64::from_str_radix(&xuid, 16).unwrap();
    let pool = try_get_connection_pool().await;

    let mut blf_file_builder = BlfFileBuilder::new();
        blf_file_builder
            .add_chunk(s_blf_chunk_start_of_file::new("halo3 user", byte_order_mark::default()))
            .add_chunk(s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>());

    // If we can't connect to the database, return a generic user file.
    if pool.is_err() {
        blf_file_builder.add_chunk(s_blf_chunk_player_data::default());
    } else {
        let pool = pool.unwrap();

        if let Ok(fupd) = get_player_data(&pool, player_xuid).await {
            blf_file_builder.add_chunk(fupd);
        } else {
            blf_file_builder.add_chunk(s_blf_chunk_player_data::default());
        }

        if let Ok(srid) = get_service_record_by_xuid(&pool, player_xuid).await {
            blf_file_builder.add_chunk(srid);
        }
    }

    blf_file_builder
        .add_chunk(s_blf_chunk_end_of_file::default())
        .write()
}