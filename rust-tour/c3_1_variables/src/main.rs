fn main() {
    // mutable variable
    let mut x = 5;
    println!("The value of x is: {x}"); // 5
    x = 6;
    println!("The value of x is: {x}"); // 6

    // constant variable
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {MAX_POINTS}");

    // shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}"); // 12

        // shadowing with different type
        let y = "different type";
        println!("The value of y in the inner scope is: {y}"); // different type
    }
    println!("The value of y is: {y}"); // 6
}
