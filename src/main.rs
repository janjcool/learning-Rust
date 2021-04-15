use std::io;
use std::cmp::Ordering;
use rand::Rng;

mod cat_game;
use cat_game::cat_game;

fn main() {
    println!("Hello, world!");
    println!("This is a program were I learn rust");
    println!("Choose the program you want to run");

    loop {
        println!("Enter the desired program from the following options:");
        println!("1) Guessing game\n2) Cat Game\n3)\n4) Quit");

        // Get input the program for the user
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Check if exit is typed
        if input.trim().to_ascii_lowercase() == "exit" {break} else {};

        // Notify user of there chose
        println!("You entered: {}", input);

        // Convert input to user
        let input: i8 = input.trim().parse().expect("Please type a number!");

        match input {
            1 => guessing_game(),
            2 => cat_game(),
            4 => break,
            _ => println!("This is not an option"),
        }
    }
    println!("Quiting ...");
}

fn guessing_game () {
    println!("Staring guessing game");
    println!("I choose a secret number between 1 and 101. Try to guess the number.");

    let secret_number: i8 = rand::thread_rng().gen_range(1..101); // Random number in range
    println!("The secret number is {}", secret_number);

    loop {
        // Get guess for user
        let mut guess: String = String::new(); // Make variable
        println!("Please input your guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Check if exit is typed
        if guess.trim().to_ascii_lowercase() == "exit" {
            println!("Quiting ...");
            break;
        } else {};

        println!("You guessed: {}", guess);

        // Convert String to int
        let guess: i8 = match guess.trim().parse() {
            Ok(num) => if num >= 1 && num < 101 {num} else {
                println!("Number is out of range");
                continue;
            },
            Err(_) => {
                println!("This is not a valid input");
                continue;
            },
        };

        // Compare the secret number with the guess
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your number is smaller"),
            Ordering::Greater => println!("Your number is bigger"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}
