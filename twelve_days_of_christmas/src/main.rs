fn main() {
    for day in 1..=12 {
        match day {
            1  => println!("On the first day of Christmas, my true love gave to me:"),
            2  => println!("On the second day of Christmas, my true love gave to me:"),
            3  => println!("On the third day of Christmas, my true love gave to me:"),
            4  => println!("On the fourth day of Christmas, my true love gave to me:"),
            5  => println!("On the fifth day of Christmas, my true love gave to me:"),
            6  => println!("On the sixth day of Christmas, my true love gave to me:"),
            7  => println!("On the seventh day of Christmas, my true love gave to me:"),
            8  => println!("On the eigth day of Christmas, my true love gave to me:"),
            9  => println!("On the ninth day of Christmas, my true love gave to me:"),
            10 => println!("On the tenth day of Christmas, my true love gave to me:"),
            11 => println!("On the eleventh day of Christmas, my true love gave to me:"),
            12 => println!("On the twelfth day of Christmas, my true love gave to me:"),
            _  => println!("Somehow got an integer outside the range 1..=12")
        };

        for day_gift in (1..=day).rev() {
            match day_gift {
                1  => { 
                        match day {
                            1 => println!("A partridge in a pear tree."),
                            _ => println!("And a partridge in a pear tree.")
                        }
                        
                },
                2  => println!("Two turtle doves,"),
                3  => println!("Three French hens,"),
                4  => println!("Four calling birds,"),
                5  => println!("Five golden rings,"),
                6  => println!("Six geese a laying,"),
                7  => println!("Seven swans a swimming,"),
                8  => println!("Eight mades a milking,"),
                9  => println!("Nine ladies dancing,"),
                10 => println!("Ten lords a leaping,"),
                11 => println!("Eleven pipers piping,"),
                12 => println!("Twelve drummers drumming,"),
                _  => println!("Somehow got an integer outside the range 1..=12")
            };
        }

        println!("----"); 
    }
}
