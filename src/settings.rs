use bevy::window::{WindowResizeConstraints, WindowResolution};

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

pub const WINDOW_RESIZE_CONSTRAINTS: WindowResizeConstraints = WindowResizeConstraints {
    min_width: WINDOW_WIDTH,
    min_height: WINDOW_HEIGHT,
    max_width: f32::INFINITY,
    max_height: f32::INFINITY,
};

pub fn default_window_resolution() -> WindowResolution {
    WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT)
}
