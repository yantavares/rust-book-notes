struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

fn main() {
    let mut user1 = User {
        username: String::from("yan"),
        email: String::from("yan@email.com"),
        sign_in_count: 1,
        is_active: true,
    };

    let name = user1.username;

    user1.username = String::from("Yan");
}
