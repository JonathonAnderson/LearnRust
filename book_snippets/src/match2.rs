fn main() {
  let five = Some(5);
  let six = plus_one(five);
  let seven = plus_one(six);
  let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => {
      println!("Orignal value: {}", i);
      Some(i +  1)
    }
  }
}