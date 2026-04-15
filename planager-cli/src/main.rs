use std::fs;
use planager_data::Class;
use planager_data::database::{insert_class, insert_lesson};
use planager_parser::*;
use clap::Parser;
use planager_data::queries::fetch_subject_cancellation;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = String::from("../plan.xml"))]
    xml_file: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = String::from("database.db"))]
    database: String,
}

fn main() {
    println!("Welcome the planager cli!");
    let args = Arguments::parse();
    let db = args.database;
    let xml_file = args.xml_file;

    let file = fs::read_to_string(xml_file);
    let result = parse_xml(file);

    populate_database(result, db);

    let results = fetch_subject_cancellation();
    
    for res in results {
        println!("{} {}", res.0, res.1)
    } 
}


fn populate_database(classes: Vec<Class>, db_location: String) {

    for class in classes {
        insert_class(&class);

        for lesson in &class.lessons {
            insert_lesson(lesson, &class);
        }
    }
}