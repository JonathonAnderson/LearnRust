pub use crate::front_of_house::hosting;
use std::{ fmt::Result, io::Result as IoResult, collections::HashMap };

mod front_of_house;
mod back_of_house;

fn deliver_order() {}

fn make_map() {
    let mut map = HashMap::new();
    map.insert(1,2);
}

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Imported path
    hosting::add_to_waitlist();

    // Order breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = "Wheat".to_owned();
    println!("I'd like {} toast please", meal.toast);

    // Won't compile because seasonal_fruit is private
    // meal.seasonal_fruit = "blueberries".to_owned();

    let order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;

    order1.call();
}