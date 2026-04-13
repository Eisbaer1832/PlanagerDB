use std::fs;
use planager_data::Class;
use planager_data::database::{insert_class, insert_lesson};
use planager_parser::*;

fn main() {
    println!("Welcome the planager cli!");
    let file = fs::read_to_string("../plan.xml");
    let result = parse_xml(file);

    populate_database(result)
}


fn populate_database(classes: Vec<Class>) {
    for class in classes {
        insert_class(&class);

        for lesson in &class.lessons {
            insert_lesson(lesson, &class);
        }
    }
}