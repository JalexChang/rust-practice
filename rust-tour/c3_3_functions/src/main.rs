fn another_function(x: i32, y: i32) {
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    // function call
    println!("Function call");
    another_function(5, 6);


    // statements and expressions
    println!("Statements and expressions");
    let x = 5; // statement
    let y = { // block expression
        x + 1
    };
    println!("The value of x is: {x}"); // 5
    println!("The value of y is: {y}"); // 6

    // return value
    println!("Return value");
    println!("The value of plus_one(5) is: {}", plus_one(5));
}