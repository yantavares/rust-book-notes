// *** Named Struct ***
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

impl User {
    // impl -> Implementation
    // Implementing Method
    fn display_username(&self) {
        println!("{}", self.username);
    }
}

// *** Tuple Structs ***

// implementing display trait (will now be printable)
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    named_srtucts();
    tuple_structs();
}

fn named_srtucts() {
    let mut user1 = User {
        username: String::from("yan"),
        email: String::from("yan@email.com"),
        sign_in_count: 1,
        is_active: true,
    };

    user1.username = String::from("Yan");

    user1.display_username();

    let user2 = build_user(String::from("test@email.com"), String::from("Test"));

    let user3 = User {
        username: String::from("User3"),
        ..user2 // Copy the rest of the attr from user2s
    };

    println!(
        "Same as user2: {} {} {}",
        user3.email, user3.is_active, user3.sign_in_count
    )
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        is_active: true,
        sign_in_count: 1,
    }
}

fn tuple_structs() {
    let point = Point(1, 2, 3);
    let color = Color(1, 2, 3);

    println!("This is my color: {:?}", color); // Printing struct
    println!("This is my Pretty point. {:#?}", point) // Printing Pretty String (attr separated)
}
