use std::collections::HashMap;

use rand::Rng;

fn main() {
    let mut numbers = Vec::new();

    for _ in 0..10 {
        let n = rand::thread_rng().gen_range(1..=100);
        numbers.push(n);
    }

    println!("Numbers: {:?}", numbers);

    println!("Median: {}", median(&numbers));
    println!("Mode: {}", mode(&numbers));
}

fn median(numbers: &Vec<i32>) -> i32 {
    if numbers.len() == 0 {
        return 0;
    }

    let mut sorted = numbers.clone();
    sorted.sort();

    let m = sorted.len()/2;
    sorted[m]
}

fn mode(numbers: &Vec<i32>) -> i32 {
    if numbers.len() == 0 {
        return 0;
    }

    let mut counts = HashMap::new();

    for n in numbers {
        let count = counts.entry(n).or_insert(0);
        *count += 1;
    }

    let mut max_value = numbers[0];
    let mut max_i: usize = 0;

    for (i, value) in counts {
        if value > max_value {
            max_i = usize::try_from(*i).expect("Should be able to convert to usize");
            max_value = value;
        }
    }

    numbers[max_i]
}
