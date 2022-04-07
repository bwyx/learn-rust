use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut chances = 5;
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        if chances == 0 {
            println!("You lost! the number was {}", secret_number);
            break;
        }

        println!("Please input your guess. {} chances left.", chances);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Rust allow shadowing of variables. wow!,
        // for make `cmp` to work, we need to use same type of variable
        // this infer guess to u32 (same as secret_number)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Compare guess to secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };

        chances -= 1;
    }
}
