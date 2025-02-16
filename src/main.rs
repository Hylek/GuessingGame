use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess The Number!\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please Input Your Guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!\n");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too small!\n"),
            Ordering::Greater => println!("Your guess is too big!\n"),
            Ordering::Equal => {
                println!("Correct! You Win!");
                break;
            }
        }
    }
}
