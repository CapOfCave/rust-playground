use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("IO error.");
    let n: u8 = n.trim().parse().expect("Error parsing value");
    for i in 1..=n {
        print_verse(i);
        println!();
    }
}

fn print_verse(n: u8) {
    let number_words = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    println!(
        "On the {} day of Christmas my true love sent to me",
        number_words[n as usize - 1]
    );
    if n == 1 {
        println!("A partridge in a pear tree.")
    } else {
        print_verse_line(n);
    };
}

fn print_verse_line(n: u8) {
    let out = match n {
        1 => "And a partridge in a pear tree.",
        2 => "Two turtle doves,",
        3 => "Three French hens,",
        4 => "Four calling birds,",
        5 => "Five gold rings,",
        6 => "Six geese a-laying,",
        7 => "Seven swans a-swimming,",
        8 => "Eight maids a-milking,",
        9 => "Nine ladies dancing,",
        10 => "Ten lords a-leaping,",
        11 => "Eleven pipers piping,",
        12 => "Twelve drummers drumming,",
        _ => return,
    };
    println!("{}", out);

    if n > 0 {
        print_verse_line(n - 1);
    }
}
