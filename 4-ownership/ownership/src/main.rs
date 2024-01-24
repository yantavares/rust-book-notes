fn main() {
    move_example();
    clone_example();
    copy_example();
    ownership_example();
}

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