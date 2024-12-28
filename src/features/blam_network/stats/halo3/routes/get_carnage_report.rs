use axum::{extract::Path, Json, response::IntoResponse};
use crate::features::blam_network::stats;
use serde::Serialize;
use uuid::Uuid;
use crate::features::blam_network::stats::halo3::db::carnage_report::{CarnageReportPlayer, CarnageReportTeam, KillEvent};

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
struct CarnageReport {
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
    pub players: Vec<CarnageReportPlayer>,
    pub teams: Vec<CarnageReportTeam>,
    pub kills: Vec<KillEvent>
}



#[axum::debug_handler]
pub async fn get_carnage_report(Path(carnage_report_id): Path<Uuid>) -> impl IntoResponse {
    // Fetch players and report concurrently
    let players_or_error = stats::halo3::db::carnage_report::get_player_stats(carnage_report_id).await;
    let report_or_error = stats::halo3::db::carnage_report::fetch_carnage_report(carnage_report_id).await;
    let kills_or_error = stats::halo3::db::carnage_report::get_kill_events(carnage_report_id).await;
    let teams_or_error = stats::halo3::db::carnage_report::get_team_stats(carnage_report_id).await;


    match (players_or_error, report_or_error, kills_or_error, teams_or_error) {
        (Ok(players), Ok(report), Ok(kills), Ok(teams)) => {
            Json(CarnageReport {
                team_game: report.team_game,
                start_time: report.start_time,
                finish_time: report.finish_time,
                game_variant_name: report.game_variant_name,
                map_variant_name: report.map_variant_name,
                map_id: report.map_id,
                hopper_name: None,
                game_engine: report.game_engine,
                file_type: report.file_type,
                duration: report.duration,
                players: players,
                kills: kills,
                teams: teams
            })
        }
        _ => {
            panic!("error retrieving carnage_report data")
            // eprintln!("Error fetching carnage report: {}", e);
            // Json(format!("Error: {}", e))
        }
    }
}