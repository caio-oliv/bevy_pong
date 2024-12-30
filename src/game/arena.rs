use bevy::{math::bounding::BoundingCircle, prelude::*};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use super::physics::LinearVelocity;

#[derive(Default, Component)]
#[require(Transform, Visibility)]
pub struct Arena;

impl Arena {
    pub const SIZE: Vec2 = Vec2::new(100.0, 50.0);

    pub const COLOR: Color = Color::WHITE;

    pub fn bounds() -> Vec2 {
        Self::SIZE - Wall::THICKNESS
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ArenaDirection {
    Left,
    Right,
}

impl ArenaDirection {
    pub const fn inverted(&self) -> Self {
        match *self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl Distribution<ArenaDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ArenaDirection {
        if rng.gen::<bool>() {
            ArenaDirection::Left
        } else {
            ArenaDirection::Right
        }
    }
}

#[derive(PartialEq, Eq, Component)]
#[require(Transform)]
pub enum Wall {
    Top = 1,
    Bottom = 2,
}

impl Wall {
    pub const THICKNESS: f32 = 2.0;

    pub const fn top_transform() -> Transform {
        Transform::from_xyz(0.0, Arena::SIZE.y / 2.0, 0.0).with_scale(Vec3::new(
            Arena::SIZE.x + Self::THICKNESS,
            Self::THICKNESS,
            1.0,
        ))
    }

    pub const fn bottom_transform() -> Transform {
        Transform::from_xyz(0.0, -Arena::SIZE.y / 2.0, 0.0).with_scale(Vec3::new(
            Arena::SIZE.x + Self::THICKNESS,
            Self::THICKNESS,
            1.0,
        ))
    }

    #[expect(unused)]
    pub const fn transform(&self) -> Transform {
        match *self {
            Wall::Top => Self::top_transform(),
            Wall::Bottom => Self::bottom_transform(),
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Component)]
#[require(Transform, Visibility)]
pub struct Paddle;

impl Paddle {
    pub const Z_INDEX: f32 = 1.0;

    pub const LENGTH: f32 = 10.0;
    pub const EDGE_MARGIN: f32 = 1.5;
    pub const VELOCITY: f32 = 50.0;

    pub const COLOR: Color = Color::WHITE;

    pub fn new_main_transform() -> Transform {
        Transform::from_xyz(
            -Arena::bounds().x / 2.0 + Paddle::EDGE_MARGIN,
            0.0,
            Self::Z_INDEX,
        )
        .with_scale(Vec3::new(1.0, Self::LENGTH, 1.0))
    }

    pub fn new_second_transform() -> Transform {
        Transform::from_xyz(
            Arena::bounds().x / 2.0 + -Paddle::EDGE_MARGIN,
            0.0,
            Self::Z_INDEX,
        )
        .with_scale(Vec3::new(1.0, Self::LENGTH, 1.0))
    }

    pub fn min_y_position() -> f32 {
        -Arena::bounds().y / 2.0 + Self::LENGTH / 2.0
    }

    pub fn max_y_position() -> f32 {
        Arena::bounds().y / 2.0 - Self::LENGTH / 2.0
    }
}

#[derive(Default, Component)]
#[require(Transform, Visibility)]
pub struct Ball;

impl Ball {
    pub const Z_INDEX: f32 = 1.0;
    pub const RADIUS: f32 = 0.5;
    pub const START_VELOCITY: f32 = 18.0;
    pub const MAX_SPEED: f32 = 120.0;
    pub const ACCELERATION_PERCENT: f32 = 0.05;
    pub const COLOR: Color = Color::WHITE;

    pub const fn primitive() -> Circle {
        Circle::new(Self::RADIUS)
    }

    pub fn bounding_circle(transform: &Transform) -> BoundingCircle {
        let scale = (transform.scale.x + transform.scale.y) * 0.5;
        BoundingCircle::new(transform.translation.truncate(), Ball::RADIUS * scale)
    }

    pub const fn start_velocity_x(direction: ArenaDirection) -> f32 {
        match direction {
            ArenaDirection::Left => -Ball::START_VELOCITY,
            ArenaDirection::Right => Ball::START_VELOCITY,
        }
    }

    pub fn random_linear_velocity(direction: ArenaDirection) -> Vec2 {
        Vec2::new(
            Self::start_velocity_x(direction),
            rand::thread_rng().gen_range(-1.0..1.0) * Ball::START_VELOCITY * 0.5,
        )
    }

    pub fn limit_velocity(velocity: &mut LinearVelocity) {
        velocity.0.x = velocity.0.x.clamp(-Ball::MAX_SPEED, Ball::MAX_SPEED);
        velocity.0.y = velocity.0.y.clamp(-Ball::MAX_SPEED, Ball::MAX_SPEED);
    }

    pub const fn initial_transform() -> Transform {
        Transform::from_xyz(0.0, 0.0, Self::Z_INDEX).with_scale(Vec3::new(2.0, 2.0, 1.0))
    }

    pub const fn reset_initial_stationary_position(
        transform: &mut Transform,
        velocity: &mut LinearVelocity,
    ) {
        transform.translation = Vec3::ZERO;
        velocity.0 = Vec2::ZERO;
    }
}
