use std::process::exit;

struct Rectangle { 
    base: u32,
    height: u32 
}

impl Rectangle {
    fn area(&self, measure: &str) -> String {
        if measure.len() == 0 {
            eprintln!(
                "Error: You try calculate area of Rectangle without measure"
            );
            exit(0)
        }

        format!("{}{measure}", self.base * self.height)
    }
}

fn main() {
    let rectangle1 = Rectangle {
        base: 10,
        height: 20,
    };

    let rectangle2 = Rectangle {
        base: 30,
        height: 40,
    };

    let rectangle3 = Rectangle {
        base: 35,
        height: 45,
    };

    let rectangle4 = Rectangle {
        base: 50,
        height: 65,
    };

    println!("{}", rectangle1.area("mm"));
    println!();

    println!("{}", rectangle2.area("cm"));
    println!();

    println!("{}", rectangle3.area("m"));
    println!();

    println!("{}", rectangle4.area("km"));

}
