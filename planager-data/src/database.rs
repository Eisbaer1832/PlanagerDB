use diesel::prelude::*;
use dotenvy::dotenv;

pub fn establish_connection(db_location: String) -> SqliteConnection {
    dotenv().ok();
    println!("{}", db_location);
    SqliteConnection::establish(&db_location)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_location))
}
