#![warn(missing_docs)]
//! rain2d core functionality

use minifb::{Window, WindowOptions, KeyRepeat, MouseMode};
use bresenham::Bresenham;
use std::{
    time::Duration,
    time::Instant,
    mem::swap
};

pub use crate::core::color::*;

/// Reexported from minifb
///
pub use minifb::Key as Key;

/// Reexported from minifb
///
pub use minifb::MouseButton as MouseButton;

use crate::core::rendertarget::*;

mod color;
mod rendertarget;

#[allow(unused_variables)]
/// Trait used to call event functions from main loop
///
/// It's not required to implement any of these functions
/// although you probably want to or nothing will happen
///
/// ### Example
/// ```no_run
/// use rain2d::core::*;
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
///     true);
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
    window: Option<Window>,
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
    /// use rain2d::core::*;
    ///
    /// let mut core = RainCore::init("example app",
    ///         640,
    ///         360,
    ///         true);
    /// ```
    pub fn init(window_title: &str, width: usize, height: usize, exit_on_esc: bool) -> Self {
        RainCore {
            exit_on_esc,
            active: true,
            window_title: window_title.to_string(),
            window: None,
            render_target: RenderTarget::new(width, height),
            screen_width: width,
            screen_height: height,
            frame_timer: 1.0,
            frame_count: 0,
        }
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
    /// # use rain2d::core::*;
    /// #
    /// struct ExampleApp;
    ///
    /// impl RainApp for ExampleApp {}
    ///
    /// # let mut core = RainCore::init("example app", 640, 360, true);
    /// core.run(&mut ExampleApp {});
    /// ```
    pub fn run(&mut self, app: &mut dyn RainApp) {
        self.window = Some(Window::new(&self.window_title,
                                 self.screen_width,
                                 self.screen_height,
                                 WindowOptions::default()).unwrap());

        app.on_start();

        let mut last_time = Instant::now();
        while self.active {
            let current_time = Instant::now();
            let elapsed = current_time - last_time;
            last_time = current_time;

            // update state
            app.on_update(self, elapsed);

            // draw to screen
            if let Some(window) = &mut self.window {
                window.update_with_buffer(&self.render_target.data,
                                               self.render_target.width,
                                               self.render_target.height).unwrap();

            }

            // update frame count
            self.frame_timer += elapsed.as_secs_f32();
            self.frame_count += 1;
            if self.frame_timer >= 1.0 {
                self.frame_timer -= 1.0;

                if let Some(window) = &mut self.window {
                    let title = format!("{} - FPS: {}", self.window_title, self.frame_count);
                    window.set_title(&title);
                }

                self.frame_count = 0;
            }

            if let Some(window) = &self.window {
                // check window status
                if !window.is_open() {
                    self.active = false;
                }

                if self.exit_on_esc && window.is_key_down(Key::Escape) {
                    self.active = false;
                }
            }
        }

        app.on_exit();
    }

    /// Stops the main loop after the current frame has been drawn and calls [`on_exit`]
    ///
    /// ### Example
    ///```no_run
    ///# use rain2d::core::*;
    ///#
    ///# let mut core = RainCore::init("example app",
    ///#     640,
    ///#     360,
    ///#     true);
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

    /// Checks if the key is currently down
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::core::*;
    /// # let core = RainCore::init("example app", 640, 360, true);
    /// if core.key_down(Key::Space) {
    ///     println!("Spacebar down");
    /// }
    /// ```
    pub fn key_down(&self, key: Key) -> bool {
        if let Some(window) = &self.window {
            return window.is_key_down(key);
        }
        false
    }

    /// Checks if the key was pressed (not held) since the last update
    pub fn key_pressed(&self, key: Key) -> bool {
        if let Some(window) = &self.window {
            return window.is_key_pressed(key, KeyRepeat::No);
        }
        false
    }

    /// Checks if the key was released since the last update
    pub fn key_released(&self, key: Key) -> bool {
        if let Some(window) = &self.window {
            window.is_key_released(key);
        }
        false
    }

    /// Gets all keys that are currently down
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::core::*;
    /// # let mut core = RainCore::init("example app", 640, 360, true);
    /// core.get_keys().map(|keys| {
    ///     for key in keys {
    ///         match key {
    ///             Key::A => println!("A key down"),
    ///             Key::Escape => core.exit(),
    ///             _ => (),
    ///         }
    ///     }
    /// });
    /// ```
    pub fn get_keys(&self) -> Option<Vec<Key>> {
        if let Some(window) = &self.window {
            return window.get_keys();
        }
        None
    }

    /// Get mouse position relative to the window, (0, 0) in upper left corner
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::core::*;
    /// # let core = RainCore::init("example app", 640, 360, true);
    /// if let Some((x, y)) = core.get_mouse_pos() {
    ///     println!("x: {}, y: {}", x, y);
    /// }
    /// ```
    pub fn get_mouse_pos(&self) -> Option<(f32, f32)> {
        if let Some(window) = &self.window {
            return window.get_mouse_pos(MouseMode::Pass);
        }
        None
    }

    /// Checks if the button is currently down
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::core::*;
    /// # let core = RainCore::init("example app", 640, 360, true);
    /// if core.mouse_button_down(MouseButton::Left) {
    ///     println!("Left mouse button down");
    /// }
    /// ```
    pub fn mouse_button_down(&self, button: MouseButton) -> bool {
        if let Some(window) = &self.window {
            return window.get_mouse_down(button);
        }
        false
    }

    /// Get current scroll wheel movement
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::core::*;
    /// # let core = RainCore::init("example app", 640, 360, true);
    /// if let Some((x, y)) = core.get_scroll_wheel() {
    ///     println!("x: {}, y: {}", x, y);
    /// }
    /// ```
    pub fn get_scroll_wheel(&self) -> Option<(f32,f32)> {
        if let Some(window) = &self.window {
            return window.get_scroll_wheel();
        }
        None
    }

    /// Clears the screen with the provided color
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::core::*;
    /// # let mut core = RainCore::init("example app", 640, 360, true);
    /// core.clear(NONE);
    /// ```
    pub fn clear(&mut self, color: Color) {
        self.render_target.clear(color);
    }

    /// Draws a pixel if the location is in bounds
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::core::*;
    /// # let mut core = RainCore::init("example app", 640, 360, true);
    /// core.draw(10, 10, WHITE);
    /// ```
    pub fn draw(&mut self, x: i32, y: i32, color: Color) {
        self.render_target.set_pixel(x, y, color);
    }

    /// Draws a line from `(x1, y1)` to `(x2, y2)`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::core::*;
    /// # let mut core = RainCore::init("example app", 640, 360, true);
    /// core.draw_line(10, 10, 100, 50, WHITE);
    /// ```
    pub fn draw_line(&mut self, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, color: Color) {

        // vertical line
        if x2 - x1 == 0 {
            if y2 < y1 { swap(&mut y1, &mut y2); }
            for y in y1..y2 {
                self.draw(x1, y, color);
            }
            return;
        }

        // horizontal line
        if y2 - y1 == 0 {
            if x2 < x1 { swap(&mut x1, &mut x2); }
            for x in x1..x2 {
                self.draw(x, y1, color);
            }
            return;
        }

        for (x,y) in Bresenham::new((x1 as isize, y1 as isize), (x2 as isize, y2 as isize)) {
            self.draw(x as i32, y as i32, color);
        }
    }

    /// Draws a circle at `(x, y)` with radius `r`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::core::*;
    /// # let mut core = RainCore::init("example app", 640, 360, true);
    /// core.draw_circle(100, 100, 10, WHITE);
    /// ```
    pub fn draw_circle(&mut self, x: i32, y: i32, r: i32, color: Color) {
        let mut x0 = 0;
        let mut y0 = r;
        let mut d = 3 - 2 * r;
        if r <= 0 { return; }

        while y0 >= x0 {
            self.draw(x + x0, y - y0, color);
            self.draw(x + y0, y - x0, color);
            self.draw(x + y0, y + x0, color);
            self.draw(x + x0, y + y0, color);
            self.draw(x - x0, y - y0, color);
            self.draw(x - y0, y - x0, color);
            self.draw(x - y0, y + x0, color);
            self.draw(x - x0, y + y0, color);
            if d < 0 { d += 4 * x0 + 6; x0 += 1; }
            else { x0 += 1; y0 -= 1; d += 4 * (x0 - y0) + 10; }
        }
    }

    /// Draws a filled in circle at `(x, y)` with radius `r`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::core::*;
    /// # let mut core = RainCore::init("example app", 640, 360, true);
    /// core.fill_circle(100, 100, 10, WHITE);
    /// ```
    pub fn fill_circle(&mut self, x: i32, y: i32, r: i32, color: Color) {
        let mut x0 = 0;
        let mut y0 = r;
        let mut d = 3 - 2 * r;
        if r <= 0 { return; }

        while y0 >= x0 {
            self.draw_line(x - x0, y - y0, x + x0, y - y0, color);
            self.draw_line(x - y0, y - x0, x + y0, y - x0, color);
            self.draw_line(x - x0, y + y0, x + x0, y + y0, color);
            self.draw_line(x - y0, y + x0, x + y0, y + x0, color);
            if d < 0 { d += 4 * x0 + 6; x0 += 1; }
            else { x0 += 1; y0 -= 1; d += 4 * (x0 - y0) + 10; }
        }
    }

    /// Draws a rectangle at `(x, y)` with specified dimensions
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::core::*;
    /// # let mut core = RainCore::init("example app", 640, 360, true);
    /// core.draw_rect(100, 100, 50, 50, WHITE);
    /// ```
    pub fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        self.draw_line(x, y, x + width, y, color);
        self.draw_line(x + width, y, x + width, y + height, color);
        self.draw_line(x + width, y + height, x, y + height, color);
        self.draw_line(x, y + height, x, y, color);
    }

    /// Draws a filled in rectangle at `(x, y)` with specified dimensions
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::core::*;
    /// # let mut core = RainCore::init("example app", 640, 360, true);
    /// core.fill_rect(100, 100, 50, 50, WHITE);
    /// ```
    pub fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        let x_max = x + width;
        let y_max = y + height;
        for i in x..x_max {
            for j in y..y_max {
                self.draw(i, j, color);
            }
        }
    }

    /// Draws a triangle with vertices `(x1, y1)`, `(x2, y2)` and `(x3, y3)`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::core::*;
    /// # let mut core = RainCore::init("example app", 640, 360, true);
    /// core.draw_triangle(25, 100, 75, 100, 50, 0, WHITE);
    /// ```
    pub fn draw_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: Color) {
        self.draw_line(x1, y1, x2, y2, color);
        self.draw_line(x2, y2, x3, y3, color);
        self.draw_line(x3, y3, x1, y1, color);
    }

    /// Draws a filled in triangle with vertices `(x1, y1)`, `(x2, y2)` and `(x3, y3)`
    ///
    /// ### Example
    /// ```no_run
    /// # use rain2d::core::*;
    /// # let mut core = RainCore::init("example app", 640, 360, true);
    /// core.fill_triangle(25, 100, 75, 100, 50, 0, WHITE);
    /// ```
    // http://www.sunshine2k.de/coding/java/TriangleRasterization/TriangleRasterization.html
    pub fn fill_triangle(&mut self, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, mut x3: i32, mut y3: i32, color: Color) {
        // sort vertices
        if y1 > y2 { swap(&mut x1, &mut x2); swap(&mut y1, &mut y2); }
        if y1 > y3 { swap(&mut x1, &mut x3); swap(&mut y1, &mut y3); }
        if y2 > y3 { swap(&mut x2, &mut x3); swap(&mut y2, &mut y3); }

        // flat bottom triangle
        if y2 == y3 {
            self.fill_triangle_bottom(x1, y1, x2, y2, x3, y3, color);
        }
        // flat top triangle
        else if y1 == y2 {
            self.fill_triangle_top(x1, y1, x2, y2, x3, y3, color);
        }
        // split triangle and fill sides
        else {
            let x4 = x1 + f32::round(((y2 - y1) as f32 / (y3 - y1) as f32) * (x3 - x1) as f32) as i32;
            self.fill_triangle_bottom(x1, y1, x2, y2, x4, y2, color);
            self.fill_triangle_top(x2, y2, x4, y2, x3, y3, color);
        }
    }

    fn fill_triangle_bottom(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: Color) {
        // calculate slope
        let s1 = (x2 - x1) as f32 / (y2 - y1) as f32;
        let s2 = (x3 - x1) as f32 / (y3 - y1) as f32;

        let mut x1 = x1 as f32;
        let mut x2 = x1 as f32;

        // draw scanlines, adjust ends of lines according to slopes
        for y in y1..=y2 {
            self.draw_line(f32::round(x1) as i32, y, f32::round(x2) as i32, y, color);
            x1 += s1;
            x2 += s2;
        }
    }

    fn fill_triangle_top(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: Color) {
        // calculate slopes
        let s1 = (x3 - x1) as f32 / (y3 - y1) as f32;
        let s2 = (x3 - x2) as f32 / (y3 - y2) as f32;

        let mut x1 = x3 as f32;
        let mut x2 = x3 as f32;

        // draw scanlines, adjust ends of lines according to slopes
        for y in (y1..=y3).rev() {
            self.draw_line(f32::round(x1) as i32, y, f32::round(x2) as i32, y, color);
            x1 -= s1;
            x2 -= s2;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_core(w: usize, h: usize) -> RainCore {
        RainCore::init("", w, h, false)
    }

    #[test]
    fn test_clear() {
        let mut core = create_core(10, 10);

        core.clear(WHITE);
        for p in core.render_target.data {
            assert_eq!(p, WHITE.into());
        }
    }

    #[test]
    fn test_draw() {
        let mut core = create_core(10, 10);

        core.draw(5, 3, WHITE);
        assert_eq!(core.render_target.get_pixel(5, 3), Some(WHITE));
    }
}
