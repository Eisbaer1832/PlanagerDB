pub fn sanitize_subject(subject: &String) -> String {
    let mut s = subject.to_uppercase();
    s.retain(|c| !c.is_ascii_digit()); // remove subject id integers
    s = s.replace("-", "");

    s
}