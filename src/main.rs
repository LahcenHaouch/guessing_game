use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    let mut lives = 5;
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        if lives == 0 {
            println!("Game Over");
            break;
        }

        println!("LIVES: {lives}");
        println!("Please input your guess.");

        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                lives -= 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                lives -= 1;
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
