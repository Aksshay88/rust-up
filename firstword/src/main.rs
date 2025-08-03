fn main() {
    let  s = String::from("Hello, world!");
    let len = s.len();
    let slice= &s[0..3];
    println!("slice of the string is: {}" , slice);
}
