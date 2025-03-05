use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // generate a random number between 1 - 100
    let secret_number = rand::rng().random_range(1..=100);
    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // or skip import with std::io::stdin
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadow the guess variable to alter it's type
        let guess: u32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
              println!("That's it!");
              break;
            },
        }
    }
}
