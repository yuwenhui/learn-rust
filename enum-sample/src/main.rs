fn main() {
  let _light = TrafficLight::Red;
  let _light = TrafficLight::Green;
  let light = TrafficLight::Yellow;
  println!("light is {}", light.time());
}

enum TrafficLight {
  Red,
  Green,
  Yellow,
}

impl TrafficLight {
  fn time(&self) -> u8 {
    60
  }
}

/*
enum Option<T> {
  Some(T),
  None,
}

enum Result<T, E> {
  Ok(T),
  Err(E),
}
*/