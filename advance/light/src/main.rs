use std::time::Duration;

fn main() {
    let light = TrafficLight::Green(Duration::from_secs(60));
    println!("green time is {}", light.time().as_secs());
    let light = TrafficLight::Red(Duration::from_secs(45));
    println!("red time is {}", light.time().as_secs());
    let light = TrafficLight::Yellow(Duration::from_secs(5));
    println!("yellow time is {}", light.time().as_secs());
}

enum TrafficLight {
    Red(Duration),
    Green(Duration),
    Yellow(Duration),
}

trait LightTrait {
    fn time(&self) -> &Duration;
}

impl LightTrait for TrafficLight {
    fn time(&self) -> &Duration {
        match self {
            TrafficLight::Red(t) => t,
            TrafficLight::Green(t) => t,
            TrafficLight::Yellow(t) => t,
        }
    }
}
