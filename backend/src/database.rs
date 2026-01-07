use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use dotenvy::dotenv; // Note: 'dotenvy' is the modern version of 'dotenv'

// 1. Define the DbPool type so other files can use it
pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set!");

    // 2. Create a connection manager
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    // 3. Build the pool
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}