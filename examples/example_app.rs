use std::time::Duration;

use rain2d::{
    core::*,
    math::vec2
};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut core = RainCore::init("example app",
        WIDTH,
        HEIGHT,
        true).unwrap();

    core.run(&mut ExampleApp {});
}

// can be used to store application state
struct ExampleApp;

impl RainApp for ExampleApp {
    fn on_update(&mut self, rain: &mut RainCore, _dt: Duration) {
        rain.fill_triangle(vec2(120, 300), vec2(520, 300), vec2(320, 100), WHITE);
    }
}