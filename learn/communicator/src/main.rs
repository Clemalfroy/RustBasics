extern crate communicator;

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

use a::series::of;

fn main() {
    communicator::client::connect();
    of::nested_modules();

    use TrafficLight::{Red, Yellow};
    let _red = Red;
    let _yellow = Yellow;
    let _green = TrafficLight::Green;

    use TrafficLight::*;
    let _red = Red;
    let _yellow = Yellow;
    let _green = Green;

}