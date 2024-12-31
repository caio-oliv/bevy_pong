use bevy::math::bounding::{Aabb2d, RayCast2d};
use bevy::prelude::*;
use rand::Rng;

use super::{
    arena::{Arena, ArenaDirection, Ball, Paddle, PaddleDirection, Wall},
    event::{GameDataUpdated, PointMarked},
    physics::{ball_collision, resolve_ball_collision, Collider, LinearVelocity},
    player::{Player, PlayerAI, PlayerSide, PlayerType, SecondPlayerType},
    resource::{CommonMesh, GameActiveData, SecondPlayer, StartMatchTimer, UserGamepad},
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
    app_meshs: Res<CommonMesh>,
    second_player: Res<SecondPlayer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let material = materials.add(Paddle::COLOR);

    commands.spawn((
        Player::new_main(),
        Paddle,
        Mesh2d(app_meshs.quad()),
        MeshMaterial2d(material.clone()),
        Collider,
        Paddle::new_main_transform(),
    ));

    match second_player.opponent {
        SecondPlayerType::Player => commands.spawn((
            Player::new_second(),
            Paddle,
            Mesh2d(app_meshs.quad()),
            MeshMaterial2d(material),
            Collider,
            Paddle::new_second_transform(),
        )),
        SecondPlayerType::AI => commands.spawn((
            Player::new_second(),
            PlayerAI,
            Paddle,
            Mesh2d(app_meshs.quad()),
            MeshMaterial2d(material),
            Collider,
            Paddle::new_second_transform(),
        )),
    };
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
    let mesh = meshes.add(Ball::primitive());
    let material = materials.add(Ball::COLOR);

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

#[expect(clippy::type_complexity)]
pub fn move_paddle_by_player(
    mut paddles: Query<(&mut Transform, &Player), (With<Paddle>, Without<PlayerAI>)>,
    gamepads: Query<&Gamepad>,
    user_gamepad: Res<UserGamepad>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
) {
    for (mut transform, player) in &mut paddles {
        let gamepad = user_gamepad
            .get_by_player(player)
            .and_then(|entity| gamepads.get(entity).ok());

        if keyboard.pressed(player.keyboard_input().paddle_up)
            || gamepad.is_some_and(|gpad| gpad.pressed(player.gamepad_input().paddle_up))
        {
            Paddle::move_vertically(PaddleDirection::Up, &mut transform, time.delta_secs());
        }
        if keyboard.pressed(player.keyboard_input().paddle_down)
            || gamepad.is_some_and(|gpad| gpad.pressed(player.gamepad_input().paddle_down))
        {
            Paddle::move_vertically(PaddleDirection::Down, &mut transform, time.delta_secs());
        }

        Paddle::clamp_position(&mut transform);
    }
}

fn intelligent_paddle_ai_movement(
    paddle: &mut Transform,
    ball: &Transform,
    ball_velocity: &LinearVelocity,
    delta_time: f32,
) {
    // when the ball is on the side or going to the main player:
    if Ball::current_arena_direction(ball) == ArenaDirection::Left
        || Ball::moving_to_arena_direction(ball_velocity) == ArenaDirection::Left
    {
        // stay where we are
        return;
    }

    // when the ball is on the side of the AI player:
    // move paddle to the predicted position of the ball.

    let arena_right_collider = Arena::right_collider();
    let ball_direction = match Dir2::new(ball_velocity.0) {
        Ok(dir) => dir,
        _ => return,
    };
    let ray_cast = RayCast2d::new(ball.translation.truncate(), ball_direction, 100.0);
    let x_distance = Arena::SIZE.x * 0.5 - ball.translation.x;

    if let Some(move_distance) = ray_cast.aabb_intersection_at(&arena_right_collider) {
        let delta_y = (move_distance * move_distance - x_distance * x_distance).sqrt();

        let final_ball_position = if ball_direction.y > 0.0 {
            ball.translation.y + delta_y
        } else {
            ball.translation.y - delta_y
        };

        if (final_ball_position - paddle.translation.y).abs() < Paddle::AI_DEADZONE {
            // the paddle is already aligned with the final ball position.
            return;
        }

        // move paddle to the ball position
        if final_ball_position > paddle.translation.y {
            Paddle::move_vertically(PaddleDirection::Up, paddle, delta_time);
        } else if final_ball_position < paddle.translation.y {
            Paddle::move_vertically(PaddleDirection::Down, paddle, delta_time);
        }
    }

    Paddle::clamp_position(paddle);
}

#[expect(unused)]
fn simple_paddle_ai_movement(
    paddle: &mut Transform,
    ball: &Transform,
    ball_velocity: &LinearVelocity,
    delta_time: f32,
) {
    // when the ball is on the side or going to the main player:
    if Ball::current_arena_direction(ball) == ArenaDirection::Left
        || Ball::moving_to_arena_direction(ball_velocity) == ArenaDirection::Left
    {
        // stay where we are
        return;
    }

    // when the ball is on the side of the AI player:

    let diff = (ball.translation.y - paddle.translation.y).abs();
    if diff < Paddle::AI_DEADZONE {
        // the paddle is already aligned with the ball.
        return;
    }

    let smooth = (ball_velocity.y.abs() / diff).clamp(0.7, 1.0);

    // move paddle to the ball position
    if ball.translation.y > paddle.translation.y {
        Paddle::move_vertically(PaddleDirection::Up, paddle, delta_time * smooth);
    } else if ball.translation.y < paddle.translation.y {
        Paddle::move_vertically(PaddleDirection::Down, paddle, delta_time * smooth);
    }

    Paddle::clamp_position(paddle);
}

#[expect(clippy::type_complexity)]
pub fn move_paddle_by_ai(
    ball: Single<(&Transform, &LinearVelocity), With<Ball>>,
    paddles: Single<&mut Transform, (With<Paddle>, With<PlayerAI>, Without<Ball>)>,
    time: Res<Time<Fixed>>,
) {
    let (ball_transform, ball_velocity) = ball.into_inner();
    let mut paddle_transform = paddles.into_inner();

    intelligent_paddle_ai_movement(
        &mut paddle_transform,
        ball_transform,
        ball_velocity,
        time.delta_secs(),
    );
}

#[expect(clippy::type_complexity)]
pub fn move_ball(
    ball: Single<(&mut Transform, &mut LinearVelocity), With<Ball>>,
    colliders: Query<(&Transform, Option<&Paddle>), (With<Collider>, Without<Ball>)>,
    time: Res<Time<Fixed>>,
) {
    let (mut transform, mut velocity) = ball.into_inner();

    transform.translation += velocity.0.extend(0.0) * time.delta_secs();
    velocity.0 *= Ball::ACCELERATION_PERCENT * time.delta_secs() + 1.0;

    let bounding_ball = Ball::bounding_circle(&transform);

    for (collider, paddle) in &colliders {
        let bounding_box = Aabb2d::new(
            collider.translation.truncate(),
            collider.scale.truncate() * 0.5,
        );

        let offset = match ball_collision(&bounding_ball, &bounding_box) {
            None => continue,
            Some(offset) => offset,
        };

        resolve_ball_collision(offset, &mut transform, &mut velocity);

        if paddle.is_some() {
            // apply a random Y direction when the ball bounces off the paddle.
            let start = f32::min(-velocity.x, velocity.x);
            let end = -start;

            velocity.y = rand::thread_rng().gen_range(start..end) * 0.8;
        }
    }

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
