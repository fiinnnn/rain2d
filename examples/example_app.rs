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
        true);

    core.run(&mut ExampleApp {});
}

// can be used to store application state
struct ExampleApp;

impl RainApp for ExampleApp {
    fn on_update(&mut self, rain: &mut RainCore, _dt: Duration) {
        // drawing
        rain.fill_triangle(vec2(120, 300), vec2(520, 300), vec2(320, 100), WHITE);

        // keyboard input
        // gets all keys that are currently down
        if let Some(keys) = rain.get_keys() {
            for key in keys {
                match key {
                    Key::Space => println!("Spacebar down"),
                    Key::A => println!("A down"),
                    _ => (),
                }
            }
        }

        // only true on keypress, doesn't repeat
        if rain.key_pressed(Key::Key1) {
            println!("1 pressed");
        }

        // mouse input
        if rain.mouse_button_down(MouseButton::Left) {
            if let Some(pos) = rain.get_mouse_pos() {
                println!("Mouse x: {}, Mouse y: {}", pos.x, pos.y);
            }
        }
    }
}