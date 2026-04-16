use clap::Parser;
use diesel::SqliteConnection;
use planager_data::database::establish_connection;
use planager_data::insert::populate_database;
use planager_data::queries::fetch_subject_cancellation;
use planager_parser::*;
use std::fs;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// Name of the person to greet
    #[arg(short, long,  default_value_t = String::new())]
    add: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = String::from("../planager-data/database.db"))]
    database: String,
}

fn main() {
    println!("Welcome the planager cli!");

    let args = Arguments::parse();
    let db_location = args.database;
    let add_this = args.add;

    let conn = &mut establish_connection(db_location);

    if add_this != "" {
        add(add_this, conn)
    }


    let results = fetch_subject_cancellation(conn);
    
    for res in results {
        println!("{} {}", res.0, res.1)
    } 
}

fn add(file: String, conn: &mut SqliteConnection) {
    println!("Adding {} to the database", file);
    let file = fs::read_to_string(file);
    let result = parse_xml(file);
    populate_database(result, conn);

}