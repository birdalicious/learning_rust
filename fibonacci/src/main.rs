use std::io;

fn main() {
    println!("Enter the nth number of the fibonacci number");

    let mut input = String::new();

    let n: u32 = loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");

        match input.trim().parse() {
            Ok(val) => break val,
            Err(_) => {
                println!("Enter a number");
                continue;
            }
        }
    };

    println!("{}", get_fib_number(n));
}

fn get_fib_number(n: u32) -> u32 {
    if n < 2 {
        return 1;
    };

    let mut i1 = 1;
    let mut i2 = 1;
    let mut i = 1;

    while i < n {
        let t = i2; 
        i2 = i1 + i2;
        i1 = t;

        i += 1;
    }

    i2
}
