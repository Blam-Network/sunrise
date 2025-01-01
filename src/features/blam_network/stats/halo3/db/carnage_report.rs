use serde::Serialize;
use sqlx::{FromRow, Error};
use uuid::Uuid;
use crate::features::common::database::get_connection_pool;

#[derive(Debug, Serialize, FromRow)]
pub struct CarnageReportPlayer {
    pub player_name: String,
    pub highest_skill: i64,
    pub rank: i32,
    pub grade: i32,
    pub place: i32,
    pub score: i16,
    pub primary_color: i32,
    pub secondary_color: i32,
    pub tertiary_color: i32,
    pub emblem_primary_color: i32,
    pub emblem_secondary_color: i32,
    pub emblem_background_color: i32,
    pub foreground_emblem: i32,
    pub background_emblem: i32,
    pub service_tag: String,
    pub player_team: i32,
}

#[derive(Debug, Serialize, FromRow)]
pub struct CarnageReportTeam {
    pub team_index: i32,
    pub place: i32,
    pub score: i16,
}


pub async fn get_player_stats(
    carnage_report_id: Uuid,
) -> Result<Vec<CarnageReportPlayer>, Error> {
    let query = r#"
        SELECT crp.player_name,
               crp.global_statistics_highest_skill as highest_skill,
               crp.host_stats_global_rank as rank,
               crp.host_stats_global_grade as grade,
               crp.standing + 1 as place,
               crp.score,
               crp.primary_color,
               crp.secondary_color,
               crp.tertiary_color,
               crp.emblem_primary_color,
               crp.emblem_secondary_color,
               crp.emblem_background_color,
               crp.foreground_emblem,
               crp.background_emblem,
               crp.service_tag,
               crp.player_team
        FROM halo3.carnage_report_player crp
        WHERE crp.carnage_report_id = $1;
    "#;

    let players = sqlx::query_as::<_, CarnageReportPlayer>(query)
        .bind(carnage_report_id)
        .fetch_all(get_connection_pool().await)
        .await?;

    Ok(players)
}

pub async fn get_team_stats(
    carnage_report_id: Uuid,
) -> Result<Vec<CarnageReportTeam>, Error> {
    let query = r#"
        SELECT crt.team_index,
               crt.standing + 1 as place,
               crt.score
        FROM halo3.carnage_report_team crt
        WHERE crt.carnage_report_id = $1;
    "#;

    let teams = sqlx::query_as::<_, CarnageReportTeam>(query)
        .bind(carnage_report_id)
        .fetch_all(get_connection_pool().await)
        .await.unwrap();

    Ok(teams)
}

#[derive(Debug, FromRow)]
pub struct CarnageReport {
    pub team_game: bool,
    pub start_time: chrono::NaiveDateTime,
    pub finish_time: chrono::NaiveDateTime,
    pub game_variant_name: Option<String>,
    pub map_variant_name: Option<String>,
    pub map_id: Option<i32>,
    pub hopper_name: Option<String>,
    pub game_engine: i16,
    pub file_type: i32,
    pub duration: String,
}

pub async fn fetch_carnage_report(carnage_report_id: Uuid) -> Result<CarnageReport, Error> {
    let query = r#"
        SELECT
            cr.team_game,
            cr.start_time,
            cr.finish_time,
            crgv.name AS game_variant_name,
            cr.map_variant_name,
            cr.map_id,
            crmo.hopper_name,
            crgv.game_engine,
            crgv.file_type,
            TO_CHAR(AGE(cr.finish_time, cr.start_time), 'HH24:MI:SS') AS duration
        FROM
            halo3.carnage_report cr
        LEFT JOIN halo3.carnage_report_matchmaking_options crmo ON cr.id = crmo.id
        JOIN halo3.carnage_report_game_variant crgv ON cr.id = crgv.id
        WHERE cr.id = $1
    "#;

    let report = sqlx::query_as::<_, CarnageReport>(query)
        .bind(carnage_report_id)
        .fetch_one(get_connection_pool().await)
        .await?;

    Ok(report)
}

#[derive(Serialize, FromRow)]
pub struct KillEvent {
    pub killer: String,
    pub killed: String,
    pub time: i64,
    pub kill_type: i32,
}

pub async fn get_kill_events(carnage_report_id: Uuid) -> Result<Vec<KillEvent>, Error> {
    let query = r#"
        SELECT
            crp.player_name AS killer,
            crp2.player_name AS killed,
            crek."time",
            crek.kill_type
        FROM
            halo3.carnage_report_event_kill crek
        LEFT JOIN
            halo3.carnage_report_player crp
            ON crp.carnage_report_id = crek.carnage_report_id AND killer_player_index = crp.player_index
        LEFT JOIN
            halo3.carnage_report_player crp2
            ON crp2.carnage_report_id = crek.carnage_report_id AND dead_player_index = crp2.player_index
        WHERE
            crek.carnage_report_id = $1;
    "#;

    // Execute the query and map the results to KillEvent struct
    let rows = sqlx::query_as::<_, KillEvent>(query)
        .bind(carnage_report_id)
        .fetch_all(get_connection_pool().await)
        .await.unwrap();

    Ok(rows)
}
