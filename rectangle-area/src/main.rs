fn main() {
    struct Rectangle { base: f32, height: f32 }

    let rectangle1 = Rectangle {
        base: 40.0,
        height: 30.0,
    };

    println!("{}", find_rectangle_area(
        rectangle1.base, 
        rectangle1.height,
        "m"
    ));
}

fn find_rectangle_area(base: f32, height: f32, measure: &str) -> String {
    let area: String = format!("{} {measure}", base * height);

    area
}