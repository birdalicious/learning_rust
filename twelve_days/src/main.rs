fn main() {
    let gifts = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves and",
        "A partridge in a pear tree",
    ];

    for i in 1..=12usize {
        println!(
            "On the {}{} day of Christmas, my true love sent to me", 
            i, ordinal_indicator(i)
        );

        for j in 12-i..12 {
            println!("{}", gifts[j]);
        }   

        println!("");
    }
}

fn ordinal_indicator(n: usize) -> String {
    match n {
        1 => String::from("st"),
        2 => String::from("nd"),
        3 => String::from("rd"),
        _ => String::from("th"),
    }
}
