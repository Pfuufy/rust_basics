#[derive(Debug)]
enum State {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State)
}

fn main() {
    // let some_val = Some(3u8);

    // match some_val {
    //     Some(3) => println!("three"),
    //     _ => ()
    // }

    // // this and the match are the same
    // if let Some(3) = some_val {
    //     println!("three");
    // }

    // if some_val == Some(3) {
    //     println!("another three")
    // }

    let mut count = 0;
    let coin = Coin::Quarter(State::Alabama);

    // match coin {
    //     Coin::Quarter(state) => (),
    //     _ => count += 1
    // }

    // this and the above match statement are the same
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}", state);
    // } else {
    //     count += 1;
    // }


    // but I still dont understand the point of if let, why not just if?

    // the answer is that you can't extract values out of an enum with just an if.
    // the below code will not work if you do an if. Also, an if statement wont
    // work for coin to compare it to coin because youre comparing a value
    // to an enum type, and the two are incompatible.

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }

    if true == true {
        println!("Mais non! C'est vrai!");
    }

}
