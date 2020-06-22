#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    // let cents = get_cents(Coin::Penny);
    // let cents = get_cents(Coin::Quarter(UsState::Alabama));

    // println!("Cents: {}", cents);


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


    match_u8(127);
}

fn get_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => get_one(),
        Coin::Nickel => {
            5
        },
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {:?}", state);
            25
        }
    }
}

fn get_one() -> i32 {
    1
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(val) => Some(val + 1)
    }
}

fn match_u8(num: u8) {
    match num {
        1 => println!("One"),
        3 => println!("Three"),
        _ => ()
    }
}