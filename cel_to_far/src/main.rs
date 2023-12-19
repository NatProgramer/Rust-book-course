fn main() {
    let farenheit_temp = celsius_to_farenheit(35);
    println!("{}°F", farenheit_temp);
}

fn celsius_to_farenheit(temperature: i32) -> i32 {
    let farenheit = temperature * 2 + 32;
    farenheit
}