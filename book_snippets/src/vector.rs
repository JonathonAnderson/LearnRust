fn main() {
  // Declaration
  let _v1: Vec<i32> = Vec::new();
  let _v2 = vec![1, 2, 3];

  // Mutability
  let mut v3 = Vec::new();

  v3.push(5);
  v3.push(6);
  v3.push(7);
  v3.push(8);

  // Getting element
  let v4 = vec![1, 2, 3, 4, 5];

  let third: &i32 = &v4[2];
  println!("The third element is {third}");

  let third: Option<&i32> = v4.get(2);
  match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element")
  }

  // Index out of bounds
  // let _does_not_exist = &v4[100];
  let _does_not_exist = v4.get(100);

  // Mutable and immutable borrow
  let first = &v3[0];

  // v3.push(9);

  println!("The first element is {first}");

  // Double mutable borrow
  let mut first = &mut v3[0];

  // v3.push(9);

  println!("The first element is {first}");

  // Iterating
  for i in &v4 {
    println!("{i}");
  }
  for i in &mut v3 {
    *i += 50;
  }
  for i in &v3 {
    println!("{i}");
  }

  // Mutable and immutable borrow
  for _i in &v3 {
    // v3.push(*i+1);
  }
  // Double mutable borrow
  for _i in &mut v3 {
    // v3.push(*i+1);
  }

  // Enum to store different types
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let mut row: Vec<SpreadsheetCell> = Vec::new();

  row.push(SpreadsheetCell::Int(3));
  row.push(SpreadsheetCell::Float(10.12));
  row.push(SpreadsheetCell::Text("blue".to_owned()));

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Float(10.12),
    SpreadsheetCell::Text("blue".to_owned()),
  ];

  for cell in &row {
    match cell {
      SpreadsheetCell::Int(x) => println!("Int = {x}"),
      SpreadsheetCell::Float(x) => println!("Float = {x}"),
      SpreadsheetCell::Text(text) => println!("Text = {text}"),
    }
  }
}