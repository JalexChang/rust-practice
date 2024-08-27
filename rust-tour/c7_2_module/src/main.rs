use crate::garden::vegetables::Asparagus;
use restaurant::eat_at_restaurant;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");

    eat_at_restaurant();
}