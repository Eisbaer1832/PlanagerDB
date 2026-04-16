use std::fs;
use planager_data::database::{establish_connection};
use planager_parser::*;
use clap::Parser;
use planager_data::insert::populate_database;
use planager_data::queries::fetch_subject_cancellation;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = String::from("../plan.xml"))]
    xml_file: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = String::from("../planager-data/database.db"))]
    database: String,
}

fn main() {
    println!("Welcome the planager cli!");


    let args = Arguments::parse();
    let db_location = args.database;
    let xml_file = args.xml_file;

    let file = fs::read_to_string(xml_file);
    let result = parse_xml(file);

    let conn = &mut establish_connection(db_location);

    populate_database(result, conn);

    let results = fetch_subject_cancellation(conn);
    
    for res in results {
        println!("{} {}", res.0, res.1)
    } 
}