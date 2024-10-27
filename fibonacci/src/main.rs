use std::io;

fn main() {
    let f0 = 0;
    let f1 = 1;

    let n: u128 =  loop {
        println!("Enter the Nth position in the Fibonacci sequence to calculate");

        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Input should be receievd from console");

        match n.trim().parse() {
            Ok(number) => break number,
            Err(_) => continue
        }
    };

    let fnth = fibonacci_sum(f0, f1, n);

    println!("The {n} position in the Fibonacci sequence calculates to {fnth}");
}

fn fibonacci_sum(fa: u128, fb: u128, remaining: u128) -> u128 {
    if remaining == 0 {
        return fa
    }
    else if remaining == 1 {
        return fb
    }

    return fibonacci_sum(fb, fa + fb, remaining -1);
}