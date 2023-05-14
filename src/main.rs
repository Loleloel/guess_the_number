use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // Generate a random number between 1 - 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Loop the game until we get the correct answer.
    loop {
        // Print line macro.
        println!("Guess a number between 1 - 100.");

        // New empty MUTABLE string to take user input.
        let mut guess = String::new();

        // Take user input, read line and append to guess variable.
        io::stdin()
            // guess variable takes the size of the user input.
            .read_line(&mut guess)
            // If something goes wrong, we crash the program and tell user a message.
            .expect("Failed to read line.");

        // Parse input from a string to an unsigned int32 type.
        let guess: u32 = match guess.trim().parse() {
            // If OK, proceed as usual.
            Ok(num) => num,
            // If not OK, restart loop.
            Err(_) => continue,
        };

        // Compare the guess with the secret number using the cmp from standard library.
        match guess.cmp(&secret_number) {
            // If guess is less than secret_number, write "Too low."
            Ordering::Less => println!("Too low."),
            // If guess is greater than secret_number, write "Too big."
            Ordering::Greater => println!("Too big."),
            // If guess is equal to secret_number, tell user he/she wins and break loop.
            Ordering::Equal => {
                println!("You win! The number was {secret_number}!");
                break;
            }
        }
    }
}
