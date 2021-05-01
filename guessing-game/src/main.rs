use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let (min, max) : (i32, i32) = (1, 100);
    
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(min, max + 1);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number between {} and {}.", min, max);
                continue;
            }
        };

        println!("You guessed: {}", guess);

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
