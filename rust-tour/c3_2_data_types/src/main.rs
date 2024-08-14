use std::any::type_name_of_val;

fn main() {
    // i32 and f64 are in default data types
    let x1 = 2; // i32
    let x2: u32 = 2; // u32
    let y1 = 3.0; // f64
    let y2: f32 = 3.0; // f32
    println!(
        "{} {} {} {}",
        type_name_of_val(&x1),
        type_name_of_val(&x2),
        type_name_of_val(&y1),
        type_name_of_val(&y2)
    );

    // addition
    let sum = 5 + 10; // 15
    // subtraction
    let difference = 95.5 - 4.3; // 91.2
    // multiplication
    let product = 4 * 30; // 120
    // division
    let quotient = 56.7 / 32.2; // 1.76
    let truncated = -5 / 3; // -1
    // remainder
    let remainder = 43 % 5; // 3
    println!("{sum} {difference} {product} {quotient} {truncated} {remainder}");

    // boolean
    let t = true;
    let f: bool = false;
    println!("{t} {f}");

    // character
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{c} {z} {heart_eyed_cat}");

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let five_hundred = tup.0;
    println!("{:?}: {}", tup, type_name_of_val(&tup));
    println!("{x} {y} {z} {five_hundred}");

    // unit type
    let _unit = ();
    println!("{_unit:?}");

    // array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [3; 5]; // [3, 3, 3, 3, 3]
    println!("{:?}: {}", arr, type_name_of_val(&arr));
    println!("{:?}: {}", arr2, type_name_of_val(&arr2));
}
