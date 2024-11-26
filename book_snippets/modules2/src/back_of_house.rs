fn fix_incorrect_order() {
  cook_order();
  super::deliver_order();
}

fn cook_order() {}

// Each element of a struct must be declared pub to be accessible by parent
pub struct Breakfast {
  pub toast: String,
  seasonal_fruit: String,
}

// Each function implemented on the struct must be pub to be accessible by parent
impl Breakfast {
  // Because seasonal_fruit is private, a constructor method is needed to return 
  // a configured Breakfast struct. Otherwise, there is no war for the parent
  // to supply a value for seasonal_fruit, and it won't compile because there is 
  // no null and we're not using Option<T>
  pub fn summer(toast: &str) ->  Breakfast {
      Breakfast {
          toast: toast.to_owned(),
          seasonal_fruit: "peaches".to_owned(),
      }
  }
}

// All variants are accessible by parent when enum is declared pub
pub enum Appetizer {
  Soup,
  Salad,
}

// Each function implemented on the enum must be declared pub to be accessible by the parent
impl Appetizer {
  pub fn call(&self) {
      println!("Enum method called");
  }
}