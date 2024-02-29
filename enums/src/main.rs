#[derive(Debug)] // This trait make posible print a value of this enum to console with println
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    println!("Hello, world!");
    enum_basics()
}

fn enum_basics() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    println!("IP adress V4: {:?}", home);
    println!("IP adress V6: {:?}", loopback);
}
