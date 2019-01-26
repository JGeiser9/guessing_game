use std::io; // Import input/output from the standard Library
use std::cmp::Ordering;
use rand::Rng; // Import the range from the rand crate

fn main() {
    // Prompting the user with messages of how to play the game
    println!("<< --- Number Guessing Game --- >>");

    /* rand::thread_rng() is the random number generator we want to use
       .gen_range(1, 101) generates a random number and stores it as 'secret_number'
       (lower, upper) inclusive on lower bound / exlusive on upper bound gives us 1 - 100 */
    let secret_number = rand::thread_rng().gen_range(1,101);

    // Loop until the user guesses correctly
    loop {
        // println!("The secret number is: {}", secret_number);
        println!("Please input your guess.");

        // Mutable variable declaration as a new empty string type
        let mut guess = String::new();

        // Get the input from user, read it, and store it as a string in our mutable guess variable
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); // Error handling

        /* guess = the original guess String from user
           trim = eliminates any whitespace and new lines (enter key) from user input
           parse = convert to number
           u32 = convert to a 32-bit number
           Any inputs that aren't numbers will trigger an error */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // Allow guesses that are numbers
            Err(_) => continue, // Ignore non-number guesses
        };

        // {} is a placeholder for the user's guess which is stored in the variable 'guess'
        println!("You guessed: {}", guess);

        // Compare guess to our secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Only exit the game if guessed correctly
            }
        }
    }
}
