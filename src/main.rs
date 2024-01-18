use rand::Rng;
use std::{cmp::Ordering, io, u16};

fn main() {
    let secret_number: u16 = rand::thread_rng().gen_range(1..=50);
    let write_a_number_message: String = "Write a number between 1 and 50".to_string();

    println!("Guess my number!");
    println!("{write_a_number_message}:");

    loop {
        let end_game: bool = guess_number(&secret_number);
        if end_game {
            break;
        }
    }
}

fn guess_number(secret_number: &u16) -> bool {
    let mut guess = String::new();
    match io::stdin().read_line(&mut guess) {
        Ok(_) => {}
        Err(error) => {
            guess = error.to_string();
        }
    }

    // uses "shadowing"
    let guess = match guess.trim().parse::<u16>() {
        Ok(parsed) => parsed,
        Err(_) => {
            // greets user for trying to fuck with us
            println!(
                "Very funny motherfucker...\nDick. Input a fucking number IN THE RANGE! 1 -> 50!"
            );

            //this return from the function (guess_number)
            return false;
        }
    };

    println!("You guessed: {guess}");
    println!("My number is: {secret_number}");

    let guessing_result: bool = match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("\nyour number is lower!\n");
            println!("GO AGANE");
            false
        }
        Ordering::Greater => {
            println!("\nyour number is too high...\n");
            println!("GO AGANE");
            false
        }
        Ordering::Equal => {
            println!("\nyou aced it, motherfucker!");
            true
        }
    };

    return guessing_result;
}
