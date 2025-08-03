fn main() {
    let my_string = String::from("Hello World");
    let word= first_word(&my_string[0..6]);
    let word= first_word(&my_string[0..]);
    let word = first_word(&my_string);
    println!("First word (full string): {}", word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
