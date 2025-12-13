struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user = User {
        active: true,
        username: String::from("william"),
        email: String::from("wlop@mail.com"),
        sign_in_count: 1,
    };

    //println!("User: {:?} ", user);
    dbg!(&user);

    user.email = String::from("another@mail.com");

    let user2 = User {
        email: String::from("mail@mail.com"),
        ..user
    };

    println!("User2 email: {}", user.email);

    user.email = String::from("another@mail.com");
    println!("User2 email: {}", user.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    let width = 30;
    let height = 50;
    println!("Area is {} square pixels.", area(width, height));

}

#[derive(Debug)]
struct User {
    active: bool,
    username: String, 
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}