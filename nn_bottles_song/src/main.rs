use std::io;

fn main() {
    println!("How many bottles?");

    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line!");
    let number: u32 = number.trim().parse().expect("Cannot parse number!");

    println!("\n***");
    for bottles in (2..=number).rev() {
        println!("{bottles} bottles of beer on the wall, {bottles} bottles of beer");
        let bottles_left = bottles - 1;
        println!("Take one down pass it around, {bottles_left} bottles of beer on the wall\n");
    }

    println!(
        "1 bottle of beer on the wall, 1 bottle of beer\n\
        Take one down and pass it around, no bottles of beer on the wall!"
    );
}
