use core::time::Duration;

use bevy::{
    asset::{Assets, Handle},
    ecs::{entity::Entity, system::Resource, world::FromWorld},
    math::{primitives::Rectangle, Vec2},
    render::mesh::Mesh,
    time::{Timer, TimerMode},
};

use crate::game::player::{Player, PlayerSide, PlayerType, SecondPlayerType};

#[derive(Clone, PartialEq, Eq, Default, Resource)]
pub struct UserGamepad {
    main: Option<Entity>,
    second: Option<Entity>,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ActiveGamepad {
    #[default]
    Main = 1,
    Second = 2,
}

impl UserGamepad {
    pub const fn add_gamepad(&mut self, gamepad: Entity) -> Option<ActiveGamepad> {
        if self.main.is_none() {
            self.main = Some(gamepad);

            Some(ActiveGamepad::Main)
        } else if self.second.is_none() {
            self.second = Some(gamepad);

            Some(ActiveGamepad::Second)
        } else {
            None
        }
    }

    pub fn remove_gamepad(&mut self, gamepad: Entity) -> Option<ActiveGamepad> {
        if self.main.is_some_and(|ent| ent == gamepad) {
            self.main = None;

            let removed = if self.try_move_second_to_main() {
                ActiveGamepad::Second
            } else {
                ActiveGamepad::Main
            };

            Some(removed)
        } else if self.second.is_some_and(|ent| ent == gamepad) {
            self.second = Some(gamepad);

            Some(ActiveGamepad::Second)
        } else {
            None
        }
    }

    const fn try_move_second_to_main(&mut self) -> bool {
        if self.second.is_some() {
            self.main = self.second;
            self.second = None;

            true
        } else {
            false
        }
    }

    pub const fn get_main(&self) -> Option<Entity> {
        self.main
    }

    pub const fn get_second(&self) -> Option<Entity> {
        self.second
    }

    pub const fn get_by_player(&self, player: &Player) -> Option<Entity> {
        match player.get_side() {
            PlayerSide::Main => self.get_main(),
            PlayerSide::Other => self.get_second(),
        }
    }
}

#[derive(Resource)]
pub struct StartMatchTimer(pub Timer);

impl StartMatchTimer {
    pub const SECONDS: u32 = 3;

    pub fn default_timer() -> Timer {
        Timer::new(Duration::from_secs(Self::SECONDS as u64), TimerMode::Once)
    }
}

impl Default for StartMatchTimer {
    fn default() -> Self {
        Self(Self::default_timer())
    }
}

#[derive(Clone, Default, Resource)]
pub struct GameActiveData {
    last_winner: Option<PlayerType>,
    score: GameScore,
}

impl GameActiveData {
    pub const fn score(&self) -> GameScore {
        self.score
    }

    pub const fn last_winner(&self) -> Option<PlayerType> {
        self.last_winner
    }

    pub const fn register_point(&mut self, player: PlayerType) {
        self.last_winner = Some(player);
        match player {
            PlayerType::Main => self.score.main += 1,
            PlayerType::Second => self.score.second += 1,
            PlayerType::AI => self.score.second += 1,
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct GameScore {
    main: u8,
    second: u8,
}

impl GameScore {
    pub const fn player1(&self) -> u8 {
        self.main
    }

    pub const fn player2(&self) -> u8 {
        self.second
    }

    #[expect(unused)]
    pub const fn winning_player(&self) -> Option<PlayerSide> {
        if self.main > self.second {
            Some(PlayerSide::Main)
        } else if self.main < self.second {
            Some(PlayerSide::Other)
        } else {
            None
        }
    }
}

#[derive(Default, Resource)]
pub struct SecondPlayer {
    pub opponent: SecondPlayerType,
}

#[derive(Resource)]
pub struct CommonMesh {
    quad: Handle<Mesh>,
}

impl CommonMesh {
    pub fn quad(&self) -> Handle<Mesh> {
        self.quad.clone()
    }
}

impl FromWorld for CommonMesh {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let quad = meshes.add(Rectangle::from_size(Vec2::ONE));
        Self { quad }
    }
}
