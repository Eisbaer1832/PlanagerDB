pub fn sanitize_subject(subject: &String) -> String {
    let mut s = subject.to_uppercase();
    s.retain(|c| !c.is_ascii_digit()); // remove subject id integers
    s = s.replace("-", "");

    if s.len() == 3 {
        let lc = s.chars().nth(2);
        if lc.unwrap().eq(&'E') {
            s = s.get(0..2).unwrap().to_string();
        }
    }

    s
}