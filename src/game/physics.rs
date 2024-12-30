use core::fmt;
use core::ops::{Deref, DerefMut};

use bevy::{
    ecs::component::Component,
    math::{
        bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
        Vec2,
    },
    transform::components::Transform,
};

#[derive(Clone, Copy, PartialEq, Eq, Component)]
#[require(Transform)]
pub struct Collider;

#[derive(Clone, Copy, Component)]
#[require(Transform)]
pub struct LinearVelocity(pub Vec2);

impl Deref for LinearVelocity {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LinearVelocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Collision {
    Left = 1,
    Right = 2,
    Top = 3,
    Bottom = 4,
}

impl Collision {
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Top => "top",
            Self::Bottom => "bottom",
        }
    }

    pub fn from_penetration(offset: Vec2) -> Self {
        match offset.y.abs() > offset.x.abs() {
            true => {
                if offset.y > 0.0 {
                    Self::Top
                } else {
                    Self::Bottom
                }
            }
            false => {
                if offset.x > 0.0 {
                    Self::Right
                } else {
                    Self::Left
                }
            }
        }
    }
}

impl fmt::Display for Collision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

pub fn resolve_ball_collision(
    offset: Vec2,
    transform: &mut Transform,
    velocity: &mut LinearVelocity,
) {
    transform.translation += offset.extend(0.0);

    let mut reflect_x = false;
    let mut reflect_y = false;

    match Collision::from_penetration(offset) {
        Collision::Left => reflect_x = velocity.x > 0.0,
        Collision::Right => reflect_x = velocity.x < 0.0,
        Collision::Top => reflect_y = velocity.y < 0.0,
        Collision::Bottom => reflect_y = velocity.y > 0.0,
    }

    // Reflect velocity on the x-axis if we hit something on the x-axis
    if reflect_x {
        velocity.x = -velocity.x;
    }

    // Reflect velocity on the y-axis if we hit something on the y-axis
    if reflect_y {
        velocity.y = -velocity.y;
    }
}

// Returns `Some` if `ball` collides with `bounding_box`.
// The returned `Collision` is the side of `bounding_box` that `ball` hit.
pub fn ball_collision(ball: &BoundingCircle, bounding_box: &Aabb2d) -> Option<Vec2> {
    if !ball.intersects(bounding_box) {
        return None;
    }

    let closest = bounding_box.closest_point(ball.center());
    let offset = ball.center() - closest;

    Some(offset)
}
