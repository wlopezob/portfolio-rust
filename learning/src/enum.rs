// enum IpAddrKind {
//     V4,
//     V6,
// }

enum IpAddrKind {
    V4(String),
    V6(String),
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        
    }
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn values_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn values_in_cents2(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    let home = IpAddr {
        kind: IpAddrKind::V4(String::from("127.0.0.1")),
        address: String::from("10.10.10.2"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6(String::from("::1")),
        address: String::from("::1"),
    };
    let m = Message::Write(String::from("hello"));
    m.call();


}