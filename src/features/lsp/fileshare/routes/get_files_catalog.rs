use axum::response::IntoResponse;

#[axum::debug_handler]
pub async fn get_files_catalog() -> impl IntoResponse {
    String::from(
        "QuotaBytes: 0\r\n\
        QuotaSlots: 0\r\n\
        SlotCount: 0\r\n\
        VisibleSlots: 0\r\n\
        SubscriptionHash: 0\r\n\
        Message: Pardon our dust! File Share is currently Unavailable.\r\n"
    )
}