use std::io;

fn main() {
    println!("Enter a sentence to convert to pig latin:");

    let mut string = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Could not read line");

    println!("{}", pig_latin(&string));
}

// todo: check for punctuation 
fn pig_latin(string: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut pig = String::new();

    for word in string.split_ascii_whitespace() {
        let first_letter = word.chars()
            .next()
            .unwrap()
            .to_ascii_lowercase();

        if vowels.contains(&first_letter) {
            pig.push_str(&format!("{}-hay", word));
        }
        else {
            let index = word.chars()
                .position(|c| vowels.contains(&c))
                .unwrap_or(0);

            let (start, end) = word.split_at(index);
            pig.push_str(&format!("{}-{}ay", end, start));
        }

        pig.push(' ');
    }

    pig.pop();
    pig
}