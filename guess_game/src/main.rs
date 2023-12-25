use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
    println!("Guess the number: ");

        let mut guess = String::new();

    io::stdin()

        .read_line(&mut guess)
        .expect("Error while reading the number");

    let guess: i32 = guess.trim().parse().expect("Failed to parse the string");

    match guess.cmp(&secret_number) {

        Ordering::Less => println!("Secret number is greater than the guessed"),
        Ordering::Greater => println!("Secret number is lesser than the guessed"),
        Ordering::Equal => {
            println!("You guessed the correct number!");
            break;
        }
    }

  }

}

