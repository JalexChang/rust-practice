fn main() {
    let v: Vec<i32> = Vec::new();
    println!("empty vector: {:?}", v);

    let v = vec![1, 2, 3];
    println!("vector with values: {:?}", v);

    // Add elements to a mutable vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.pop(); // last in first out
    println!("mutable vector: {:?}", v); // [5, 6]

    // Access elements by index
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("The third element is {}", third);

    // Access elements by get method
    match v.get(100) {
        Some(hundred) => println!("The 100th element is {}", hundred),
        None => println!("There is no 100th element."),
    }

    // Iterate over elements
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 10;
        println!("{}", i);
    }

    // Enum with vector
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}
