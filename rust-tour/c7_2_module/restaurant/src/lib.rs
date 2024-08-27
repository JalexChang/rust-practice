mod front_of_house {
    // add pub to make it public
    pub mod hosting {
        // add pub to make it public
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        // public field
        pub toast: String,
        // private field
        seasonal_fruit: String,
    }

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
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::seat_at_table();

    // Access public fields
    back_of_house::Breakfast::summer("Rye");
    back_of_house::Appetizer::Soup;
}