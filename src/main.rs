use std::fs;
use roxmltree::Document;

fn main() -> std::io::Result<()> {
    let file = fs::read_to_string("plan.xml");
    match file {
        Ok(xml) => {
            let string = &*xml;
            let doc = Document::parse(string);
            match doc {
                Ok(doc) => {
                    for class in doc.descendants() {
                        // get class
                        if let Some(class) = class.descendants().find(|n| n.has_tag_name("Kl")) {
                            // get class name
                            if let Some(name_node) = class.descendants().find(|n| n.has_tag_name("Kurz")) {
                                println!("Klasse: {}", name_node.text().unwrap_or(""))
                            }

                            // get subjects
                            for subject_node in class.descendants().filter(|n| n.has_tag_name("UeNr")) {
                                if let Some(subject_name_node) = subject_node.attributes().find(|n| n.name().eq("UeFa")) {
                                    print!("    Fach: {} bei ", subject_name_node.value())
                                }

                                if let Some(teacher_node) = subject_node.attributes().find(|n| n.name().eq("UeLe")) {
                                    println!("{}", teacher_node.value())
                                }
                            }

                            // get Timetable

                        }
                    }
                }
                _ => {}
            }
        }
        Err(_) => {}
    }
    Ok(())
}
