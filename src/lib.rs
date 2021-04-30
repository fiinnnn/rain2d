#![warn(missing_docs)]
//! Simple 2d framework/engine
//!
//! Provides some utilities to draw basic shapes,
//! might turn into an actual game engine at some point
//!
//! ### Example
//!
//! ```no_run
//! use std::time::Duration;
//!
//! use rain2d::core::*;
//!
//! const WIDTH: usize = 640;
//! const HEIGHT: usize = 360;
//!
//! // can be used to store application state
//! struct ExampleApp;
//!
//! impl RainApp for ExampleApp {
//!     fn on_update(&mut self, rain: &mut RainCore, dt: Duration) {
//!         // drawing
//!         rain.fill_triangle(120, 300, 520, 300, 320, 100, WHITE);
//!
//!         // keyboard input
//!         // gets all keys that are currently down
//!         if let Some(keys) = rain.get_keys() {
//!             for key in keys {
//!                 match key {
//!                     Key::Space => println!("Spacebar down"),
//!                     Key::A => println!("A down"),
//!                     _ => (),
//!                 }
//!             }
//!         }
//!
//!         // only true on keypress, doesn't repeat
//!         if rain.key_pressed(Key::Key1) {
//!             println!("1 pressed");
//!         }
//!
//!         // mouse input
//!         if rain.mouse_button_down(MouseButton::Left) {
//!             if let Some((x, y)) = rain.get_mouse_pos() {
//!                 println!("Mouse x: {}, Mouse y: {}", x, y);
//!             }
//!         }
//!     }
//! }
//!
//! let mut core = RainCore::init("example app",
//!     WIDTH,
//!     HEIGHT,
//!     true);
//!
//! core.run(&mut ExampleApp {});
//! ```

pub mod core;
