mod front_of_house  {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn server_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
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
}

fn deliver_order() {}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

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