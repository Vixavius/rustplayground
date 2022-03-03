use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess:");

    // thread_rng is local to thread and seeded by OS
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        // Growable UTF-8 String (alloc::string::String), intialized empty with static method
        let mut guess = String::new();
        assert!(guess == "");
        assert!(guess == "".to_string());
        assert!(guess.len() == 0);

        io::stdin()
            .read_line(&mut guess) // Pass mutable reference to the String which read_line() updates
            .expect("Failed to read line."); // Crash on Err variant of Result

        println!("You've guessed: {}", guess);

        // Shadow the guest variable
        // Match to handle the error instead of crashing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Match all error values
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
