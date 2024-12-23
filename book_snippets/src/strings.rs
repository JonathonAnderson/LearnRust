fn main() {
  let mut s = "foo".to_owned();
  s.push_str("bar");
  
  let mut s1 = "foo".to_owned();
  let s2 = "bar";
  s1.push_str(s2);
  println!("s2 is {s2}");

  let mut s3 = "lo".to_owned();
  s3.push('l');

  let s4 = "Hello, ".to_owned();
  let s5 = "World!".to_owned();
  let s6 = s4 + &s5;

  let s7 = "tic".to_owned();
  let s8 = "tac".to_owned();
  let s9 = "toe".to_owned();

  // let s10 = s7 + "-" + &s8 + "-" + &s9;
  let s10 = format!("{s7}-{s8}-{s9}");
}
