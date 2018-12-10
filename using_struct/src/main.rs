#[derive(Debug)]
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    // === Structの使い方-====
    let mut user1 = User{
        // Using &str woudl be better, but need to use liftimes.
        email: String::from("mail@mail.com"),
        name: String::from("Name Nam"),
        active: true,
        sign_in_count: 1,
    };

    println!("{:?}", user1);
    println!("{}", user1.email);

    user1.email = String::from("hey@yo.com");

    println!("{}", user1.email);

    // 関数から作る
    let user2 = build_user(String::from("hey@yo.com"),
                           String::from("My Name"),
                           &user1);
    println!("{:?}", user2);

    // Shorthand
    let user3 = User{
        email: String::from("new@user.com"),
        name: String::from("New Name"),
        ..user1
    };

    println!("{:?}", user3);

    // ==== Tuple Struct ====
    #[derive(Debug)]
    struct Color(i32, i32, i32);

    #[derive(Debug)]
    struct Something(i32, i32, i32);

    // color and something is different datatypes.
    let black = Color(0,0,0);
    let something = Something(1,2,3);

    println!("{:?}", black);
    println!("{:?}", something);
}

fn build_user(email: String, name: String, user: &User) -> User{
    User{
        email: email,
        name, // <- shorthand
        active: user.active,
        sign_in_count: user.sign_in_count,
    }
}
