use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // Panic macro
    // * panic!("crash and burn");
    a();

    // Result enum
    let f = File::open("hello.txt");

    // Match expression - A little verbose
    let _f = match f {
        Ok(file) => file,
        Err(error) => {
            // ErrorKind enum
            match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating file: {:?}", e),
                },
                other_error => panic!("Problem opening file: {:?}", other_error),
            }
        }
    };

    // Using closures - more clean
    // See more about closures in chapter 13
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating file: {:?}", error);
            })
        } else {
            panic!("Problem opening file: {:?}", error);
        }
    });

    // Using unwrap and expect
    let _f = File::open("hello.txt").unwrap();
    let _f = File::open("hello.txt").expect("Failed to open hello.txt");

    // Propagating errors
    let _f = read_username_from_file().expect("Failed to read username from file");
}

fn a() {
    b();
}

fn b() {
    // Will cause panic if num == 22
    c(21);
}

fn c(num: i32) {
    if num == 22 {
        panic!("Don't use 22!");
    }
}

fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = File::open("hello.txt");

    // Propagating errors
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    // Using ? operator
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    // Using ? operator in a chain
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// Same as read_username_from_file but using ? operator and chaining calls
fn _read_username_from_file_short() -> Result<String, std::io::Error> {
    // Using ? operator in a chain
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
