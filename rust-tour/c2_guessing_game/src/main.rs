use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // 1. Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // 2. Read the user input for the guess and compare it with the secret number
    println!("Guess the number!");
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed t/o read line");
        // Shadowing the guess variable to convert it to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    // 3. Announce the secret number
    println!("The secret number is: {secret_number}");
}
