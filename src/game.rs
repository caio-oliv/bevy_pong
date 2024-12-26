use bevy::prelude::*;

use event::{GameDataUpdated, PointMarked};
use resource::{CommonMesh, GameActiveData};
use state::{GameActiveState, GameState};

pub mod arena;
pub mod event;
pub mod player;
pub mod resource;
pub mod state;
pub mod system;

pub fn plugin(app: &mut App) {
    use system::*;

    app.init_resource::<GameActiveData>();
    app.init_resource::<CommonMesh>();

    app.add_event::<PointMarked>();
    app.add_event::<GameDataUpdated>();

    app.add_systems(
        OnEnter(GameState::MainMenu),
        (spawn_arena, spawn_players, spawn_ball),
    );

    app.add_systems(OnEnter(GameActiveState::Playing), reset_ball);

    app.add_systems(
        FixedUpdate,
        (check_ball_leaved_arena, move_paddle).run_if(in_state(GameActiveState::Playing)),
    );
    app.add_systems(
        FixedUpdate,
        (reset_ball_after_point, register_score_point)
            .run_if(in_state(GameActiveState::Playing).and(on_event::<PointMarked>)),
    );
    app.add_systems(
        FixedLast,
        fix_ball_up_and_down_movement.run_if(in_state(GameActiveState::Playing)),
    );
}
