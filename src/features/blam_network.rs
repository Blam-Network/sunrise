use axum::{Router, routing::get};
use axum::routing::post;
use crate::features::blam_network::routes::unlock_recon::unlock_recon;
use crate::features::common::title_server::APIFeature;

mod stats;
mod screenshots;
mod routes;

pub struct BlamNetwork {}

impl APIFeature for BlamNetwork {
    fn get_router(&self) -> Router {
        Router::new()
            .route("/blamnet/stats/halo3/carnage-reports/{carnage_report_id}", get(stats::halo3::routes::get_carnage_report::get_carnage_report))
            .route("/blamnet/halo3/emblem", get(routes::get_emblem::generate_emblem_image))
            .route("/blamnet/halo3/screenshot/{screenshot_id}/view", get(screenshots::halo3::routes::view_screenshot::get_screenshot_jpeg))
            .route("/blamnet/halo3/unlock_recon", post(unlock_recon))
            .route("/blamnet/halo3odst/screenshot/{screenshot_id}/view", get(screenshots::halo3odst::routes::view_screenshot::get_odst_screenshot_jpeg))

    }

    fn get_name(&self) -> &str {
        "Blam Network APIs"
    }
}
