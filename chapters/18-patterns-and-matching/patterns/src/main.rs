fn main() {
    // Using match expression
    #[derive(Debug)]
    enum Language {
        Rust,
        _Python,
        _Java,
        _C,
        _Cpp,
        _PHP,
    }

    let lang = Language::Rust;

    match lang {
        Language::Rust => println!("Rust"),
        Language::_Python => println!("Python"),
        Language::_Java => println!("Java"),
        Language::_C => println!("C"),
        Language::_Cpp => println!("Cpp"),
        // This is a catch all
        // _ => println!("Other"),

        // We can also use the caught value
        lang => println!("Caught: {:?}", lang),
    }

    // Using if let expression
    // We can use if let to match a single pattern, and ignore the rest

    let lang = Language::Rust;

    if let Language::Rust = lang {
        println!("Rust");
    } else {
        println!("Other");
    }

    // We can use while let to match a pattern

    let mut lang = Some(Language::Rust);

    // The loop will run until lang is not Some(Language::Rust)
    while let Some(Language::Rust) = lang {
        println!("Rust");
        lang = None;
    }

    // In for loop
    let languages = vec![
        Language::Rust,
        Language::_Python,
        Language::_Java,
        Language::_C,
    ];

    // In this case we can use iterators or simply `for lang in languages`
    for lang in languages.iter() {
        if let Language::Rust = lang {
            println!("Rust");
        } else {
            println!("Other");
        }
    }

    // Using let statements

    // We can use let to match a pattern and assign the value to a variable
    // (Simmilar to Haskell)
    let (x, y, z) = (1, 2, 3);

    println!("x: {}, y: {}, z: {}", x, y, z);

    // Matching in function parameters

    // This function takes a reference to a tuple that matches the pattern (i32, i32)
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("x: {}, y: {}", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);

    // Refutable patterns
}
