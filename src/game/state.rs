use bevy::prelude::*;

use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState {
    #[default]
    MainMenu,
    GameActive {
        playing: bool,
    },
}

impl GameState {
    pub const fn playing() -> Self {
        Self::GameActive { playing: true }
    }

    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::MainMenu => "main_menu",
            Self::GameActive { playing: false } => "game_active",
            Self::GameActive { playing: true } => "game_active.playing",
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GameActiveState {
    Playing = 1,
    Pause = 2,
}

impl GameActiveState {
    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::Playing => "playing",
            Self::Pause => "pause",
        }
    }
}

impl ComputedStates for GameActiveState {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            GameState::MainMenu => None,
            GameState::GameActive { playing: true } => Some(Self::Playing),
            GameState::GameActive { playing: false } => Some(Self::Pause),
        }
    }
}

impl fmt::Display for GameActiveState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct InGame;

impl ComputedStates for InGame {
    type SourceStates = GameState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            GameState::MainMenu => None,
            GameState::GameActive { playing: true } => Some(Self),
            GameState::GameActive { playing: false } => Some(Self),
        }
    }
}
