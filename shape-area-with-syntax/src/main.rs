use std::f64::consts::PI;

fn main() {
  let rectangle = Rectangle {
    width: 12_u8,
    height: 12_u8,
  };
  println!("rectangle area {}", rectangle.area());

  let circle = Circle {
    r: 32.5_f64,
  };
  println!("circle area {}", circle.area());

  let triangle = Triangle {
    side: 12,
    height: 32,
  };
  println!("triangle area {}", triangle.area());
}

struct Rectangle<T> {
  width: T,
  height: T,
}

struct Circle<T> {
  r: T,
}

struct Triangle<T> {
  side: T,
  height: T,
}

pub trait Area {
  fn area(&self) -> String;
}

impl <T: Copy + std::ops::Mul<Output = T> + std::fmt::Display> Area for Rectangle<T> {
  fn area(&self) -> String {
    format!("{}", self.width * self.height)
  }
}

impl <T: Copy + std::ops::Mul<Output = T> + std::fmt::Display + std::convert::Into<f64>> Area for Circle<T> {
  fn area(&self) -> String {
    format!("{}", self.r * self.r * PI)
  }
}

impl <T: Copy + std::ops::Mul<Output = T> + std::fmt::Display + std::convert::Into<f64>> Area for Triangle<T> {
  fn area(&self) -> String {
    format!("{}", self.side * self.height / 2.0)
  }
}
