fn main() {
  let red = Light {
    name: TrafficLight::Red
  };
  let green = Light {
    name: TrafficLight::Green
  };
  let yellow = Light {
    name: TrafficLight::Yellow
  };

  println!("red time is: {}", red.time());
  println!("green time is: {}", green.time());
  println!("yellow time is: {}", yellow.time());
}

const RED_TIME: u8 = 60;
const GREEN_TIME: u8 = 90;
const YELLOW_TIME: u8 = 3;

enum TrafficLight {
  Red,
  Green,
  Yellow,
}

struct Light {
  name: TrafficLight,
}

trait TrafficTime {
  fn time(&self) -> u8;
}

impl TrafficTime for Light {
  fn time(&self) -> u8 {
    match self.name {
      TrafficLight::Red => RED_TIME,
      TrafficLight::Green => GREEN_TIME,
      TrafficLight::Yellow => YELLOW_TIME,
    }
  }
}
