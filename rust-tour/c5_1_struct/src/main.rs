#[derive(Debug)] // derive the Debug trait for User
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
#[derive(Debug)] // derive the Debug trait for Point
struct Point(i32, i32, i32);
struct Unit;

fn main() {
    // struct
    let user1 = build_user(String::from("someone@xxx.com"), String::from("someone"));
    println!("user1: {user1:#?}");
    let user2 = User {
        username: String::from("anotherone"),
        ..user1
    };

    // println!("user1 email: {}", user1.email); // [E0382]: borrow of moved value: `user1.email`
    println!("user1 username: {}", user1.username);
    println!("user2: {user2:?}", );


    // tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: ({}, {}, {})", black.0, black.1, black.2);
    println!("origin: {origin:#?}");

    // unit-like struct
    let _ = Unit; // anonymous unit-like struct instance
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}