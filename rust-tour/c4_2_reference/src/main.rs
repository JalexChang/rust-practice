fn main() {
    // Reference and borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}");

    // Mutable reference
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("s2={s2}"); // hello, world!

    let r1 = &mut s2;
    println!("r1={r1}"); // hello, world!!!
    // let r2 = &mut s2; // error[E0499]: cannot borrow `s2` as mutable more than once at a time
    // workaround: use a new scope
    {
        let r2 = &mut s2;
        println!("r2={r2}"); // hello, world!!!
    }


    // Dangling reference
    // let reference_to_nothing = dangle(); // error[E0106]: missing lifetime specifier

}

fn calculate_length(s: &String) -> usize {
    s.len()
    // s goes out of scope but does not drop the value because it does not have ownership
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

// fn dangle() -> &String { // dangle returns a reference to a String
//     let s = String::from("hello");
//     &s // we return a reference to the String, s
// } // s goes out of scope, and is dropped. Its memory goes away. Danger!
