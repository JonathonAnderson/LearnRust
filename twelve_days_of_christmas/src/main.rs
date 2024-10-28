fn main() {
    for day in 1..=12 {

        print!("On the ");
        match day {
            1  => print!("first"),
            2  => print!("second"),
            3  => print!("third"),
            4  => print!("fourth"),
            5  => print!("fifth"),
            6  => print!("sixth"),
            7  => print!("seventh"),
            8  => print!("eigth"),
            9  => print!("ninth"),
            10 => print!("tenth"),
            11 => print!("eleventh"),
            12 => print!("twelfth"),
            _  => print!("Somehow got an integer outside the range 1..=12")
        };
        println!(" day of Christmas, my true love gave to me:");

        for day_gift in (1..=day).rev() {
            match day_gift {
                1  => { 
                        match day {
                            1      => println!("A partridge in a pear tree."),
                            2..=12 => println!("And a partridge in a pear tree."),
                            _      => println!("Somehow got an integer outside the range 1..=12")
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
