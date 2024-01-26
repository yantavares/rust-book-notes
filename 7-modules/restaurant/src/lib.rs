mod house {
    // Private by default
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn _seat_at_table() {}
    }

    mod serving {
        fn _take_order() {}
        pub fn serve_order() {}
        fn _take_payment() {}
    }

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
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::house::hosting::add_to_waitlist();

    // Relative path
    house::hosting::add_to_waitlist();

    house::back_of_house::fix_incorrect_order();

    // Order a breakfast in the summer with rye toast
    let meal = house::back_of_house::Breakfast::summer("Rye");

    let order1 = house::back_of_house::Appetizer::Soup;
    let order2 = house::back_of_house::Appetizer::Salad;

    println!(
        "I'd like {} toast please and {} fruit.",
        meal.toast, meal.seasonal_fruit
    );

    println!("Order 1: {:?}. Order 2: {:?}", order1, order2);
}
