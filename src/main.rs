use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guesses = 0;

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        
        let guess = loop {

            let mut guess = String::new();
            io::stdin()
            .read_line(&mut guess)
            .expect("Something has gone very wrong; terminating.");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("That's not a number, you silly goose!\nTry again :)");
                    continue;
                }
            };
            break guess;
        };

        guesses += 1;
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                if guesses == 1 {
                    println!("You got it in 1 guess! Lucky!");
                } else {
                    println!("You got it in {guesses} guesses.");
                }
                break;
            }
        }
    }
}
