struct Rectangle { base: f32, height: f32 }
fn main() {

    let rectangle1 = &Rectangle {
        base: 40.0,
        height: 30.0,
    };

    println!("{}", find_rectangle_area(
        rectangle1,
        "m"
    ));
}

fn find_rectangle_area(rectangle: &Rectangle, measure: &str) -> String {
    let area: String = format!("{} {measure}", rectangle.base * rectangle.height);

    area
}