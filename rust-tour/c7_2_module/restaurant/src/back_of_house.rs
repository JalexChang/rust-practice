pub struct Breakfast {
    // public field
    pub toast: String,
    // private field
    seasonal_fruit: String,
}

#[derive(Debug)]
pub enum Appetizer {
    // enum is public by default
    Soup,
    Salad,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

fn fix_incorrect_order() {
    cook_order();
    // super is like .. in the file system:
    // back_of_house -> crate -> front_of_house
    super::front_of_house::serving::serve_order();
}

fn cook_order() {}