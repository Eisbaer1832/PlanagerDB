use std::io;
use roxmltree::Document;
use planager_data::*;

pub fn parse_xml(file: io::Result<String>) -> Vec<Class> {
    let mut classes: Vec<Class> = vec![];
    let xml = file.unwrap();
    let string = &*xml;
    let doc = Document::parse(string);

    match doc {
        Ok(doc) => {
            for class in doc.descendants() {
                // get class
                if let Some(class) = class.descendants().find(|n| n.has_tag_name("Kl")) {
                    let mut c= Class::default();

                    // get class name
                    if let Some(name_node) = class.descendants().find(|n| n.has_tag_name("Kurz")) {
                        let name = String::from(name_node.text().unwrap_or(""));
                        (c.post_fix, c.year) = parse_class_name(name)
                    }

                    // get subjects
                    for subject_node in class.descendants().filter(|n| n.has_tag_name("UeNr")) {
                        let mut subject = Subject::default();
                        if let Some(subject_name_node) = subject_node.attributes().find(|n| n.name().eq("UeFa")) {
                            subject.name = String::from(subject_name_node.value());
                        }

                        if let Some(teacher_node) = subject_node.attributes().find(|n| n.name().eq("UeLe")) {
                            subject.teacher = String::from(teacher_node.value());
                        }
                        c.subjects.push(subject);
                    }

                    // get Timetable
                    let mut lessons: Vec<Lesson> = vec![];
                    for lesson_node in class.descendants().filter(|n| n.has_tag_name("Std")) {
                        let mut lesson = Lesson::default();
                        if let Some(subject_name_node) = lesson_node.descendants().find(|n| n.has_tag_name("St")) {
                            lesson.time = subject_name_node.text().unwrap_or("").parse().unwrap();
                        }

                        let mut subject = Subject::default();
                        if let Some(subject_name_node) = lesson_node.descendants().find(|n| n.has_tag_name("Fa")) {
                            subject.name = String::from(subject_name_node.text().unwrap_or(""));

                            if subject.name == "---" {
                                lesson.canceled = true
                            }else {
                                lesson.canceled = false
                            }
                        }

                        if let Some(teacher_node) = lesson_node.descendants().find(|n| n.has_tag_name("Le")) {
                            subject.teacher = String::from(teacher_node.text().unwrap_or(""));
                        }
                        lesson.subject = subject;

                        if let Some(teacher_node) = lesson_node.descendants().find(|n| n.has_tag_name("If")) {
                            let note = teacher_node.text().unwrap_or("");
                            if note != "" {
                                lesson.note = String::from(note);
                            }
                        }
                        lessons.push(lesson);
                    }


                    c.lessons = lessons;
                    classes.push(c)
                }

            }
        }
        _ => {}
    }


    classes
}


fn parse_class_name(name:String) -> (String, i32) {
    let num_end = name.find(|c: char| !c.is_numeric()).unwrap_or(name.len());
    let potential_num_str = &name[..num_end];

    let mut class_year: i32 = 0;
    if !potential_num_str.is_empty() {
        class_year = if potential_num_str.len() > 2 {
            *&potential_num_str[..2].parse().unwrap()
        } else {
            potential_num_str.parse().unwrap()
        };
    }

    let class_post_fix: String = name.chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    (class_post_fix, class_year)
}