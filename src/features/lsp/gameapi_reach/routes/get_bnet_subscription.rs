use axum::response::IntoResponse;
use blf_lib::blf::BlfFile;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_machine_network_statistics, s_blf_chunk_player_data, s_blf_chunk_start_of_file};
use blf_lib::blf_file;
use blf_lib::types::array::StaticArray;
use crate::features::lsp::gameapi_reach::blf::s_blf_chunk_daily_challenges::s_blf_chunk_daily_challenges;
use crate::features::lsp::gameapi_reach::blf::s_blf_chunk_rewards_persistance::s_blf_chunk_rewards_persistance;

#[axum::debug_handler]
pub async fn get_bnet_subscription() -> impl IntoResponse {
    String::from("Status: Suscribed")
}