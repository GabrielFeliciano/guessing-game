use rand::Rng;
use std::cmp;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            cmp::Ordering::Greater => println!("Too big!"),
            cmp::Ordering::Less => println!("Too small!"),
            cmp::Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
