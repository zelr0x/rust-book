use std::cmp::Ordering;
use std::io::{self, Write};
use rand::Rng;

fn main() {
    let opts = GameOpts::default();
    let target_number = rand::thread_rng()
        .gen_range(opts.start, opts.end + 1);
    println!("[Debug]: target number is: {}", target_number);

    loop {
        let guess = get_guess(&opts);
        match guess.cmp(&target_number) {
            Ordering::Less => print!("Too small! Wanna try again? "),
            Ordering::Greater => print!("Too big! Wanna try again? "),
            Ordering::Equal => {
                println!("Congratz, you win!");
                std::process::exit(0);
            }
        };
    }
}

fn get_guess(opts: &GameOpts) -> i32 {
    let start = opts.start;
    let end = opts.end;

    loop {
        print!("Guess a number between {} and {}: ", start, end);
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");

        match guess.trim().parse::<i32>() {
            Err(_) => continue,
            Ok(guess) => {
                if guess >= start && guess <= end {
                    return guess
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct GameOpts {
    start: i32,
    end: i32,
}

impl Default for GameOpts {
    fn default() -> Self {
        GameOpts { start: 1, end: 100 }
    }
}
