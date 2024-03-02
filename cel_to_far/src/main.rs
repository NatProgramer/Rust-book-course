fn main() {
    let fahrenheit_temp: f32 = celsius_to_fahrenheit(-273.15);
    let celcius_temp: f32 = fahrenheit_to_celsius(fahrenheit_temp);
    
    println!("Celsius to fahrenheit convertion start");
    println!("{}°F", fahrenheit_temp);

    println!("Fahrenheit to celsius convertion start");
    println!("{}°C", celcius_temp);
}

fn celsius_to_fahrenheit(temperature: f32) -> f32 {
    let fahrenheit = (temperature as f32) * 1.8 + 32.0;
    fahrenheit
}

fn fahrenheit_to_celsius(temperature: f32) -> f32 {
    let celsius = ((temperature as f32) - 32.0) / 1.8;
    celsius
}
