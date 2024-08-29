use std::collections::HashMap;

fn main() {
    // Create a new hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("hash map: {:?}", scores);

    // Create a hash map from a vector of tuples
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("hash map from vector: {:?}", scores);

    // Accessing values in a hash map
    // default value 0 if key not found
    let score_blue = scores.get(&"Blue".to_string()).copied().unwrap_or(&0);
    let score_red = scores.get(&"Red".to_string()).copied().unwrap_or(&0);
    println!("score of Blue: {}", score_blue);
    println!("score of Red: {}", score_red);

    // Iterate over a hash map
    for (key, value) in scores {
        println!("{}: {}", key, value);
    }

    // Update a hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // Overwrite the value
    scores.insert(String::from("Blue"), 20);
    println!("updated hash map: {:?}", scores); // {"Blue": 20}
    // Insert a value if the key has no value
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("inserted hash map: {:?}", scores); // {"Blue": 20, "Yellow": 50}

    // Update a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("word count: {:?}", map);
}
