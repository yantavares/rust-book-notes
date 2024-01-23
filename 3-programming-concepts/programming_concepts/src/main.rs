fn main() {
    variables();
    data_types();
    functions();
    loops();
}

fn variables(){
    println!("--- Variables ---");

    let mut x = 5; // Variable (mutable)
    let y = 5; // variable (immutable)

    println!("reassigning values");
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // y = 6 => error

    println!("Shadowing");
    println!("The value of y is: {}", y);
    let y = "five";
    println!("The value of y is: {}", y);

    const MY_CONST: i32 = 100_000; // Type needs to be specified. Underline to make it more readable
    println!("My const: {}", MY_CONST);
}

fn data_types(){
    println!("--- Data Types ---");

    let tup = ("Hello!", 100); // Tuple
    let  (message, number) = tup; // Destructing tuple
    println!("Message: {} Number: {}", message, number);
    println!("Message: {} Number: {}", tup.0, tup.1); // Dot notation;

    let error_codes = [200, 404, 500];
    println!("Not_found: {}", error_codes[1]);

    let byte = [0; 8]; // Create an array with 8 values, all set to 0
    println!("First bit: {}", byte[0])
}

fn functions(){
    println!("--- Functions ---");
    
    fn my_function(x: i32, y: i32) -> i32 {
        let sum = x + y;
        return sum
    }

    fn my_function_2(x: i32, y: i32) -> i32 {
        x + y
    }

    let sum_1 = my_function(10, 20);
    let sum_2 = my_function_2(10, 20);

    println!("Sum using return: {}", sum_1);
    println!("Sum using implicit return: {}", sum_2);
}

fn loops(){
    let mut counter = 0;

    let result = loop { // Loop that returns i32 value
        counter += 1;

        if counter == 10 {
            break counter; // returns counter
        }
    };

    println!("Result: {}", result);

    let arr = [0, 1, 2, 3];

    // For loop example
    for element in arr.iter() { // .iter() to use references instead of actual values
        print!("{}", element);
    }
    println!("");

    for number in 0..4 { // Range (exclusive)
        print!("{}", number);
    }
    println!("");

}

// Normal comment

/*
Block comment
*/
