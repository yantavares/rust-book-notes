fn main() {
    move_example();
    clone_example();
    copy_example();
    ownership_example();
    references_example();
    slices_example();
}

// *** Ownership ***

fn move_example(){
    let s1: String = String::from("Hello");
    let s2: String = s1; // Move (Now shallow copy)
    // * s1 goes out of scope
    // ! println!("This will not work: {}", s1);
    println!("This will work: {}", s2);
}

fn clone_example(){
    let s1: String = String::from("Hello");
    let s2: String = s1.clone(); // Clone (Now s1 doesnt go out of scope)

    println!("This will work: {}", s1);
    println!("This will also work: {}", s2);
}

fn copy_example(){
    // Simple types in stack are not moved, are copyied by default
    // int, boolean, char
    let x = 5;
    let mut y = 5; // Copy (x is still in scope)

    y = y + 1;

    println!("This will work: {}", x);
    println!("This will also work: {}", y);
}

fn ownership_example(){
    let s = String::from("Hello");

    takes_ownership(s);

    // * String s is moved to some_string, so s gets droped
    // ! println!("This will not work {}", s)

    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s2 = takes_and_gives_back(s2);

    println!("s1: {}, s2: {}", s1, s2);

}

fn takes_ownership(some_string: String){
    println!("This takes ownership of {}", some_string)
}

fn gives_ownership() -> String{
    let some_string: String = String::from("Hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String{
    a_string
}

// *** References ***

fn references_example(){
    let s1 = String::from("Hello");

    let len = calculate_length(&s1); // Function gets reference to the value

    // * Value doenst get owned, gets borrowed!

    println!("The length of {} is {}.", s1, len);

    let mut s2 = String::from("Hello");
    change(&mut s2); // References are immutable by default

    let r1 = &mut s2;

    // ! let r2 = &mut s2;
    // There can only be one mutable reference to a value at a time 

    // ! let r2 = &s2;
    // You cant have both mutable and immutable references at the same time

    println!("This is r1: {}", r1);
    
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    // Changes value without taking ownership
    s.push_str(" World!")
}

/*
* fn invalid_reference_example(){
    let reference_to_nothing = dangle();
}

* fn dangle() -> &String {
    let s = String::from("Hello");
    &s
}

! After function runs, s is droped, so reference points to nothing
*/

// *** String Slices ***
fn slices_example(){
    let mut s: String = String::from("hello world");

    let _hello = &s[..5];
    let _world = &s[6..];
    let _hello_world = &s[..];

    let word = first_word(&s); // Word is a string slice, tied to the string itself

    println!("First word: {}", word);

    s.clear(); // if s is droped, word value also changes

    // ! println!("This will not work: {}", word)

}

fn first_word(s: &str) -> &str {
    /* Notice that referece to String
    is automatically converted to string slice reference

    &String -> &str */
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..]
}

