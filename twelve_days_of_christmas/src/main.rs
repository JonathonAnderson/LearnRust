fn main() {
    for day in 1..=12 {
        let ordinal = match day {
            1  => String::from("first"),
            2  => String::from("second"),
            3  => String::from("third"),
            4  => String::from("fourth"),
            5  => String::from("fifth"),
            6  => String::from("sixth"),
            7  => String::from("seventh"),
            8  => String::from("eigth"),
            9  => String::from("ninth"),
            10 => String::from("tenth"),
            11 => String::from("eleventh"),
            12 => String::from("twelfth"),
            _  => String::from("error")
        };

        println!("On the {ordinal} day of Christmas, my true love gave to me:");

        for day_gift in (1..=day).rev() {
            let gift = match day_gift {
                1  => String::from("A partridge in a pear tree."),
                2  => String::from("Two turtle doves"),
                3  => String::from("Three French hens"),
                4  => String::from("Four calling birds"),
                5  => String::from("Five golden rings"),
                6  => String::from("Six geese a laying"),
                7  => String::from("Seven swans a swimming"),
                8  => String::from("Eight mades a milking"),
                9  => String::from("Nine ladies dancing"),
                10 => String::from("Ten lords a leaping"),
                11 => String::from("Eleven pipers piping"),
                12 => String::from("Twelve drummers drumming"),
                _  => String::from("error")
            };

            if day > 1 && day_gift == 1 {
                print!("And ");
                println!("{}", gift.replace("A", "a"));
            }
            else if day > 1 && day_gift > 1 {
                println!("{gift},");
            }
            else {
                println!("{gift}");
            }
        }

        println!("----"); 
    }
}
