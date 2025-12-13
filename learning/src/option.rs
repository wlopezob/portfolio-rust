#[derive(Debug, Clone)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // --snip--
        }
    }
}

#[derive(Debug, Clone)]
enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let some_number = Some(5);
    let some_chars = Some('e');

    let absent_number: Option<i32> = None;
    

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y.unwrap_or(0);

    let five = Some(5);
    let six = plus_one(five);   
    let none = plus_one(None);

    if let Some(i) = six {
        println!("The value of six is: {}", i);
    }

    let rs = five.map(|f| f + 1).unwrap_or_else(|| 0);

    let mut count = 0;
    let coint = Coin2::Quarter(UsState::Alaska);
    let coint2 = coint.clone();
    match coint {
        Coin2::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    println!("Count: {}", count);

    
    if let Coin2::Quarter(state) = coint2 {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    describe_state_quarter(Coin2::Quarter(UsState::Alabama));
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn describe_state_quarter(coin: Coin2) {
    if let Coin2::Quarter(state) = coin {
        if state.existed_in(1999) {
            println!("This quarter is from a state that existed in 1999: {:?}", state);
        } else {
            println!("This quarter is from a state that did not exist in 1999: {:?}", state);
        }
    }
}