<<<<<<< HEAD

#[derive(Debug)]
=======
#[derive(Debug)] // This trait make posible print a value of this enum to console with println
#[warn(dead_code)]
>>>>>>> 3cb3196 (Upstream to ssh)
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    println!("Hello, world!");
<<<<<<< HEAD
=======

    enum_basics()
}

fn enum_basics() {
>>>>>>> 3cb3196 (Upstream to ssh)
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    println!("IP adress V4: {:?}", home);
    println!("IP adress V6: {:?}", loopback);
}
