use std::env;

use diesel::{Connection, PgConnection};
use dotenv::dotenv;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&url).expect(&format!("Error connecting to {}", url))
}
