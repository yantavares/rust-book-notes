use std::cmp::PartialOrd;
use std::fmt::Display;

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string3 = String::from("long string is long");
    {
        let string4 = String::from("xyz");
        let result2 = longest(string3.as_str(), string4.as_str());
        // This code will still run, as result gets same lifetime as string4
        // (Shorter lifetime of the two strings)
        println!("The longest string is {}", result2);
    }
    // This code will not run, as result gets same lifetime as string4
    // println!("The longest string is {}", result2);

    // A curious case
    let string5 = String::from("long string is long");
    {
        let string6 = String::from("xyz");
        let result = longest(string5.as_str(), string6.as_str());
        println!("The longest string is {}", result);
        // result goes out of scope here
    }
    println!(
        "'Result' now refers back to first assignment \
        (line 5) whose lifetime is still valid. Result: {}",
        result
    );

    let number1 = 10;
    let number2 = 20;
    compare_and_display(&number1, &number2);
}

// A lifetime annotation is a way of telling the Rust compiler that
// the references in the function signature and the actual parameters will have the same lifetime.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Without lifetime annotation, the following code will not compile
    // because x and y could have different lifetimes.
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// A generic function `compare_and_display` that takes two references to values of the same type `T`
// T must implement `Display` (for printing) and `PartialOrd` (for comparison)
// The `'a` lifetime ensures that both references are valid for the same duration
fn compare_and_display<'a, T: Display + PartialOrd>(first: &'a T, second: &'a T) {
    if first > second {
        println!("First is greater: {}", first);
    } else if first < second {
        println!("Second is greater: {}", second);
    } else {
        println!("Both are equal: {}", first);
    }
}
