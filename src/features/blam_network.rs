use axum::{Router, routing::get};
use crate::features::common::title_server::APIFeature;

mod stats;

pub struct BlamNetwork {}

impl APIFeature for BlamNetwork {
    fn get_router(&self) -> Router {
        Router::new().route(
            "/blamnet/stats/halo3/carnage-reports/{carnage_report_id}",
            get(stats::halo3::routes::get_carnage_report::get_carnage_report),
        )
    }

    fn get_name(&self) -> &str {
        "Blam Network APIs"
    }
}
