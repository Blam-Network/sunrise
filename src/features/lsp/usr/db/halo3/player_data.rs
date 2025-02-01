use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::s_blf_chunk_player_data;
use blf_lib::types::c_string::StaticString;
use sqlx::{PgPool, Row};

pub async fn get_player_data(pool: &PgPool, xuid: u64) -> Result<s_blf_chunk_player_data, sqlx::Error> {
    let row = sqlx::query(
        r#"
        SELECT * FROM halo3.get_or_create_player_data($1)
        "#,
    )
        .bind(xuid as i64)
        .fetch_one(pool)
        .await?;

    let mut bungie_user_role = 0;

    // BNet players get the 7th column
    bungie_user_role += 1;
    if row.get::<bool, _>("is_pro") {
        bungie_user_role += 2;
    }
    if row.get::<bool, _>("is_bungie") {
        bungie_user_role += 4;
    }
    if row.get::<bool, _>("has_recon") || row.get::<bool, _>("road_to_recon_completed") {
        bungie_user_role += 8;
    }

    let player_data = s_blf_chunk_player_data {
        hopper_access: row.get::<i32, _>("hopper_access") as u32,
        highest_skill: row.get::<i32, _>("highest_skill") as u32,
        bungie_user_role,
        hopper_directory: StaticString::from_string(
            row.get::<Option<String>, _>("hopper_directory_override")
                .unwrap_or(String::from("default_hoppers")
            )
        ).unwrap(),
    };

    Ok(player_data)
}