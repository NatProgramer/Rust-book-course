use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let ramdon_num = rand::thread_rng().gen_range(1..=100);

    println!("");
    println!("Guess the number");
    println!("");

    loop {
        println!("Please input a number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("");
        println!("You writed: {guess}");

        match guess.cmp(&ramdon_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("");
    }
}
