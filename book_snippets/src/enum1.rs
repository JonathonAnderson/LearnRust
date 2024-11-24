enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {
    match self {
      Self::Write(x) => {
        println!("{}", x);
      },
      Self::Move{ x, y } => {
        println!("x {}, y {}", x, y);
      },
      Self::ChangeColor(r, g, b) => {
        println!("r: {}, g: {}, b: {}", r, g, b);
      },
      Self::Quit => {
        println!("quit");
      }
    }
  }
}

fn main() {
  let m = Message::Write("hello".to_owned());
  m.call();

  let w = Message::Move{ x: -10, y: 43 };
  w.call();

  let p = Message::ChangeColor(18, -83, 23);
  p.call();

  let q = Message::Quit;
  q.call();
}