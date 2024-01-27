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
}
