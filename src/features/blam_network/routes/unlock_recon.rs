use std::collections::HashMap;
use axum::debug_handler;
use axum::http::{StatusCode};
use axum::response::IntoResponse;
use hyper::HeaderMap;
use serde::Deserialize;
use crate::features::common::database::get_connection_pool;

#[derive(Deserialize)]
struct Achievement {
    id: u32,
    unlockedOnline: bool,
}

#[derive(Deserialize)]
struct AchievementsResponse {
    achievements: Vec<Achievement>,
}

#[debug_handler]
pub async fn unlock_recon(headers: HeaderMap) -> impl IntoResponse {
    let xuid = headers.get("x-xuid")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok()).unwrap();
    let uhs = headers.get("x-uhs").and_then(|v| v.to_str().ok()).unwrap();
    let xsts = headers.get("Authorization").and_then(|v| v.to_str().ok()).unwrap();

    let auth_header = format!("XBL3.0 x={};{}", uhs, xsts);
    let mut headers_map = reqwest::header::HeaderMap::new();
    headers_map.insert("x-xbl-contract-version", "1".parse().unwrap());
    headers_map.insert("Authorization", auth_header.parse().unwrap());

    let halo3_url = format!(
        "https://achievements.xboxlive.com/users/xuid({})/achievements?titleId=1297287142&unlockedOnly=true&maxItems=79",
        xuid
    );
    let halo3_odst_url = format!(
        "https://achievements.xboxlive.com/users/xuid({})/achievements?titleId=1297287287&unlockedOnly=true&maxItems=47",
        xuid
    );

    let client = reqwest::Client::new();
    let halo3_response = client.get(&halo3_url).headers(headers_map.clone()).send().await;
    let halo3_odst_response = client.get(&halo3_odst_url).headers(headers_map).send().await;

    if halo3_response.is_err() || halo3_odst_response.is_err() {
        println!("Can't unlock recon: Failed to request cheevos.");
        return StatusCode::UNAUTHORIZED;
    }

    let halo3_achievements: AchievementsResponse = halo3_response.unwrap().json().await.unwrap();
    let halo3_odst_achievements: AchievementsResponse = halo3_odst_response.unwrap().json().await.unwrap();

    let required_ids = [
        (91, true),  // Lightswitch (unlockedOnline)
        (92, true),  // Seven on Seven (unlockedOnline)
        (63, true),  // Annual (unlockedOnline)
        (90, false), // Brainpan (doesn't need unlockedOnline)
        (108, true), // Endure (unlockedOnline)
        (109, true), // Déjà Vu (unlockedOnline)
        (107, true), // Classic (unlockedOnline)
    ];

    let achievements_map: HashMap<u32, bool> = halo3_achievements.achievements.iter()
        .chain(&halo3_odst_achievements.achievements)
        .map(|a| (a.id, a.unlockedOnline))
        .collect();

    let all_unlocked = required_ids.iter().all(|&(id, check_online)| {
        achievements_map.get(&id).map_or(false, |&unlocked_online| !check_online || unlocked_online)
    });

    if all_unlocked {
        let pool = get_connection_pool().await;
        sqlx::query("CALL halo3.unlock_recon($1)")
            .bind(xuid as i64)
            .execute(pool)
            .await.unwrap();

        StatusCode::OK
    } else {
        println!("Can't unlock recon: Not all vidmasters unlocked.");
        StatusCode::UNAUTHORIZED
    }
}