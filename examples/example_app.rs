use std::time::Duration;
use nalgebra_glm::IVec2;

use rain2d::*;

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
        rain.fill_triangle(IVec2::new(120, 300), IVec2::new(520, 300), IVec2::new(320, 100), WHITE);
    }
}