use std::io;
use rand::Rng;


fn main() {
    loop {
        println!("Guess the number!");
        println!("Please input your guess.");

        let mut guess = String::new();
        let number = rand::thread_rng().gen_range(1..=100);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if (guess.trim() == "quit") {
            println!("Exiting the game. Goodbye!");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if (guess == number) {
            println!("You guessed the number!");
            break;
        } else {
            println!("Wrong guess, the number was: {number}");
        }
    }
}