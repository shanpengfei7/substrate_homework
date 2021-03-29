use std::f32::consts::PI;

fn main() {
    let shape = Rectangle {
        width: 10.0,
        height: 8.0,
    };
    println!("rectangle's area = {}", calculate(&shape));

    let shape = Square { width: 7.0 };
    println!("square's area = {}", calculate(&shape));

    let shape = Round { radius: 2.0 };
    println!("round's area = {}", calculate(&shape));
}

// 泛型 和 泛型约束 实现通用的调用类，只需要传入一个形状对象就可以计算面积了
fn calculate<T: Shape>(shape: &T) -> f32 {
    shape.area()
}

// 定义形状的trait
trait Shape {
    fn area(&self) -> f32;
}

// 定义一个长方形
struct Rectangle {
    width: f32,
    height: f32,
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

// 定义一个正方形
struct Square {
    width: f32,
}

impl Shape for Square {
    fn area(&self) -> f32 {
        self.width * self.width
    }
}

// 定义一个圆
struct Round {
    radius: f32,
}

impl Shape for Round {
    fn area(&self) -> f32 {
        PI * self.radius * self.radius
    }
}
