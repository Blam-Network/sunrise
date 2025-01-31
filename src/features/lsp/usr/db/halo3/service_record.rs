use blf_lib::blf::versions::halo3::v12070_08_09_05_2031_halo3_ship::{s_blf_chunk_service_record, Color, EliteArmour, Grade, PlayerModel, Rank, SpartanBody, SpartanHelmet, SpartanShoulder};
use blf_lib::types::c_string::StaticWcharString;
use num::FromPrimitive;
use sqlx::{query, PgPool, Row};

pub async fn get_service_record_by_xuid(pool: &PgPool, player_xuid: u64) -> Result<s_blf_chunk_service_record, sqlx::Error> {
    let row = query(
        "SELECT * FROM halo3.service_record WHERE player_xuid = $1"
    )
        .bind(player_xuid as i64)
        .fetch_one(pool)
        .await?;

    let service_record = s_blf_chunk_service_record {
        player_name: StaticWcharString::from_string(&row.get::<String, _>("player_name")).unwrap(),
        appearance_flags: row.get::<i16, _>("appearance_flags") as u8,
        primary_color: FromPrimitive::from_i16(row.get::<i16, _>("primary_color")).unwrap(),
        secondary_color: FromPrimitive::from_i16(row.get::<i16, _>("secondary_color")).unwrap(),
        tertiary_color: FromPrimitive::from_i16(row.get::<i16, _>("tertiary_color")).unwrap(),
        is_elite: FromPrimitive::from_i16(row.get::<i16, _>("is_elite")).unwrap(),
        foreground_emblem: row.get::<i16, _>("foreground_emblem") as u8,
        background_emblem: row.get::<i16, _>("background_emblem") as u8,
        emblem_flags: row.get::<i16, _>("emblem_flags") as u8,
        emblem_primary_color: FromPrimitive::from_i16(row.get::<i16, _>("emblem_primary_color")).unwrap(),
        emblem_secondary_color: FromPrimitive::from_i16(row.get::<i16, _>("emblem_secondary_color")).unwrap(),
        emblem_background_color: FromPrimitive::from_i16(row.get::<i16, _>("emblem_background_color")).unwrap(),
        spartan_helmet: FromPrimitive::from_i16(row.get::<i16, _>("spartan_helmet")).unwrap(),
        spartan_left_shoulder: FromPrimitive::from_i16(row.get::<i16, _>("spartan_left_shoulder")).unwrap(),
        spartan_right_shoulder: FromPrimitive::from_i16(row.get::<i16, _>("spartan_right_shoulder")).unwrap(),
        spartan_body: FromPrimitive::from_i16(row.get::<i16, _>("spartan_body")).unwrap(),
        elite_helmet: FromPrimitive::from_i16(row.get::<i16, _>("elite_helmet")).unwrap(),
        elite_left_shoulder: FromPrimitive::from_i16(row.get::<i16, _>("elite_left_shoulder")).unwrap(),
        elite_right_shoulder: FromPrimitive::from_i16(row.get::<i16, _>("elite_right_shoulder")).unwrap(),
        elite_body: FromPrimitive::from_i16(row.get::<i16, _>("elite_body")).unwrap(),
        service_tag: StaticWcharString::from_string(&row.get::<String, _>("service_tag")).unwrap(),
        campaign_progress: row.get::<i32, _>("campaign_progress"),
        highest_skill: row.get::<i32, _>("highest_skill"),
        total_exp: row.get::<i32, _>("total_exp"),
        unknown_insignia: row.get::<i32, _>("unknown_insignia"),
        rank: FromPrimitive::from_i32(row.get::<i32, _>("rank")).unwrap(),
        grade: FromPrimitive::from_i32(row.get::<i32, _>("grade")).unwrap(),
        unknown_insignia2: row.get::<i32, _>("unknown_insignia2"),
    };
    Ok(service_record)
}