// Private by default
mod serving {
    fn _take_order() {}
    pub fn serve_order() {}
    fn _take_payment() {}
}

pub mod hosting; // importing from "house/hosting.rs"

/*
Equivalent to

mod hosting {
    pub fn add_to_waitlist() {}
}

*/

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        _cook_order();

        // Super refers to parent module
        super::serving::serve_order();
    }

    fn _cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
