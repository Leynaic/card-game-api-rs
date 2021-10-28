use dotenv::dotenv;
use std::env;
use postgres::{Client, NoTls};

pub fn establish_connection() -> Client {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    Client::connect(&database_url, NoTls)
        .expect(&format!("Error connecting to {}", database_url))
}