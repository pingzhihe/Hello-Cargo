fn main() {
    let user_1= User{
        username: String::from("Charles"),
        email: String::from("12@gmail.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("user_1: {}", user_1.username);

    let user_2 = User{
        username: String::from("Darton"),
        email: String::from("example@com"),
        ..user_1
    };
}

struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}