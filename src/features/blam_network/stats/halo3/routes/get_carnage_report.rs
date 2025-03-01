use axum::{extract::Path, response::IntoResponse};
use axum::http::HeaderMap;
use uuid::Uuid;
use crate::features::blam_network::stats::halo3::db::carnage_report::fetch_carnage_report_with_details;


#[axum::debug_handler]
pub async fn get_carnage_report(Path(carnage_report_id): Path<Uuid>) -> impl IntoResponse {
    let json_string = fetch_carnage_report_with_details(carnage_report_id).await.unwrap();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    (headers, json_string)
}