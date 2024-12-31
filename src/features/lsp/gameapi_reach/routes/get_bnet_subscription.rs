use axum::response::IntoResponse;

#[axum::debug_handler]
pub async fn get_bnet_subscription() -> impl IntoResponse {
    String::from("Status: Suscribed")
}