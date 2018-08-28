#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
#[derive(Debug)]
struct Triangle {
    base: f64,
    height: f64,
}

impl Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

#[derive(Debug)]
enum Shape {
    Rect(Rectangle),
    Circle(Circle),
    Triangle(Triangle),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Rect(x) => x.area(),
            Shape::Circle(x) => x.area(),
            Shape::Triangle(x) => x.area(),
        }
    }
}

fn main() {

    let rectangle = Shape::Rect(Rectangle {
        width : 4.23,
        height: 5.0,
    });
    println!("{:?}", rectangle.area());
}