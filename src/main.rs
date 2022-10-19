use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
    loop {
        println!("Input your guess!");
        let mut guess  = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("The number is {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small. The random number was {secret_number}"),
            Ordering::Greater => println!("Too big. The random number was {secret_number}"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
