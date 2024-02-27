#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)  // Quarter のみ州情報を保持する
}

fn main() {
    let penny = value_in_cents(Coin::Penny);
    println!("Penny value: {}", penny);

    let quarter = value_in_cents(Coin::Quarter(UsState::Alabama));
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            // ブロックの最後の値が返る
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
