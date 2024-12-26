use bevy::prelude::*;

use super::player::PlayerSide;

#[derive(Clone, Copy, PartialEq, Eq, Event)]
pub struct PointMarked {
    pub winner: PlayerSide,
}

impl PointMarked {
    pub const fn new(winner: PlayerSide) -> Self {
        Self { winner }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Event)]
pub struct GameDataUpdated;
