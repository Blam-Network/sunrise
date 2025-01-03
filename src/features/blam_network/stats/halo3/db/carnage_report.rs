use serde::Serialize;
use sqlx::{FromRow, PgPool, Transaction, Error, PgConnection};
use uuid::Uuid;
use chrono::NaiveDateTime;
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
    pub games_played: i16,
    pub games_completed: i16,
    pub games_won: i16,
    pub games_tied: i16,
    pub rounds_completed: i16,
    pub rounds_won: i16,
    pub in_round_score: i16,
    pub in_game_total_score: i16,
    pub kills: i16,
    pub assists: i16,
    pub deaths: i16,
    pub betrayals: i16,
    pub suicides: i16,
    pub most_kills_in_a_row: i16,
    pub seconds_alive: i16,
    pub ctf_flag_scores: i16,
    pub ctf_flag_grabs: i16,
    pub ctf_flag_carrier_kills: i16,
    pub ctf_flag_returns: i16,
    pub assault_bomb_arms: i16,
    pub assault_bomb_grabs: i16,
    pub assault_bomb_disarms: i16,
    pub assault_bomb_detonations: i16,
    pub oddball_time_with_ball: i16,
    pub oddball_unused: i16,
    pub oddball_kills_as_carrier: i16,
    pub oddball_ball_carrier_kills: i16,
    pub king_time_on_hill: i16,
    pub king_total_control_time: i16,
    pub king_unused0: i16,
    pub king_unused1: i16,
    pub unused0: i16,
    pub unused1: i16,
    pub unused2: i16,
    pub vip_takedowns: i16,
    pub vip_kills_as_vip: i16,
    pub vip_guard_time: i16,
    pub vip_time_as_vip: i16,
    pub vip_lives_as_vip: i16,
    pub juggernaut_kills: i16,
    pub juggernaut_kills_as_juggernaut: i16,
    pub juggernaut_total_control_time: i16,
    pub total_wp: i16,
    pub juggernaut_unused: i16,
    pub territories_owned: i16,
    pub territories_captures: i16,
    pub territories_ousts: i16,
    pub territories_time_in_territory: i16,
    pub infection_zombie_kills: i16,
    pub infection_infections: i16,
    pub infection_time_as_human: i16,
}


#[derive(Debug, Serialize, FromRow)]
pub struct CarnageReportTeam {
    pub team_index: i32,
    pub place: i32,
    pub score: i16,
}

#[derive(Debug, FromRow)]
pub struct CarnageReport {
    pub team_game: bool,
    pub start_time: NaiveDateTime,
    pub finish_time: NaiveDateTime,
    pub game_variant_name: Option<String>,
    pub map_variant_name: Option<String>,
    pub map_id: Option<i32>,
    pub hopper_name: Option<String>,
    pub game_engine: i16,
    pub file_type: i32,
    pub duration: String,
}

#[derive(Serialize, FromRow)]
pub struct KillEvent {
    pub killer: String,
    pub killed: String,
    pub time: i64,
    pub kill_type: i32,
}

// Function to fetch carnage report
pub async fn fetch_carnage_report(
    carnage_report_id: Uuid,
    transaction: &mut PgConnection,
) -> Result<CarnageReport, Error> {
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

    sqlx::query_as::<_, CarnageReport>(query)
        .bind(carnage_report_id)
        .fetch_one(&mut *transaction)
        .await
}

// Function to fetch player stats
pub async fn get_player_stats(
    carnage_report_id: Uuid,
    transaction: &mut PgConnection,
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
               crp.player_team,
               crps.*
        FROM halo3.carnage_report_player crp
        LEFT JOIN halo3.carnage_report_player_statistics crps
            ON crp.player_index = crps.player_index
            AND crp.carnage_report_id = crps.carnage_report_id
        WHERE crp.carnage_report_id = $1;
    "#;

    sqlx::query_as::<_, CarnageReportPlayer>(query)
        .bind(carnage_report_id)
        .fetch_all(transaction)
        .await
}

// Function to fetch team stats
pub async fn get_team_stats(
    carnage_report_id: Uuid,
    transaction: &mut PgConnection,
) -> Result<Vec<CarnageReportTeam>, Error> {
    let query = r#"
        SELECT crt.team_index,
               crt.standing + 1 as place,
               crt.score
        FROM halo3.carnage_report_team crt
        WHERE crt.carnage_report_id = $1;
    "#;

    sqlx::query_as::<_, CarnageReportTeam>(query)
        .bind(carnage_report_id)
        .fetch_all(transaction)
        .await
}

// Function to fetch kill events
pub async fn get_kill_events(
    carnage_report_id: Uuid,
    transaction: &mut PgConnection,
) -> Result<Vec<KillEvent>, Error> {
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

    sqlx::query_as::<_, KillEvent>(query)
        .bind(carnage_report_id)
        .fetch_all(transaction)
        .await
}

// Main function to fetch all related data using a transaction
pub async fn fetch_carnage_report_with_details(
    carnage_report_id: Uuid,
) -> Result<(CarnageReport, Vec<CarnageReportPlayer>, Vec<CarnageReportTeam>, Vec<KillEvent>), Error> {
    let pool = get_connection_pool().await;
    let mut transaction = pool.begin().await?;

    let report = fetch_carnage_report(carnage_report_id, &mut transaction).await?;
    let players = get_player_stats(carnage_report_id, &mut transaction).await?;
    let teams = get_team_stats(carnage_report_id, &mut transaction).await?;
    let kill_events = get_kill_events(carnage_report_id, &mut transaction).await?;

    transaction.commit().await?;

    Ok((report, players, teams, kill_events))
}
