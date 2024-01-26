use rand::{thread_rng, Rng}; // Nested paths

// Blob import
// * use rand::*;

mod house; // Importing a module from another file in the same directory

// For other modules to access hosting directly, we need to make it public
pub use crate::house::hosting;

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

// Absolute path
// * use crate::house::hosting;

// Relative path
// * use self::house::hosting;

pub fn eat_at_restaurant2() {
    let mut rng = thread_rng();
    let _secret_number = rand::thread_rng().gen_range(1..100);
    let _random_value: u8 = rng.gen();

    hosting::add_to_waitlist();
}
