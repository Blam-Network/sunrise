use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use image::{Rgb, RgbImage};
use std::{fs, net::SocketAddr, path::PathBuf};
use std::error::Error;
use axum::body::Body;
use ip2location::{error, Record, DB};
use sqlx::Row;
use crate::features::common::database::get_connection_pool;

/// Converts latitude and longitude to pixel coordinates on an equirectangular map.
fn lat_lon_to_pixel(lat: f32, lon: f32, width: u32, height: u32) -> (u32, u32) {
    let x = ((lon + 180.0) / 360.0 * width as f32) as u32;
    let y = ((90.0 - lat) / 180.0 * height as f32) as u32;
    (x, y)
}

/// Global constant for the stored image path.
static IMAGE_PATH: &str = "./res/features/ttl/dynamic_matchmaking_nightmap.jpg";
const IPV4BIN: &str = "./res/features/ttl/IP2LOCATION-LITE-DB5.BIN";

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

async fn get_lat_longs() -> Result<Vec<(f32, f32)>, Box<dyn Error>> {
    let mut results = Vec::<(f32, f32)>::new();

    let mut db = DB::from_file(IPV4BIN)?;
    let pool = get_connection_pool().await;

    let mut ip_addresses = sqlx::query(
        r#"
        SELECT DISTINCT(mq.inaddr_online)
        FROM halo3.matchmaking_quality mq
        WHERE mq.created_at >= NOW() - INTERVAL '1 day'
        "#
    )
        .fetch_all(pool)
        .await?;

    // Map the results to a vector of strings (IP addresses)
    let ip_addresses: Vec<String> = ip_addresses
        .into_iter()
        .map(|row| row.try_get(0)) // 0 is the column index for 'inaddr_online'
        .collect::<Result<Vec<String>, _>>()?;

    ip_addresses.iter().for_each(|ip| {
        let record = db.ip_lookup(ip.parse().unwrap());
        if record.is_err() { return }
        let record = record.unwrap();
        let record = if let Record::LocationDb(rec) = record {
            Some(rec)
        } else {
            None
        };

        if let Some(record) = record {
            if record.latitude.is_some() && record.longitude.is_some() {
                results.push((
                    record.latitude.unwrap(),
                    record.longitude.unwrap()
                ))
            }
        }
    });

    Ok(results)
}

/// Endpoint to generate the image with predefined dots.
pub async fn generate_matchmaking_nightmap_image() -> impl IntoResponse {
    // Load the stored image
    let mut img = match image::open(IMAGE_PATH) {
        Ok(img) => img.into_rgb8(),
        Err(_) => return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to load image").into_response(),
    };

    let (width, height) = img.dimensions();
    let white = Rgb([255, 255, 255]);

    // Add predefined dots to the image
    for (lat, lon) in get_lat_longs().await.unwrap() {
        let (x, y) = lat_lon_to_pixel(lat, lon, width - 50, height + 40);
        if x < width && y < height {
            img.put_pixel(x + 15, y - 5, white);
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