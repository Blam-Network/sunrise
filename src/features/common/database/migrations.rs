use sqlx::{migrate::Migrator};
use dotenv::dotenv;
use crate::features::common::database::get_connection_pool;

// Define the path to the migrations folder
static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn run_migrations() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    let pool = get_connection_pool().await;

    // Run migrations
    MIGRATOR.run(pool).await.unwrap();
    Ok(())
}
