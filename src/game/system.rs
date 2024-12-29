use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;
use rand::Rng;

use super::{
    arena::{Arena, ArenaDirection, Ball, Paddle, Wall},
    event::{GameDataUpdated, PointMarked},
    physics::{ball_collision, resolve_ball_collision, Collider, LinearVelocity},
    player::{Player, PlayerSide, PlayerType},
    resource::{CommonMesh, GameActiveData, SecondPlayer, StartMatchTimer},
};

pub fn reset_game_data(mut game_data: ResMut<GameActiveData>) {
    *game_data = GameActiveData::default();
}

pub fn spawn_arena(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    app_meshs: Res<CommonMesh>,
) {
    let material = materials.add(Arena::COLOR);

    commands.spawn(Arena).with_children(|children| {
        children.spawn((
            Wall::Top,
            Mesh2d(app_meshs.quad()),
            MeshMaterial2d(material.clone()),
            Collider,
            Wall::top_transform(),
        ));
        children.spawn((
            Wall::Bottom,
            Mesh2d(app_meshs.quad()),
            MeshMaterial2d(material),
            Collider,
            Wall::bottom_transform(),
        ));
    });
}

pub fn despawn_arena(arena: Single<Entity, With<Arena>>, mut commands: Commands) {
    let entity = arena.into_inner();
    commands.entity(entity).despawn_recursive();
}

pub fn spawn_players(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    app_meshs: Res<CommonMesh>,
) {
    let material = materials.add(Paddle::COLOR);

    commands.spawn((
        Player::new_main(),
        Paddle::default(),
        Mesh2d(app_meshs.quad()),
        MeshMaterial2d(material.clone()),
        Collider,
        Paddle::new_main_transform(),
    ));
    commands.spawn((
        Player::new_second(),
        Paddle::default(),
        Mesh2d(app_meshs.quad()),
        MeshMaterial2d(material),
        Collider,
        Paddle::new_second_transform(),
    ));
}

pub fn despawn_players(players: Query<Entity, With<Player>>, mut commands: Commands) {
    for entity in &players {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_ball(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // TODO: use a sprite to draw the ball
    let mesh = meshes.add(Ball::primitive());
    let material = materials.add(Ball::DEFAULT_COLOR);

    commands.spawn((
        Ball,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        LinearVelocity(Vec2::ZERO),
        Ball::initial_transform(),
    ));
}

pub fn despawn_ball(ball: Single<Entity, With<Ball>>, mut commands: Commands) {
    let entity = ball.into_inner();
    commands.entity(entity).despawn();
}

pub fn init_match(mut commands: Commands) {
    commands.insert_resource(StartMatchTimer::default());
}

fn next_ball_direction(last_winner: Option<PlayerType>) -> ArenaDirection {
    last_winner
        .map(|player| player.arena_direction())
        .unwrap_or_else(|| rand::thread_rng().gen::<ArenaDirection>())
}

pub fn start_match(
    ball: Single<&mut LinearVelocity, With<Ball>>,
    game_data: Res<GameActiveData>,
    time: Res<Time>,
    mut match_timer: ResMut<StartMatchTimer>,
    mut commands: Commands,
) {
    if match_timer.0.tick(time.delta()).just_finished() {
        let mut velocity = ball.into_inner();

        let direction = next_ball_direction(game_data.last_winner());
        velocity.0 = Ball::random_linear_velocity(direction);

        commands.remove_resource::<StartMatchTimer>();
    }
}

pub fn move_paddle(
    mut paddles: Query<(&mut Transform, &Player, &Paddle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
) {
    for (mut transform, player, paddle) in &mut paddles {
        if input.pressed(player.input_settings().move_paddle_up) {
            transform.translation.y += paddle.velocity * time.delta_secs();
        }
        if input.pressed(player.input_settings().move_paddle_down) {
            transform.translation.y -= paddle.velocity * time.delta_secs();
        }

        transform.translation.y = transform
            .translation
            .y
            .clamp(Paddle::min_y_position(), Paddle::max_y_position());
    }
}

pub fn move_ball(
    ball: Single<(&mut Transform, &mut LinearVelocity), With<Ball>>,
    colliders: Query<&Transform, (With<Collider>, Without<Ball>)>,
    time: Res<Time<Fixed>>,
) {
    let (mut transform, mut velocity) = ball.into_inner();

    // TODO: increase ball velocity over time
    transform.translation += velocity.0.extend(0.0) * time.delta_secs();

    let bounding_ball = Ball::bounding_circle(&transform);

    for collider in &colliders {
        // TODO: apply a random Y direction when the ball bounces in the paddle.

        let bounding_box = Aabb2d::new(
            collider.translation.truncate(),
            collider.scale.truncate() * 0.5,
        );

        let collision = match ball_collision(bounding_ball, bounding_box) {
            None => continue,
            Some(collision) => collision,
        };

        resolve_ball_collision(collision, &mut velocity);
    }

    Ball::correct_trajectory(&mut velocity);
    Ball::limit_velocity(&mut velocity);
}

pub fn check_ball_leaved_arena(
    ball: Single<(&mut Transform, &mut LinearVelocity), With<Ball>>,
    mut point_event: EventWriter<PointMarked>,
) {
    let (mut transform, mut velocity) = ball.into_inner();

    if Arena::SIZE.x / 2.0 - Wall::THICKNESS <= transform.translation.x.abs() {
        let winner_side = if transform.translation.x < 0.0 {
            // Ball left, point goes to player other
            PlayerSide::Other
        } else {
            // Ball right, point goes to player main
            PlayerSide::Main
        };

        point_event.send(PointMarked::new(winner_side));

        Ball::reset_initial_stationary_position(&mut transform, &mut velocity);
    }
}

pub fn reset_ball_after_point(
    ball: Single<(&mut Transform, &mut LinearVelocity), With<Ball>>,
    mut point_event: EventReader<PointMarked>,
) {
    let (mut transform, mut velocity) = ball.into_inner();

    for _ in point_event.read() {
        Ball::reset_initial_stationary_position(&mut transform, &mut velocity);
    }
}

pub fn register_score_point(
    second_player: Res<SecondPlayer>,
    mut game_data: ResMut<GameActiveData>,
    mut point_event: EventReader<PointMarked>,
    mut game_data_update: EventWriter<GameDataUpdated>,
) {
    for event in point_event.read() {
        let player = event.winner.to_player_type(second_player.opponent);
        game_data.register_point(player);

        game_data_update.send(GameDataUpdated);
    }
}
