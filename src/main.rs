use std::{io, ops::RangeInclusive};
use rand::Rng;
use std::cmp::Ordering;

const RANGE: RangeInclusive<i32> = 1..=100;

fn main() {
    println!("Guess the number!\nRange: {}~{}", RANGE.start(), RANGE.end());

    let secret_number = rand::thread_rng().gen_range(RANGE);
    let mut guesses = 0;

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        
        let guess = loop {

            let mut input = String::new();
            io::stdin()
            .read_line(&mut input)
            .expect("Something has gone very wrong; terminating.");

            let input: i32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("That's not a number, you silly goose!\nTry again :)");
                    continue;
                }
            };
            if !(RANGE.contains(&input)) {
                println!("That number is not in range!\nTry a number between {} and {}.", RANGE.start(), RANGE.end());
                continue;
            }
            break input;
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
                } if guesses > RANGE.count().ilog2() + 2 {
                    // Advice, but with a bit of leeway to account for rounding and human error before chastity
                    println!("Consider employing a binary search alogrithm next time :)");
                }
                break;
            }
        }
    }
}
