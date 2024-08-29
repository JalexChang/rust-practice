fn main() {
    // String is a growable, mutable, owned, UTF-8 encoded string type

    // Initiate String
    let s = String::new();
    println!("empty string: {}", s);

    let data = "initial contents";
    let s = data.to_string();
    println!("string from data: {}", s);

    let s = "initial contents".to_string();
    println!("string from string: {}", s);

    let s = String::from("Hello, 你好, こんにちは");
    println!("UTF-8 encoded string: {}", s);

    // Update String
    let mut s = String::from("foo");
    s.push_str("bar"); // push_str takes a string slice
    s.push('!'); // push takes a single character
    println!("updated string: {}", s);

    // Concatenate Strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    // println!("concatenated string: {}", s1); // error[E0382]: borrow of moved value: `s1`
    println!("concatenated string: {}", s3);

    // Format Macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // format! does not take ownership of its parameters
    println!("formatted string s1: {}", s1);
    println!("formatted string: {}", s);

    // Indexing
    // let s = String::from("hello");
    // let h = s[0]; // error[E0277]: the type `String` cannot be indexed by `{integer}`

    // Slicing
    // Use slicing to get a byte slice of a string.
    // This is not safe because it could return a partial character.
    let hello = "Здравствуйте";
    let first_two_chars = &hello[0..4];
    println!("first four bytes: {}", first_two_chars);
    // let first_two_chars = &hello[0..3]; // panic: byte index 3 is not a char boundary; it is inside 'д' (bytes 2..4) of `Здравствуйте`

    // Iterating
    // The best way to iterate over a string is to use the chars/bytes method.
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }
}
