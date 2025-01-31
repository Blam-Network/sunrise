use axum::debug_handler;
use axum::extract::Path;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use blf_lib::blf::BlfFile;
use blf_lib::blf::versions::halo3odst::v13895_09_04_27_2201_atlas_release::{s_blf_chunk_content_header, s_blf_chunk_end_of_file, s_blf_chunk_screenshot_camera, s_blf_chunk_screenshot_data, s_blf_chunk_start_of_file};
use blf_lib::blf_file;
use bytes::Bytes;
use sqlx::query_scalar;
use uuid::Uuid;
use crate::features::common::database::get_connection_pool;

// TODO: Move
blf_file! {
    pub struct blind_screenshot {
        pub _blf: s_blf_chunk_start_of_file,
        pub chdr: s_blf_chunk_content_header,
        pub scnc: s_blf_chunk_screenshot_camera,
        pub scnd: s_blf_chunk_screenshot_data,
        pub _eof: s_blf_chunk_end_of_file
    }
}

#[debug_handler]
pub async fn get_odst_screenshot_jpeg(Path(screenshot_id): Path<Uuid>) -> impl IntoResponse {
    let query = r#"
        SELECT author_id
        FROM odst.blind_screenshot
        WHERE id = $1
    "#;

    let pool = get_connection_pool().await;
    let author_xuid: i64 = query_scalar(query)
        .bind(screenshot_id)
        .fetch_one(pool)
        .await
        .unwrap();

    // TODO: graceful 404
    println!("author_xuid: {}", author_xuid);
    println!("./blind_uploads/odst/screenshots/{author_xuid:08X}/{screenshot_id}");

    let screenshot = blind_screenshot::read_file(&format!("./blind_uploads/odst/screenshots/{author_xuid:08X}/{screenshot_id}")).unwrap();
    ([(header::CONTENT_TYPE, "image/jpeg")], screenshot.scnd.jpeg_data)
}