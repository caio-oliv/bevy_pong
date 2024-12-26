use bevy::{
    asset::{Assets, Handle},
    ecs::{system::Resource, world::FromWorld},
    math::primitives::Rectangle,
    math::Vec2,
    render::mesh::Mesh,
};

use crate::game::player::{PlayerSide, PlayerType, SecondPlayerType};

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
