#[derive(Debug)]
enum State {
  Alabama,
  Alaska,
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(State),
}

fn main() {
  let penny = Coin::Penny;
  println!("Value: {}", value_in_cents(penny));

  let quarter = Coin::Quarter(State::Alabama);
  println!("Value: {}", value_in_cents(quarter));
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => { 
      println!("Lucky penny!");
      1
    }
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("Quarter from state: {state:?}");
      25
    }
  }
}