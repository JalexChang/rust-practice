fn main() {

    // Allocate a string on the heap
    let mut s1 = String::from("hello");

    // push_str() appends a literal to a String, allocating another string on the heap
    // After this operation, the original s1 is no longer valid.
    s1.push_str(", world!");

    println!("s1={s1}"); // hello, world!

    // Ownership move
    // 1. Copying data in stack (shallow copy) => transfer the ownership
    let s2 = s1;
    // println!("s1={s1}"); // error[E0382]: use of moved value: `s1`
    // s1.push_str("!!!"); // error[E0386]: cannot borrow `s1` as mutable, as it is not declared as mutable
    println!("s2={s2}"); // hello, world!

    // 2. Clone a heap data (deep copy) => no ownership transfer
    let s3 = s2.clone();
    println!("s2={s2}, s3={s3}"); // s2=hello, world!, s3=hello, world!

    // 3. Copying stack-only data => no ownership transfer
    let x = 5;
    let y = x;
    println!("x={x}, y={y}"); // x=5, y=5
    

    // Function ownership transfer
    let s4 = gives_ownership();
    println!("s4={s4}"); // hello
    takes_ownership(s4);
    // println!("s4={s4}"); // error[E0382]: use of moved value: `s4`

    let s5 = gives_ownership();
    let s6 = takes_and_gives_back(s5);
    // println!("s5={s5}"); // error[E0382]: use of moved value: `s5`
    println!("s6={s6}"); // hello
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    s
}

fn takes_ownership(s: String) {
    println!("s={s}");
    // s goes out of scope and is dropped
}

fn takes_and_gives_back(s: String) -> String {
    s
}