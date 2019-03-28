use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut number_of_guesses = 1;
    let input = io::stdin();

    println!("Welcome to guessing game! Type 'exit' to exit at any time.");

    loop {
        println!("Enter a number: ");

        let mut guess = String::new();

        input.read_line(&mut guess).expect("Failed to read line!");

        if guess.trim() == "exit" {
            println!("Bye!");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("That wasn't a number!");
                continue
            }
        };

        println!("Your guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations! You found the secret number in {} guesses.", number_of_guesses);
                break;
            }
        }

        number_of_guesses += 1;
    }
}
