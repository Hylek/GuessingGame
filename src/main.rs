use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess The Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please Input Your Guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too small!"),
            Ordering::Equal => println!("Correct! You Win!"),
            Ordering::Greater => {
                println!("Correct! You Win!");
                break;
            }
        }

        println!("You Guessed: {guess}");
    }
}
