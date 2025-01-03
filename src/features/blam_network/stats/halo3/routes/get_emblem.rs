use std::io::Cursor;
use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};
use axum::Json;
use uuid::Uuid;
use crate::features::blam_network::stats;
use axum::{
    extract::{Query, Path},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use axum::body::Body;
use image::imageops::FilterType;
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Deserialize)]
pub struct Emblem {
    pub armour_primary_color: Option<usize>,
    pub size: usize,
    pub primary: usize,
    pub secondary: bool,
    pub background: usize,
    pub primary_color: usize,
    pub secondary_color: usize,
    pub background_color: usize,
}

#[derive(Debug, Clone, Default)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

const COLORS: [Color; 30] = [
    Color { r: 110u8, g: 110u8, b: 110u8, a: 255u8 },  // Steel
    Color { r: 178u8, g: 178u8, b: 178u8, a: 255u8 },  // Silver
    Color { r: 200u8, g: 200u8, b: 200u8, a: 255u8 },  // White
    Color { r: 167u8, g: 59u8, b: 59u8, a: 255u8 },    // Red
    Color { r: 224u8, g: 115u8, b: 115u8, a: 255u8 },  // Mauve
    Color { r: 242u8, g: 141u8, b: 141u8, a: 255u8 },  // Salmon
    Color { r: 223u8, g: 150u8, b: 0u8, a: 255u8 },    // Orange
    Color { r: 251u8, g: 184u8, b: 98u8, a: 255u8 },   // Coral
    Color { r: 255u8, g: 210u8, b: 167u8, a: 255u8 },  // Peach
    Color { r: 212u8, g: 182u8, b: 50u8, a: 255u8 },   // Gold
    Color { r: 240u8, g: 205u8, b: 53u8, a: 255u8 },   // Yellow
    Color { r: 255u8, g: 223u8, b: 132u8, a: 255u8 },  // Pale
    Color { r: 99u8, g: 128u8, b: 28u8, a: 255u8 },    // Sage
    Color { r: 155u8, g: 176u8, b: 108u8, a: 255u8 },  // Green
    Color { r: 218u8, g: 241u8, b: 169u8, a: 255u8 },  // Olive
    Color { r: 56u8, g: 132u8, b: 137u8, a: 255u8 },   // Teal
    Color { r: 85u8, g: 196u8, b: 201u8, a: 255u8 },   // Aqua
    Color { r: 156u8, g: 239u8, b: 239u8, a: 255u8 },  // Cyan
    Color { r: 59u8, g: 101u8, b: 158u8, a: 255u8 },   // Blue
    Color { r: 96u8, g: 148u8, b: 223u8, a: 255u8 },   // Cobalt
    Color { r: 163u8, g: 191u8, b: 246u8, a: 255u8 },  // Sapphire
    Color { r: 96u8, g: 71u8, b: 155u8, a: 255u8 },   // Violet
    Color { r: 156u8, g: 129u8, b: 233u8, a: 255u8 },  // Orchid
    Color { r: 208u8, g: 196u8, b: 255u8, a: 255u8 },  // Lavender
    Color { r: 144u8, g: 0u8, b: 81u8, a: 255u8 },    // Crimson
    Color { r: 216u8, g: 69u8, b: 143u8, a: 255u8 },   // Ruby Wine
    Color { r: 255u8, g: 150u8, b: 195u8, a: 255u8 },  // Pink
    Color { r: 93u8, g: 64u8, b: 22u8, a: 255u8 },    // Brown
    Color { r: 182u8, g: 150u8, b: 121u8, a: 255u8 },  // Tan
    Color { r: 228u8, g: 198u8, b: 172u8, a: 255u8 },  // Khaki
];


fn get_color(index: usize) -> Color {
    COLORS.get(index).map(|color| color.clone()).unwrap_or(Color::default())
}

fn get_colored_layer(
    layer: &str,
    image: &DynamicImage,
    color_index: usize,
    size: u32,
) -> RgbaImage {
    let img = image.resize(size, size, FilterType::Nearest);
    let mut img = img.to_rgba8();  // Convert image to RGBA
    let color = get_color(color_index);
    let layer_index = match layer {
        "PRIMARY" => 2,
        "SECONDARY" => 1,
        _ => 0,
    };

    // Process each pixel in the image
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let Rgba([r, g, b, a]) = *pixel; // Destructure the pixel into RGBA components

        let new_color = if layer_index == 1 && r > 0 {
            // Modify alpha based on red channel value
            (color.r, color.g, color.b, (r as f32 / 15.0 * 255.0) as u8)
        } else if layer_index == 2 && g > 0 {
            // Modify alpha based on green channel value
            (color.r, color.g, color.b, (g as f32 / 240.0 * 255.0) as u8)
        } else if layer_index == 0 && b > 0 {
            // Modify alpha based on blue channel value
            (color.r, color.g, color.b, (b as f32 / 15.0 * 255.0) as u8)
        } else {
            // Default to transparent
            (0, 0, 0, 0)
        };

        // Apply the modified color to the pixel
        *pixel = Rgba([new_color.0, new_color.1, new_color.2, new_color.3]);
    }

    img
}

fn draw_emblem(emblem: &Emblem, size: usize) -> RgbaImage {
    let primary_image = image::open(format!("./res/features/blam_network/emblems/emblems [{}].png", emblem.primary))
        .expect("Failed to open primary image");
    let background_image = image::open(format!("./res/features/blam_network/emblems/emblems [{}].png", emblem.background))
        .expect("Failed to open background image");

    let colored_primary = get_colored_layer("PRIMARY", &primary_image, emblem.primary_color, size as u32);
    let colored_secondary = get_colored_layer("SECONDARY", &primary_image, emblem.secondary_color, size as u32);
    let colored_background = get_colored_layer("BACKGROUND", &background_image, emblem.background_color, size as u32);

    let mut canvas = RgbaImage::new(size as u32, size as u32);

    if let Some(armour_primary_color) = emblem.armour_primary_color {
        let color = get_color(armour_primary_color);
        for (x, y, pixel) in canvas.enumerate_pixels_mut() {
            *pixel = Rgba([color.r, color.g, color.b, 255]);
        }
    }

    for (x, y, pixel) in colored_background.enumerate_pixels() {
        let mut canvas_pixel = canvas.get_pixel_mut(x, y).clone();
        if pixel.0[3] > 0 {  // Check if the primary pixel is non-transparent
            canvas_pixel = blend_pixel(canvas_pixel, *pixel);
            canvas.put_pixel(x, y, canvas_pixel);
        }    }
    //
    if emblem.secondary {
        for (x, y, pixel) in colored_secondary.enumerate_pixels() {
            let mut canvas_pixel = canvas.get_pixel_mut(x, y).clone();
            if pixel.0[3] > 0 {  // Check if the primary pixel is non-transparent
                canvas_pixel = blend_pixel(canvas_pixel, *pixel);
                canvas.put_pixel(x, y, canvas_pixel);
            }
        }
    }
    //
    for (x, y, pixel) in colored_primary.enumerate_pixels() {
        let mut canvas_pixel = canvas.get_pixel_mut(x, y).clone();
        if pixel.0[3] > 0 {  // Check if the primary pixel is non-transparent
            canvas_pixel = blend_pixel(canvas_pixel, *pixel);
            canvas.put_pixel(x, y, canvas_pixel);
        }
    }

    canvas
}

fn blend_pixel(background: Rgba<u8>, foreground: Rgba<u8>) -> Rgba<u8> {
    let alpha = foreground.0[3] as f32 / 255.0;
    let inv_alpha = 1.0 - alpha;

    let r = (foreground.0[0] as f32 * alpha + background.0[0] as f32 * inv_alpha) as u8;
    let g = (foreground.0[1] as f32 * alpha + background.0[1] as f32 * inv_alpha) as u8;
    let b = (foreground.0[2] as f32 * alpha + background.0[2] as f32 * inv_alpha) as u8;
    let a = (alpha * 255.0) as u8;

    Rgba([r, g, b, a])
}

#[axum::debug_handler]
pub async fn generate_emblem_image(
    Query(emblem): Query<Emblem>,
) -> impl IntoResponse {
    let size = emblem.size; // You can set this dynamically if needed
    let image = draw_emblem(&emblem, size);

    let mut buf = Cursor::new(Vec::new());  // Use Cursor to wrap the Vec<u8>

    image.write_to(&mut buf, ImageFormat::Png)
        .expect("Failed to write image");

    let buf = buf.into_inner();
    let body: Body = buf.into();

    Response::builder()
        .header("Content-Type", "image/png")
        .body(body)
        .unwrap()
}