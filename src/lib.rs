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
//! use nalgebra_glm::IVec2;
//!
//! use rain2d::*;
//!
//! const WIDTH: usize = 640;
//! const HEIGHT: usize = 360;
//!
//! // can be used to store application state
//! struct ExampleApp;
//!
//! impl RainApp for ExampleApp {
//!     fn on_update(&mut self, rain: &mut RainCore, dt: Duration) {
//!         rain.fill_triangle(IVec2::new(120, 300), IVec2::new(520, 300), IVec2::new(320, 100), WHITE);
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

use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::{Vec2, IVec2};
use std::{
    time::Duration,
    error::Error,
    time::Instant,
    mem::swap
};

pub use crate::color::*;
use crate::rendertarget::*;

pub mod color;
mod rendertarget;

#[allow(unused_variables)]
/// Trait used to call event functions from main loop
///
/// It's not required to implement any of these functions
/// although you probably want to or nothing will happen
///
/// ### Example
/// ```no_run
/// use rain2d::*;
///
/// struct App;
///
/// impl RainApp for App {
///     // setup
///     fn on_start(&mut self) {}
///
///     // main loop
///     fn on_update(&mut self, rain: &mut RainCore, dt: std::time::Duration) {}
///
///     // cleanup
///     fn on_exit(&mut self) {}
/// }
///
/// let mut core = RainCore::init("example app",
///     640,
///     360,
///     true).unwrap();
///
/// core.run(&mut App {});
/// ```
pub trait RainApp {
    /// Called once when the application starts
    ///
    /// Used to do any setup required by the main application
    fn on_start(&mut self) {}

    /// Called every frame
    ///
    /// `dt` is the time since the last update
    fn on_update(&mut self, rain: &mut RainCore, dt: Duration) {}

    /// Called before the application exits
    ///
    /// Used to clean up before exiting the main application
    fn on_exit(&mut self) {}
}

/// Engine state
pub struct RainCore {
    /// Sets if the application should exit when the escape key is pressed
    pub exit_on_esc: bool,

    active: bool,
    window_title: String,
    window: Window,
    screen_width: usize,
    screen_height: usize,
    render_target: RenderTarget,
    frame_timer: f32,
    frame_count: u32,
}

impl RainCore {
    /// Initializes the engine and opens a window with the specified dimensions
    ///
    /// Set `exit_on_esc` to true if you want the application to close when pressing escape
    ///
    /// Alternatively, you can control when to exit yourself by setting
    ///
    /// ### Example
    /// ```no_run
    /// use rain2d::*;
    ///
    /// let mut core = RainCore::init("example app",
    ///         640,
    ///         360,
    ///         true).unwrap();
    /// ```
    pub fn init(window_title: &str, width: usize, height: usize, exit_on_esc: bool) -> Result<Self, Box<dyn Error>> {
        let window = Window::new(window_title,
            width,
            height,
            WindowOptions::default())?;

        Ok(RainCore {
            exit_on_esc,
            active: true,
            window_title: window_title.to_string(),
            window,
            render_target: RenderTarget::new(width, height),
            screen_width: width,
            screen_height: height,
            frame_timer: 1.0,
            frame_count: 0,
        })
    }

    /// Starts the main loop
    ///
    /// This function won't return until the application is closed, use [`on_update`]
    /// to update the application state
    ///
    /// [`on_update`]: trait.RainApp.html#method.on_update
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::*;
    /// #
    /// struct ExampleApp;
    ///
    /// impl RainApp for ExampleApp {}
    ///
    /// # let mut core = RainCore::init("example app", 640, 360, true).unwrap();
    /// core.run(&mut ExampleApp {});
    /// ```
    pub fn run(&mut self, app: &mut dyn RainApp) {
        app.on_start();

        let mut last_time = Instant::now();
        while self.active {
            let current_time = Instant::now();
            let elapsed = current_time - last_time;
            last_time = current_time;

            // update state
            app.on_update(self, elapsed);

            // draw to screen
            self.window.update_with_buffer(&self.render_target.data,
                                           self.render_target.width,
                                           self.render_target.height).unwrap();

            // update frame count
            self.frame_timer += elapsed.as_secs_f32();
            self.frame_count += 1;
            if self.frame_timer >= 1.0 {
                self.frame_timer -= 1.0;
                let title = format!("{} - FPS: {}", self.window_title, self.frame_count);
                self.window.set_title(&title);
                self.frame_count = 0;
            }

            // check window status
            if !self.window.is_open() {
                self.active = false;
            }

            if self.exit_on_esc && self.window.is_key_down(Key::Escape) {
                self.active = false;
            }
        }

        app.on_exit();
    }

    /// Stops the main loop after the current frame has been drawn and calls [`on_exit`]
    ///
    /// ### Example
    ///```no_run
    ///# use rain2d::*;
    ///#
    ///# let mut core = RainCore::init("example app",
    ///#     640,
    ///#     360,
    ///#     true).unwrap();
    ///#
    ///# core.run(&mut App {});
    ///#
    ///# struct App;
    ///#
    ///# impl RainApp for App {
    ///  fn on_update(&mut self, rain: &mut RainCore, dt: std::time::Duration) {
    ///      rain.exit();
    ///      // exit doesn't immediately stop the application so anything here will get executed
    ///  }
    ///# }
    ///```
    /// [`on_exit`]: trait.RainApp.html#method.on_exit
    pub fn exit(&mut self) {
        self.active = false;
    }

    /// Draws a pixel if the location is in bounds after casting the coordinates to `i32`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::*;
    /// # use nalgebra_glm::Vec2;
    /// # let mut core = RainCore::init("example app", 640, 360, true).unwrap();
    /// core.draw_f(Vec2::new(10.0, 10.0), WHITE);
    /// ```
    pub fn draw_f(&mut self, pos: Vec2, color: Color) {
        self.render_target.set_pixel(pos.x as i32, pos.y as i32, color);
    }

    /// Draws a pixel if the location is in bounds
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::*;
    /// # use nalgebra_glm::IVec2;
    /// # let mut core = RainCore::init("example app", 640, 360, true).unwrap();
    /// core.draw(IVec2::new(10, 10), WHITE);
    /// ```
    pub fn draw(&mut self, pos: IVec2, color: Color) {
        self.render_target.set_pixel(pos.x, pos.y, color);
    }

    /// Draws a line from `p1` to `p2` after casting the coordinates to `i32`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::*;
    /// # use nalgebra_glm::Vec2;
    /// # let mut core = RainCore::init("example app", 640, 360, true).unwrap();
    /// core.draw_line_f(Vec2::new(10.0, 10.0), Vec2::new(100.0, 50.0), WHITE);
    /// ```
    pub fn draw_line_f(&mut self, p1: Vec2, p2: Vec2, color: Color) {
        self.draw_line(vec2_to_ivec2(p1), vec2_to_ivec2(p2), color);
    }

    /// Draws a line from `p1` to `p2`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::*;
    /// # use nalgebra_glm::IVec2;
    /// # let mut core = RainCore::init("example app", 640, 360, true).unwrap();
    /// core.draw_line(IVec2::new(10, 10), IVec2::new(100, 50), WHITE);
    /// ```
    pub fn draw_line(&mut self, p1: IVec2, p2: IVec2, color: Color) {
        let mut x1 = p1.x;
        let mut y1 = p1.y;
        let mut x2 = p2.x;
        let mut y2 = p2.y;
        let dx = i32::abs(x2 - x1);
        let dy = i32::abs(y2 - y1);

        // vertical line
        if dx == 0 {
            if y2 < y1 { swap(&mut y1, &mut y2); }
            for y in y1..y2 {
                self.draw(IVec2::new(x1, y), color);
            }
            return;
        }

        // horizontal line
        if dy == 0 {
            if x2 < x1 { swap(&mut x1, &mut x2); }
            for x in x1..x2 {
                self.draw(IVec2::new(x, y1), color);
            }
            return;
        }

        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = if dx > dy { dx / 2 } else { -dy / 2 };

        loop {
            self.draw(IVec2::new(x1, y1), color);
            if (x1 == x2 && y1 == y2)
                || x1 < 0 || x1 > self.screen_width as i32
                || y1 < 0 || y1 > self.screen_height as i32 {
                break;
            }
            if err > -dx { err -= dy; x1 += sx; }
            if err < dy { err += dx; y1 += sy; }
        }
    }

    /// Draws a circle at `pos` with radius `r` after casting the coordinates to `i32`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::*;
    /// # use nalgebra_glm::Vec2;
    /// # let mut core = RainCore::init("example app", 640, 360, true).unwrap();
    /// core.draw_circle_f(Vec2::new(100.0, 100.0), 10, WHITE);
    /// ```
    pub fn draw_circle_f(&mut self, pos: Vec2, r: i32, color: Color) {
        self.draw_circle(vec2_to_ivec2(pos), r, color);
    }

    /// Draws a circle at `pos` with radius `r`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::*;
    /// # use nalgebra_glm::IVec2;
    /// # let mut core = RainCore::init("example app", 640, 360, true).unwrap();
    /// core.draw_circle(IVec2::new(100, 100), 10, WHITE);
    /// ```
    pub fn draw_circle(&mut self, pos: IVec2, r: i32, color: Color) {
        let mut x0 = 0;
        let mut y0 = r;
        let mut d = 3 - 2 * r;
        if r <= 0 { return; }

        while y0 >= x0 {
            self.draw(IVec2::new(pos.x + x0, pos.y - y0), color);
            self.draw(IVec2::new(pos.x + y0, pos.y - x0), color);
            self.draw(IVec2::new(pos.x + y0, pos.y + x0), color);
            self.draw(IVec2::new(pos.x + x0, pos.y + y0), color);
            self.draw(IVec2::new(pos.x - x0, pos.y - y0), color);
            self.draw(IVec2::new(pos.x - y0, pos.y - x0), color);
            self.draw(IVec2::new(pos.x - y0, pos.y + x0), color);
            self.draw(IVec2::new(pos.x - x0, pos.y + y0), color);
            if d < 0 { d += 4 * x0 + 6; x0 += 1; }
            else { x0 += 1; y0 -= 1; d += 4 * (x0 - y0) + 10; }
        }
    }

    /// Draws a filled in circle at `pos` with radius `r` after casting the coordinates to `i32`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::*;
    /// # use nalgebra_glm::Vec2;
    /// # let mut core = RainCore::init("example app", 640, 360, true).unwrap();
    /// core.fill_circle_f(Vec2::new(100.0, 100.0), 10, WHITE);
    /// ```
    pub fn fill_circle_f(&mut self, pos: Vec2, r: i32, color: Color) {
        self.fill_circle(vec2_to_ivec2(pos), r, color);
    }

    /// Draws a filled in circle at `pos` with radius `r`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::*;
    /// # use nalgebra_glm::IVec2;
    /// # let mut core = RainCore::init("example app", 640, 360, true).unwrap();
    /// core.fill_circle(IVec2::new(100, 100), 10, WHITE);
    /// ```
    pub fn fill_circle(&mut self, pos: IVec2, r: i32, color: Color) {
        let mut x0 = 0;
        let mut y0 = r;
        let mut d = 3 - 2 * r;
        if r <= 0 { return; }

        while y0 >= x0 {
            self.draw_line(IVec2::new(pos.x - x0, pos.y - y0), IVec2::new(pos.x + x0, pos.y - y0), color);
            self.draw_line(IVec2::new(pos.x - y0, pos.y - x0), IVec2::new(pos.x + y0, pos.y - x0), color);
            self.draw_line(IVec2::new(pos.x - x0, pos.y + y0), IVec2::new(pos.x + x0, pos.y + y0), color);
            self.draw_line(IVec2::new(pos.x - y0, pos.y + x0), IVec2::new(pos.x + y0, pos.y + x0), color);
            if d < 0 { d += 4 * x0 + 6; x0 += 1; }
            else { x0 += 1; y0 -= 1; d += 4 * (x0 - y0) + 10; }
        }
    }

    /// Draws a rectangle at `pos` with `size` after casting the coordinates and size to `i32`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::*;
    /// # use nalgebra_glm::Vec2;
    /// # let mut core = RainCore::init("example app", 640, 360, true).unwrap();
    /// core.draw_rect_f(Vec2::new(100.0, 100.0), Vec2::new(50.0, 50.0), WHITE);
    /// ```
    pub fn draw_rect_f(&mut self, pos: Vec2, size: Vec2, color: Color) {
        self.draw_rect(vec2_to_ivec2(pos), vec2_to_ivec2(size), color);
    }

    /// Draws a rectangle at `pos` with `size`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::*;
    /// # use nalgebra_glm::IVec2;
    /// # let mut core = RainCore::init("example app", 640, 360, true).unwrap();
    /// core.draw_rect(IVec2::new(100, 100), IVec2::new(50, 50), WHITE);
    /// ```
    pub fn draw_rect(&mut self, pos: IVec2, size: IVec2, color: Color) {
        let p2 = IVec2::new(pos.x + size.x, pos.y);
        let p3 = IVec2::new(pos.x + size.x, pos.y + size.y);
        let p4 = IVec2::new(pos.x, pos.y + size.y);
        self.draw_line(pos, p2, color);
        self.draw_line(p2, p3, color);
        self.draw_line(p3, p4, color);
        self.draw_line(p4, pos, color);
    }

    /// Draws a filled in rectangle at `pos` with `size` after casting the coordinates and size to `i32`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::*;
    /// # use nalgebra_glm::Vec2;
    /// # let mut core = RainCore::init("example app", 640, 360, true).unwrap();
    /// core.fill_rect_f(Vec2::new(100.0, 100.0), Vec2::new(50.0, 50.0), WHITE);
    /// ```
    pub fn fill_rect_f(&mut self, pos: Vec2, size: Vec2, color: Color) {
        self.fill_rect(vec2_to_ivec2(pos), vec2_to_ivec2(size), color);
    }

    /// Draws a filled in rectangle at `pos` with `size`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::*;
    /// # use nalgebra_glm::IVec2;
    /// # let mut core = RainCore::init("example app", 640, 360, true).unwrap();
    /// core.fill_rect(IVec2::new(100, 100), IVec2::new(50, 50), WHITE);
    /// ```
    pub fn fill_rect(&mut self, pos: IVec2, size: IVec2, color: Color) {
        let p2 = pos + size;
        for i in pos.x..p2.x {
            for j in pos.y..p2.y {
                self.draw(IVec2::new(i, j), color);
            }
        }
    }

    /// Draws a triangle with vertices `p1`, `p2` and `p3` after casting the coordinates to `i32`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::*;
    /// # use nalgebra_glm::Vec2;
    /// # let mut core = RainCore::init("example app", 640, 360, true).unwrap();
    /// core.draw_triangle_f(Vec2::new(25.0, 100.0), Vec2::new(75.0, 100.0), Vec2::new(50.0, 0.0), WHITE);
    /// ```
    pub fn draw_triangle_f(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, color: Color) {
        self.draw_triangle(vec2_to_ivec2(p1), vec2_to_ivec2(p2), vec2_to_ivec2(p3), color);
    }

    /// Draws a triangle with vertices `p1`, `p2` and `p3`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::*;
    /// # use nalgebra_glm::IVec2;
    /// # let mut core = RainCore::init("example app", 640, 360, true).unwrap();
    /// core.draw_triangle(IVec2::new(25, 100), IVec2::new(75, 100), IVec2::new(50, 0), WHITE);
    /// ```
    pub fn draw_triangle(&mut self, p1: IVec2, p2: IVec2, p3: IVec2, color: Color) {
        self.draw_line(p1, p2, color);
        self.draw_line(p2, p3, color);
        self.draw_line(p3, p1, color);
    }

    /// Draws a filled in triangle with vertices `p1`, `p2` and `p3` after casting the coordinates to `i32`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::*;
    /// # use nalgebra_glm::Vec2;
    /// # let mut core = RainCore::init("example app", 640, 360, true).unwrap();
    /// core.fill_triangle_f(Vec2::new(25.0, 100.0), Vec2::new(75.0, 100.0), Vec2::new(50.0, 0.0), WHITE);
    /// ```
    pub fn fill_triangle_f(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, color: Color) {
        self.fill_triangle(vec2_to_ivec2(p1), vec2_to_ivec2(p2), vec2_to_ivec2(p3), color);
    }

    /// Draws a filled in triangle with vertices `p1`, `p2` and `p3`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::*;
    /// # use nalgebra_glm::IVec2;
    /// # let mut core = RainCore::init("example app", 640, 360, true).unwrap();
    /// core.fill_triangle(IVec2::new(25, 100), IVec2::new(75, 100), IVec2::new(50, 0), WHITE);
    /// ```
    // http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
    pub fn fill_triangle(&mut self, mut p1: IVec2, mut p2: IVec2, mut p3: IVec2, color: Color) {
        // sort vertices
        if p1.y > p2.y { swap(&mut p1, &mut p2); }
        if p1.y > p3.y { swap(&mut p1, &mut p3); }
        if p2.y > p3.y { swap(&mut p2, &mut p3); }

        // flat bottom triangle
        if p2.y == p3.y {
            self.fill_triangle_bottom(p1, p2, p3, color);
        }
        // flat top triangle
        else if p1.y == p2.y {
            self.fill_triangle_top(p1, p2, p3, color);
        }
        // split triangle and fill sides
        else {
            let p4 = IVec2::new(p1.x + f32::round(((p2.y - p1.y) as f32 / (p3.y - p1.y) as f32) * (p3.x - p1.x) as f32) as i32, p2.y);
            self.fill_triangle_bottom(p1, p2, p4, color);
            self.fill_triangle_top(p2, p4, p3, color);
        }
    }

    fn fill_triangle_bottom(&mut self, p1: IVec2, p2: IVec2, p3: IVec2, color: Color) {
        // calculate slope
        let s1 = (p2.x - p1.x) as f32 / (p2.y - p1.y) as f32;
        let s2 = (p3.x - p1.x) as f32 / (p3.y - p1.y) as f32;

        let mut x1 = p1.x as f32;
        let mut x2 = p1.x as f32;

        // draw scanlines, adjust ends of lines according to slopes
        for y in p1.y..=p2.y {
            self.draw_line(IVec2::new(f32::round(x1) as i32, y), IVec2::new(f32::round(x2) as i32, y), color);
            x1 += s1;
            x2 += s2;
        }
    }

    fn fill_triangle_top(&mut self, p1: IVec2, p2: IVec2, p3: IVec2, color: Color) {
        // calculate slopes
        let s1 = (p3.x - p1.x) as f32 / (p3.y - p1.y) as f32;
        let s2 = (p3.x - p2.x) as f32 / (p3.y - p2.y) as f32;

        let mut x1 = p3.x as f32;
        let mut x2 = p3.x as f32;

        // draw scanlines, adjust ends of lines according to slopes
        for y in (p1.y..=p3.y).rev() {
            self.draw_line(IVec2::new(f32::round(x1) as i32, y), IVec2::new(f32::round(x2) as i32, y), color);
            x1 -= s1;
            x2 -= s2;
        }
    }
}

#[inline]
fn vec2_to_ivec2(v: Vec2) -> IVec2 {
    IVec2::new(v.x as i32, v.y as i32)
}