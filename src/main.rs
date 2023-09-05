use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is:{secret_number}");
    loop {
        println!("Please input your guess.");
        let mut guess = String::new(); // to store the user input

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // shadowing "guess".
           // Ignoring a non-number guess and asking for another guess instead of crashing the program

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("*****You win!*****");
                break;
            }
        }
    }

    // let x = 5;
    // let y = 10;
    // println!("x = {x} and y + 2 = {}", y + 2);
}
