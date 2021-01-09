use std::f32::consts::PI;

fn main() {
  let rectangle = Rectangle {
    width: 12.0,
    height: 12.0,
  };
  println!("rectangle area {}", rectangle.area());

  let circle = Circle {
    r: 32.5,
  };
  println!("circle area {}", circle.area());

  let triangle = Triangle {
    side: 12.5,
    height: 32.0,
  };
  println!("triangle area {}", triangle.area());
}

struct Rectangle {
  width: f32,
  height: f32,
}

struct Circle {
  r: f32,
}

struct Triangle {
  side: f32,
  height: f32,
}

pub trait Area {
  fn area(&self) -> f32;
}

impl Area for Rectangle {
  fn area(&self) -> f32 {
    self.width * self.height
  }
}

impl Area for Circle {
  fn area(&self) -> f32 {
    PI * self.r.powi(2)
  }
}

impl Area for Triangle {
  fn area(&self) -> f32 {
    self.side * self.height / 2.0
  }
}
