pub mod migrations;

use sqlx::{PgPool, Postgres};
use tokio::sync::OnceCell;
use std::env;
use std::error::Error;
use std::time::Duration;
use dotenv::dotenv;

// Define a static OnceCell for the connection pool
static POOL: OnceCell<PgPool> = OnceCell::const_new();

// Auxiliary async function to initialize the connection pool
async fn initialize_pool() -> Result<PgPool, sqlx::Error> {
    // Load environment variables
    dotenv().ok();

    // Get the database URL
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file or environment variables");

    // Create and return the connection pool
    sqlx::postgres::PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url).await
}

// Async function to get the connection pool
pub async fn get_connection_pool() -> &'static PgPool {
    POOL.get_or_try_init(|| async { initialize_pool().await }).await.expect("Error connecting to database")
}


// Async function to get the connection pool
pub async fn try_get_connection_pool() -> Result<&'static sqlx::Pool<Postgres>, Box<dyn Error + Send + Sync>> {
    POOL.get_or_try_init(|| async { initialize_pool().await }).await.map_err(Box::from)
}
