use rand::{thread_rng, Rng};
use std::process::exit;
use std::env;

fn main() {
    // let _args = env::args().collect::<Vec<String>>();

    gen_password(true, true, true, true, 40);
}

fn gen_password(minus: bool, mayus: bool, nums: bool, symbols: bool, length: u8) -> String {
    if length > 60 {
        println!("The lenght can't be greater than 60");
        exit(1)
    }

    let mut password: String = String::from("");

    for _character in 0..length {
        let mut characters = Vec::new();

        if minus == true {
            characters.push("abcdefghijklmnopqerstuvwxyz")
        }

        if mayus == true {
            characters.push("ABCDEFGHIJKLMNOPQERSTUVWXYZ")
        }

        if nums == true {
            characters.push("1234567890")
        }

        if symbols == true {
            characters.push("`~!@#$%^&*()_+[{}];:,.><?/'")
        }

        if !minus && !mayus && !nums && !symbols {
            characters.push("abcdefghijklmnopqerstuvwxyz")
        }

        password.push(gen_char(characters))
    }
    
    println!("{}", password);
    password
}

fn gen_char (characters: Vec<&str>) -> char {
    let char_type = characters[thread_rng().gen_range(0..characters.len())];

    char_type.chars()
    .nth(thread_rng().gen_range(0..char_type.len()))
    .unwrap()
}