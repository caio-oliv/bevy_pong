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

#[derive(Clone, Copy, PartialEq, Component)]
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
    Left,
    Right,
    Top,
    Bottom,
}

pub fn resolve_ball_collision(collision: Collision, velocity: &mut LinearVelocity) {
    let mut reflect_x = false;
    let mut reflect_y = false;

    match collision {
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
pub fn ball_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&bounding_box) {
        return None;
    }

    let closest = bounding_box.closest_point(ball.center());
    let offset = ball.center() - closest;

    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
