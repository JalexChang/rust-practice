mod front_of_house;
mod back_of_house;


pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::seat_at_table();

    // Access public fields
    back_of_house::Breakfast::summer("Rye");
    let soup = back_of_house::Appetizer::Soup;
    println!("I'd like {:#?}!", soup);
}