mod front_of_house {
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
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    front_of_house::back_of_house::fix_incorrect_order();
}
