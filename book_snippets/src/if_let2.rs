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

  let quarter = Coin::Quarter(State::Alabama);

  let mut count = 0;
  if let Coin::Quarter(state) = quarter {
    println!("Quarter from {state:?}");
  } else {
    println!("Incrementing coin count");
    count += 1;
  }

  if let Coin::Quarter(state) = penny {
    println!("Quarter from {state:?}");
  } else {
    println!("Incrementing coin count");
    count += 1;
  }
}
