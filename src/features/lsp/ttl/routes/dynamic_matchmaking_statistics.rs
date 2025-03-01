use axum::{
    response::IntoResponse,
};
use binrw::BinWrite;
use blf_lib::blf::BlfFileBuilder;
use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{hopper_population, s_blf_chunk_author, s_blf_chunk_end_of_file, s_blf_chunk_matchmaking_hopper_statistics, s_blf_chunk_start_of_file};
use blf_lib::blf::versions::v12070_08_09_05_2031_halo3_ship;
use sqlx::Row;
use crate::features::common::database::get_connection_pool;

/// Endpoint to generate the image with predefined dots.
pub async fn generate_matchmaking_statistics() -> impl IntoResponse {
    let _blf = s_blf_chunk_start_of_file::default();
    let athr = s_blf_chunk_author::for_build::<v12070_08_09_05_2031_halo3_ship>();
    let mut mmhs = s_blf_chunk_matchmaking_hopper_statistics::default();
    let _eof = s_blf_chunk_end_of_file::default();

    let pool = get_connection_pool().await;
    let results = sqlx::query(
        r#"
        SELECT count(player_xuid), hopper_identifier
        FROM (
            SELECT DISTINCT ON (crp.player_xuid)
                crp.player_xuid,
                crmo.hopper_identifier
            FROM halo3.carnage_report_player crp
            INNER JOIN halo3.carnage_report cr ON cr.id = crp.carnage_report_id
            INNER JOIN halo3.carnage_report_matchmaking_options crmo ON crmo.id = crp.carnage_report_id
            WHERE cr.finish_time >= NOW() - INTERVAL '1 hour'
            ORDER BY crp.player_xuid, cr.finish_time DESC
        ) AS hopper_players
        GROUP BY hopper_players.hopper_identifier
        ORDER BY count(player_xuid) DESC
        LIMIT 32;
        "#
    )
        .fetch_all(pool)
        .await;

    let mut data = mmhs.data.get_mut();
    for (index, row) in results.unwrap().iter().enumerate() {
        if index >= data.len() { break; }

        let player_count: i64 = row.try_get(0).unwrap();
        let hopper_identifier: i32 = row.try_get(1).unwrap();

        data[index] = hopper_population {
            player_count: player_count as u32,
            hopper_identifier: hopper_identifier as u32,
        };

        mmhs.player_count += player_count as u32;
    }

    BlfFileBuilder::new()
        .add_chunk(_blf)
        .add_chunk(athr)
        .add_chunk(mmhs)
        .add_chunk(_eof)
        .write()
}