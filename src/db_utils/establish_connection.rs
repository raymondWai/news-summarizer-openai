use std::env;
use std::fmt::format;

use diesel::{Connection, PgConnection};
use dotenvy::dotenv;

pub fn establish_connection() -> PgConnection {
    let _ = dotenv();

    let database_user: String = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let database_pw: String = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let database_db: String = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    let database_host: String = env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
    let database_url = String::from(format(format_args!(
        "postgres://{}:{}@{}/{}",
        database_user, database_pw, database_host, database_db,
    )));
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
