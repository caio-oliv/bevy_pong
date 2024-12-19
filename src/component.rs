use bevy::{
    color::Color,
    ecs::component::Component,
    input::keyboard::KeyCode,
    math::{Vec2, Vec3},
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
};

use crate::settings;

#[derive(Component)]
#[require(Paddle, Sprite, Transform)]
pub struct Player;

impl Player {
    pub const Z_INDEX: f32 = 1.0;
}

#[derive(Component)]
pub struct Paddle {
    pub move_up: KeyCode,
    pub move_down: KeyCode,
}

impl Paddle {
    pub const SIZE: Vec2 = Vec2::new(10., 150.);
    pub const EDGE_MARGIN: f32 = 20.0;

    pub const fn new_main() -> Self {
        Self {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
        }
    }

    pub const fn new_secondary() -> Self {
        Self {
            move_up: KeyCode::ArrowUp,
            move_down: KeyCode::ArrowDown,
        }
    }

    pub const fn new_main_transform() -> Transform {
        Transform::from_translation(Vec3::new(
            -PlayArea::SIZE.x * 0.5 + Paddle::EDGE_MARGIN,
            0.,
            Player::Z_INDEX,
        ))
    }

    pub const fn new_secondary_transform() -> Transform {
        Transform::from_translation(Vec3::new(
            PlayArea::SIZE.x * 0.5 + -Paddle::EDGE_MARGIN,
            0.,
            Player::Z_INDEX,
        ))
    }
}

impl Default for Paddle {
    fn default() -> Self {
        Self::new_main()
    }
}

#[derive(Default, Component)]
#[require(Sprite, Transform)]
pub struct PlayArea;

impl PlayArea {
    pub const SIZE: Vec2 = Vec2::new(settings::WINDOW_WIDTH, settings::WINDOW_HEIGHT);

    pub fn new_sprite() -> Sprite {
        Sprite {
            color: Color::BLACK,
            custom_size: Some(Self::SIZE),
            ..default()
        }
    }
}

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Ball {
    pub size: Vec2,
    pub velocity: Vec2,
}

impl Ball {
    pub const Z_INDEX: f32 = 1.0;
    pub const DEFAULT_SIZE: Vec2 = Vec2::new(25.0, 25.0);

    pub fn new_sprite(&self) -> Sprite {
        Sprite {
            custom_size: Some(self.size),
            ..default()
        }
    }

    pub const fn new_transform() -> Transform {
        Transform::from_translation(Vec3::new(0.0, 0.0, Self::Z_INDEX))
    }
}
