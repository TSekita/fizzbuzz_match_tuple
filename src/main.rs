use std::io::Write;

fn fizzbuzz() {
    let mut tuple: (bool, bool, bool) = (false, false, false);
    for i in 1..=100 {
        print!("{} ", i);
        std::io::stdout().flush().unwrap();
        tuple.0 = i % 15 == 0;
        tuple.1 = i % 3 == 0;
        tuple.2 = i % 5 == 0;
        match tuple {
            (true, _, _) => println!("fizzbuzz"),
            (false, true, _) => println!("fizz"),
            (false, _, true) => println!("buzz"),
            _ => println!(""),
        }
    }
}

fn main() {
    fizzbuzz();
}
