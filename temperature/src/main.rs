use std::io;

fn main() {

    let conversion: u32 = loop {
        println!("1) Convert F to C");
        println!("2) Convert C to F");
        println!("Choose a unit conversion: ");

        let mut conversion = String::new();
        io::stdin().read_line(&mut conversion).expect("Input should be received from console.");

        match conversion.trim().parse() {
            Ok(number) => { 
                if number == 1 || number == 2 {
                    break number
                }
            },
            Err(_) => continue
        };
    };

    let temperature: f64 = loop {
        println!("Enter a temperature");

        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature).expect("Input should be received from console.");

        match temperature.trim().parse() {
            Ok(number) => break number,
            Err(_) => continue
        }
    };

    if conversion == 1 {
        let celsius = (temperature - 32.0) * (5.0/9.0);
        println!("{temperature}-F equals {celsius}-C");
    }
    else if conversion == 2 {
        let fahrenheit = (temperature * (9.0/5.0)) + 32.0;
        println!("{temperature}-C equals {fahrenheit}-F");
    }
}
