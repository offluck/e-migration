use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn create_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(format!("Error connecting to {}", database_url).as_str())
}
