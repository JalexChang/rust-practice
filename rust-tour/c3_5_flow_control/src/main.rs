fn main() {
    // if expression
    let x = 5;
    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six :(");
    }

    // tri expression
    let y = if x == 5 { 10 } else { 15 };
    println!("y is {}", y);

    // loop expression
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    }; // 20
    println!("count result is {}", result);

    // loop with label
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");

    // while expression
    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for through range
    for number in (1..4).rev() {
        println!("{number}!");
    }
}