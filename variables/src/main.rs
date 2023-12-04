fn main() {
    let x: u8 = 15;

    println!("");
    println!("The initial value is: {x}");
    println!("");
    println!("The value with sum of x is: {}", x + 5);
    println!("The value with rest of x is: {}", x - 5);
    println!("The value with multiply of x is: {}", x * 5);
    println!("The value with divition of x is: {}", x / 5);
    println!("The rest of x is: {}", x % 5);
    println!("");

    shadows();
    mutablity();   
}

fn shadows() {
    let x: u8 = 15;
    
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    
    println!("The value of x is: {}", x + 1);
    println!("")
}

fn mutablity() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("")
}