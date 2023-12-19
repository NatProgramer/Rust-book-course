fn main() {
    //? Execute infinite loop
    infinite_loop();

    //? Execute a loop were we return a value
    returning_value();
}

//? In this function we start a infinite loop using the reserved word "loop"
fn infinite_loop() {
    loop {
        println!("again!");
    }
}
//? In this function we return a value that is result of the loop and then we use that value to save inside of a variable
fn returning_value() {
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
    
        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is {result}");
}