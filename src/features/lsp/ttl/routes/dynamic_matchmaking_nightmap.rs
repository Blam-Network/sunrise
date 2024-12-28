use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use image::{Rgb, RgbImage};
use std::{fs, net::SocketAddr, path::PathBuf};
use axum::body::Body;
use tokio::sync::OnceCell;

/// Converts latitude and longitude to pixel coordinates on an equirectangular map.
fn lat_lon_to_pixel(lat: f64, lon: f64, width: u32, height: u32) -> (u32, u32) {
    let x = ((lon + 180.0) / 360.0 * width as f64) as u32;
    let y = ((90.0 - lat) / 180.0 * height as f64) as u32;
    (x, y)
}

/// Global constant for the stored image path.
static IMAGE_PATH: &str = "./res/features/ttl/dynamic_matchmaking_nightmap.jpg";

/// Predefined list of latitude and longitude coordinates.
fn predefined_dots() -> Vec<(f64, f64)> {
    vec![
        (37.7749, -122.4194), // San Francisco
        (51.5074, -0.1278),   // London
        (35.6895, 139.6917),  // Tokyo
        (-33.8688, 151.2093), // Sydney
        (40.7128, -74.0060),  // New York
    ]
}

/// Endpoint to generate the image with predefined dots.
pub async fn generate_image() -> impl IntoResponse {
    // Load the stored image
    let mut img = match image::open(IMAGE_PATH) {
        Ok(img) => img.into_rgb8(),
        Err(_) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to load image").into_response(),
    };

    let (width, height) = img.dimensions();
    let white = Rgb([255, 0, 0]);

    // Add predefined dots to the image
    for (lat, lon) in predefined_dots() {
        let (x, y) = lat_lon_to_pixel(lat, lon, width - 40, height);
        if x < width && y < height {
            img.put_pixel(x + 10, y, white);
        }
    }

    // Save the modified image to a buffer with high JPEG quality
    let mut buffer = vec![];
    {
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, 100); // Set quality to 95
        if let Err(_) = encoder.encode_image(&img) {
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to encode image").into_response();
        }
    }

    // Convert the buffer to a Body
    let body = Body::from(buffer);

    // Return the response with the correct body type
    Response::builder()
        .header("Content-Type", "image/jpeg")
        .body(body)
        .unwrap()
}