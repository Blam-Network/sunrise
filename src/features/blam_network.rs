use axum::{Router, routing::get};
use crate::features::common::title_server::APIFeature;

mod stats;
mod screenshots;

pub struct BlamNetwork {}

impl APIFeature for BlamNetwork {
    fn get_router(&self) -> Router {
        Router::new()
            .route("/blamnet/stats/halo3/carnage-reports/{carnage_report_id}", get(stats::halo3::routes::get_carnage_report::get_carnage_report))
            .route("/blamnet/halo3/emblem", get(stats::halo3::routes::get_emblem::generate_emblem_image))
            .route("/blamnet/halo3/screenshot/{screenshot_id}/view", get(screenshots::halo3::routes::view_screenshot::get_screenshot_jpeg))

    }

    fn get_name(&self) -> &str {
        "Blam Network APIs"
    }
}
