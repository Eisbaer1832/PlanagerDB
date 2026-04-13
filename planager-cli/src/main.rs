use std::fs;
use planager_data::database::{insert_class, insert_lesson};
use planager_parser::*;

fn main() {
    println!("Welcome the planager cli!");
    let file = fs::read_to_string("../plan.xml");

    let result = parse_xml(file);
    for class in result {
        insert_class(&class);

        for lesson in &class.lessons {
            insert_lesson(lesson, &class);
        }

    }

}
