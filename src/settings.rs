pub mod scale {
    pub const PHYSICS_LENGTH_UNIT: f32 = 1.0;
    pub const PIXEL_PER_METER: f32 = 100.0;
    #[expect(unused)]
    pub const PIXEL_PER_CENTIMETER: f32 = PIXEL_PER_METER / 100.0;
}

pub mod camera {
    use bevy::render::camera::OrthographicProjection;

    use super::scale::{PHYSICS_LENGTH_UNIT, PIXEL_PER_METER};

    pub const ORTHOGRAPHIC_PROJECTION_SCALE: f32 = PHYSICS_LENGTH_UNIT / PIXEL_PER_METER;
    pub const DEFAULT_CAMERA_2D_SCALE: f32 = 10.0;

    pub fn orthographic_projection() -> OrthographicProjection {
        OrthographicProjection {
            scale: ORTHOGRAPHIC_PROJECTION_SCALE * DEFAULT_CAMERA_2D_SCALE,
            ..OrthographicProjection::default_2d()
        }
    }
}

pub mod window {
    use bevy::window::{Window, WindowResizeConstraints, WindowResolution};

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

    pub fn primary_window() -> Window {
        Window {
            resolution: default_window_resolution(),
            resize_constraints: WINDOW_RESIZE_CONSTRAINTS,
            resizable: false,
            ..Default::default()
        }
    }
}

pub mod input {
    use bevy::input::keyboard::KeyCode;

    #[derive(PartialEq, Eq)]
    pub struct PlayerInputSettings {
        pub move_paddle_up: KeyCode,
        pub move_paddle_down: KeyCode,
        pub push_paddle_front: KeyCode,
    }

    impl PlayerInputSettings {
        pub const fn new_main_settings() -> Self {
            Self {
                move_paddle_up: KeyCode::KeyW,
                move_paddle_down: KeyCode::KeyS,
                push_paddle_front: KeyCode::Space,
            }
        }

        pub const fn new_second_settings() -> Self {
            Self {
                move_paddle_up: KeyCode::ArrowUp,
                move_paddle_down: KeyCode::ArrowDown,
                push_paddle_front: KeyCode::ArrowLeft,
            }
        }
    }

    pub const MAIN_PLAYER: PlayerInputSettings = PlayerInputSettings::new_main_settings();
    pub const SECOND_PLAYER: PlayerInputSettings = PlayerInputSettings::new_second_settings();
}
