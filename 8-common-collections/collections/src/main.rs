use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let _a = [1, 2, 3]; // Arrays: fixed length
    let mut v: Vec<i32> = Vec::new(); // Vector: growable array

    v.push(1);
    v.push(2);
    v.push(3);

    // Macro to create a vector
    let v2 = vec![1, 2, 3];

    let third = &v2[2]; // Accessing elements in unsafe way

    // ! v2.push(4);  // Error Thrown

    println!("(unsafe) This is the third element: {}", third);

    match v2.get(20) {
        // Accessing elements in safe way
        Some(third) => println!("(safe) This is the third element: {}", third),
        None => println!("There is no element at this position.",),
    }

    // Iterating over the values in a vector
    let mut v3 = vec![1, 2, 3, 4, 5];

    for i in &mut v3 {
        *i += 50 // Dereferencing operator
    }

    /*
    for i in &v3 {
        print!("{} ", i);
    }
    */

    for (index, i) in v3.iter().enumerate() {
        println!("element at index {}: {}", index, i);
    }

    // Using an enum to store multiple types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Array of enums: {:?}", row);

    // Strings
    // Strings are UTF-8 encoded

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    println!("{}", s);

    // Concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    println!("{}", s3);

    // Formatting
    let s4 = format!("s2: {} s3: {}", s2, s3);
    println!("{}", s4);

    // Indexing
    let s5 = String::from("नमस्ते");
    // let h = s5[0]; // Error thrown

    // Slicing
    // let hello = &s5[0..5]; // Error thrown

    // Iterating over strings

    // Scalar values
    for c in s5.chars() {
        print!("{} ", c);
    }
    println!("");

    // Bytes
    for b in s5.bytes() {
        print!("{} ", b);
    }
    println!("");

    // Grapheme clusters - not supported by Rust standard library
    // crate unicode-segmentation
    for g in s5.graphemes(true) {
        print!("{} ", g);
    }
    println!("");

    // Hash Maps
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let score = scores.get("Blue");
    println!("Score: {:?}", score);

    // Iterating over hash maps
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating a hash map
    // Overwriting a value
    scores.insert(String::from("Blue"), 25);
    println!("Updated scores: {:?}", scores);

    // Only inserting a value if the key has no value

    // Will not overwrite the value
    scores.entry(String::from("Yellow")).or_insert(1234);
    // Adding a new key-value pair
    scores.entry(String::from("Red")).or_insert(50);
    println!("Updated scores: {:?}", scores);

    // Updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    // Counting the number of times each word appears in some text
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
