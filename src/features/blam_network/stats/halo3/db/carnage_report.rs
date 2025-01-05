use sqlx::{PgPool, Row};
use uuid::Uuid;
use crate::features::common::database::get_connection_pool;

pub async fn fetch_carnage_report_with_details(
    carnage_report_id: Uuid,
) -> Result<String, sqlx::Error> {
    let pool = get_connection_pool().await;
    let row = sqlx::query("SELECT get_carnage_report_details($1)")
        .bind(carnage_report_id)
        .fetch_one(pool)
        .await?;

    let json_string: String = row.try_get(0)?;  // We expect a TEXT column in the result

    Ok(json_string)
}
