#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coinarr = [
        Coin::Penny,
        Coin::Nickel,
        Coin::Quarter(UsState::Alaska),
        Coin::Quarter(UsState::Alabama),
        Coin::Dime,
    ];
    let mut count1 = 0;
    let mut count2 = 0;

    for coin in &coinarr {
        match coin {
            Coin::Quarter(state) => println!("match => State quarter from {:?}!", state),
            _ => count1 += 1,
        }

        if let Coin::Quarter(state) = coin {
            println!("iflet => State quarter from {:?}!", state);
        } else {
            count2 += 1
        };
    }
    println!("{}, {}", count1, count2);
}
