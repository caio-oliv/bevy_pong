use bevy::prelude::*;

use core::fmt;

use crate::{
    game::arena::{ArenaDirection, Paddle},
    settings::input::{
        GamepadInputSettings, KeyboardInputSettings, GAMEPAD_SETTINGS, MAIN_PLAYER_KEYBOARD,
        SECOND_PLAYER_KEYBOARD,
    },
};

/// All players of the game
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum PlayerType {
    #[default]
    Main = 1,
    Second = 2,
    AI = 3,
}

impl PlayerType {
    pub const fn arena_direction(&self) -> ArenaDirection {
        match *self {
            Self::Main => ArenaDirection::Left,
            Self::Second => ArenaDirection::Right,
            Self::AI => ArenaDirection::Right,
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match *self {
            Self::Main => "main",
            Self::Second => "second",
            Self::AI => "AI",
        }
    }
}

impl fmt::Display for PlayerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum PlayerSide {
    /// Main player.
    ///
    /// Is at the left side of the arena.
    /// Always controlled by the main player.
    #[default]
    Main = 1,
    /// Other player.
    ///
    /// Is at the right side of the arena.
    /// Can be controlled by the second player or the AI.
    Other = 2,
}

impl PlayerSide {
    #[expect(unused)]
    pub const fn arena_direction(&self) -> ArenaDirection {
        match *self {
            Self::Main => ArenaDirection::Left,
            Self::Other => ArenaDirection::Right,
        }
    }

    pub const fn to_player_type(self, opponent: SecondPlayerType) -> PlayerType {
        match (self, opponent) {
            (Self::Main, _) => PlayerType::Main,
            (Self::Other, SecondPlayerType::Player) => PlayerType::Second,
            (Self::Other, SecondPlayerType::AI) => PlayerType::AI,
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum SecondPlayerType {
    #[default]
    Player = 2,
    AI = 3,
}

impl SecondPlayerType {
    pub const fn change_opponent(self) -> Self {
        match self {
            Self::Player => Self::AI,
            Self::AI => Self::Player,
        }
    }

    #[expect(unused)]
    pub fn is_player(&self) -> bool {
        *self == Self::Player
    }

    #[expect(unused)]
    pub fn is_ai(&self) -> bool {
        *self == Self::AI
    }
}

#[derive(Clone, Copy, Default, Component)]
#[require(Paddle)]
pub struct Player {
    kind: PlayerSide,
}

impl Player {
    pub const fn new_main() -> Self {
        Self {
            kind: PlayerSide::Main,
        }
    }

    pub const fn new_second() -> Self {
        Self {
            kind: PlayerSide::Other,
        }
    }

    pub const fn keyboard_input(&self) -> KeyboardInputSettings {
        match self.kind {
            PlayerSide::Main => MAIN_PLAYER_KEYBOARD,
            PlayerSide::Other => SECOND_PLAYER_KEYBOARD,
        }
    }

    pub const fn gamepad_input(&self) -> GamepadInputSettings {
        GAMEPAD_SETTINGS
    }

    pub const fn get_side(&self) -> PlayerSide {
        self.kind
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Component)]
#[require(Player)]
pub struct PlayerAI;
