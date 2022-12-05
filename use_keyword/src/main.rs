pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}


//we should not add prefix of 'nested_modules'
use a::series::of::nested_modules;

// use TrafficLight::{Red, Yellow};

// can use glob(*) to call all items
use TrafficLight::*;

fn main() {
    nested_modules();
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
}
