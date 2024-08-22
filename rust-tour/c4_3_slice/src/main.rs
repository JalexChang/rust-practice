fn main() {
    // Slice
    let s = String::from("hello, world!");
    let hello = &s[..5];
    let world = &s[7..12];
    println!("{hello} {world}");

    let w1 = first_word(&s);
    // s.clear(); // error[E0382]: borrow of moved value: `s`
    println!("the first word={w1}");

    // String literals are slices
    let s = "hello, world!";
    let w2 = first_word(&s);
    println!("the first word={w2}")
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..] // whole string
}
