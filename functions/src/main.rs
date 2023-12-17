//* Functions that return a value
fn main() {
    let x = five();

    println!("The value of x is: {x}");
    //* This function is of parameters section
    parameters();
    //* This function is of a small section that explains a diferent case for "five()" function
    some_number(10);
}

//? This function return the number five as value and this value can be save in a variable inside other function like "main" for example in this case
fn five() -> i32 {
   5
}
// Note: this function you can declare variables, receive parameters and perform operations and could still return the value, like this:
fn some_number(num: i32) -> i32 {
    let number = num + 5;
    // Assuming "num" is equal to 10 then this value would be equal to 15
    number 
}


//* Functions with parameters
fn parameters() {
    println!("Hello, world!");

    sum(255, 255)
}

//? This function accept two params (a: u16, b: u16) and can be used inside of the function
fn sum(a: u16, b: u16) {
    println!("the result of sum {a} + {b} = {}", a + b);
}
