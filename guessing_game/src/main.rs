use rand::Rng;
use std::cmp::Ordering;
use std::io;

// this code was made following chapter 2 from the "rust programming language book"
// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

fn main() {
    // println! its a macro to diplay a string in the stardard output
    println!("Guess the number!");

    // 1..=100 its a way of writing a inclusive range, in this case a range
    // that from 0 to 100 including 0 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("the secret number is:{secret_number}");

    loop {
        println!("Please input your guess.");

        // mut its a keyword tha defines a muttable variable.
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // the parse method traform a string value to another type, rust can infer types
        // but in this case we need to tell rust what type we want because there are many
        // possibilities 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // the match statement work similar to switch on c language, it compares a value
        // with many possibilities called "arms", and execute the code in the arm
        // that matchs the value
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!!!!!!!!!!!!!");
                break;
            }
        }
    }
}
