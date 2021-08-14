use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut secret_number: u32 = rand::thread_rng().gen_range(1, 101);
    let mut wins: i32 = 0;
    let mut guesses: i32 = 0;

    println!("Guess the number");

    loop {
        println!("Please input your number.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        guesses = &guesses + 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} is too small!", guess),
            Ordering::Greater => println!("{} is too big!", guess),
            Ordering::Equal => {
                wins = &wins + 1;

                println!("{}, You win!", guess);

                let mut keep_playing: String = String::new();

                println!("Keep playing? (y or n)");

                io::stdin()
                    .read_line(&mut keep_playing)
                    .expect("Please type y or n");

                if keep_playing.trim() != "y" {
                    println!("You won {} times and with {} total guesses", wins, guesses);
                    println!("cya!");
                    break;
                }

                secret_number = rand::thread_rng().gen_range(1, 101);
            }
        }
    }
}
