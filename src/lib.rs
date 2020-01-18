#![warn(missing_docs)]
//! Simple 2d framework/engine
//!
//! Provides some utilities to draw basic shapes,
//! might turn into an actual game engine at some point
//!
//! ### Math
//! [nalgebra-glm](https://nalgebra.org/) is used as the math library and reexported under the
//! `rain2d::math` namespace
//!
//! For documentation see [the official nalgebra-glm documentation](https://nalgebra.org/rustdoc_glm/nalgebra_glm/index.html)
//!
//! ### Example
//!
//! ```no_run
//! use std::time::Duration;
//!
//! use rain2d::{
//!     core::*,
//!     math::vec2y
//! };
//!
//! const WIDTH: usize = 640;
//! const HEIGHT: usize = 360;
//!
//! // can be used to store application state
//! struct ExampleApp;
//!
//! impl RainApp for ExampleApp {
//!     fn on_update(&mut self, rain: &mut RainCore, dt: Duration) {
//!         rain.fill_triangle(vec2(120, 300), vec2(520, 300), vec2(320, 100), WHITE);
//!     }
//! }
//!
//! let mut core = RainCore::init("example app",
//!     WIDTH,
//!     HEIGHT,
//!     true).unwrap();
//!
//! core.run(&mut ExampleApp {});
//! ```

pub use nalgebra_glm as math;

pub mod core;