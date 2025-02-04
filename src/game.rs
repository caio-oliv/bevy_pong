use bevy::prelude::*;

use event::{GameDataUpdated, PointMarked};
use resource::{CommonMesh, GameActiveData, StartMatchTimer};
use state::{GameActiveState, InGame};

pub mod arena;
pub mod event;
pub mod physics;
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
        OnEnter(InGame),
        (
            reset_game_data,
            spawn_arena,
            spawn_players,
            spawn_ball,
            init_match,
        )
            .chain(),
    );
    app.add_systems(
        OnExit(InGame),
        (despawn_arena, despawn_players, despawn_ball),
    );

    app.add_systems(
        FixedUpdate,
        start_match
            .run_if(in_state(GameActiveState::Playing).and(resource_exists::<StartMatchTimer>)),
    );

    app.add_systems(
        FixedUpdate,
        (
            move_paddle_by_player,
            move_paddle_by_ai,
            move_ball,
            check_ball_leaved_arena,
        )
            .chain()
            .run_if(in_state(GameActiveState::Playing)),
    );
    app.add_systems(
        FixedUpdate,
        (register_score_point, init_match)
            .chain()
            .run_if(in_state(GameActiveState::Playing).and(on_event::<PointMarked>)),
    );
}
