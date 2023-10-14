// Traits
use std::f32::consts::PI;

trait Shape {
    fn new(length: f32, width: f32) -> Self;
    fn area(&self) -> f32;
}

struct Rectangle {
     length: f32,
     width: f32
}

struct Circle {
     length: f32,
     width: f32
}

// Implementing Shape trait for Rectangle struct
impl Shape for Rectangle {
     fn new(length: f32, width: f32) -> Rectangle {
         Rectangle { length, width }
     }

     fn area(&self) -> f32 {
         return self.length * self.width;
     }
}

// Implementing Shape trait for Circle struct
impl Shape for Circle {
     fn new(length: f32, width: f32) -> Circle {
         Circle { length, width }
     }

     fn area(&self) -> f32 {
         return PI * self.length * self.width;
     }
 }

fn main() {
    let rectangle: Rectangle = Rectangle::new(3.0, 4.0);
    let circle: Circle = Circle::new(2.0, 2.0);

    println!("Area of the rectangle and circle is {} and {}, respectively", rectangle.area(), circle.area());
}
