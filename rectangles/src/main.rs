fn main() {
    let w1 = 30;
    let h1 = 50;

    println! ("The area of rectangle is {} square pixels ",area(w1,h1));
}

fn area(w:u32,h:u32) -> u32 {
    w*h
}
