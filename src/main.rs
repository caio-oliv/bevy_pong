use bevy::prelude::*;
use rand::Rng;

use component::*;

mod component;
mod settings;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: settings::default_window_resolution(),
            resize_constraints: settings::WINDOW_RESIZE_CONSTRAINTS,
            resizable: false,
            ..Default::default()
        }),
        ..Default::default()
    }));

    app.add_systems(
        Startup,
        (spawn_camera, spawn_play_area, spawn_players, spawn_ball),
    );

    app.add_systems(Update, (move_paddle, move_ball, ball_collide));

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_play_area(mut commands: Commands) {
    commands.spawn((PlayArea, PlayArea::new_sprite()));
}

fn spawn_players(mut commands: Commands) {
    commands.spawn((
        Player,
        Paddle::new_main(),
        Sprite::sized(Paddle::SIZE),
        Paddle::new_main_transform(),
    ));
    commands.spawn((
        Player,
        Paddle::new_secondary(),
        Sprite::sized(Paddle::SIZE),
        Paddle::new_secondary_transform(),
    ));
}

fn spawn_ball(mut commands: Commands) {
    let ball = Ball {
        size: Ball::DEFAULT_SIZE,
        velocity: Vec2::new(100., 0.),
    };
    commands.spawn((ball.new_sprite(), Ball::new_transform(), ball));
}

fn move_paddle(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut paddle_t, settings) in &mut paddles {
        if input.pressed(settings.move_up) {
            paddle_t.translation.y += 100.0 * time.delta_secs();
        }
        if input.pressed(settings.move_down) {
            paddle_t.translation.y -= 100.0 * time.delta_secs();
        }

        paddle_t.translation.y = paddle_t.translation.y.clamp(
            -PlayArea::SIZE.y * 0.5 + Paddle::SIZE.y * 0.5,
            PlayArea::SIZE.y * 0.5 - Paddle::SIZE.y * 0.5,
        )
    }
}

fn move_ball(mut balls: Query<(&mut Transform, &Ball)>, time: Res<Time>) {
    for (mut ball_t, ball) in &mut balls {
        ball_t.translation += (ball.velocity * time.delta_secs()).extend(0.0);
    }
}

fn ball_collide(
    mut balls: Query<(&Transform, &mut Ball)>,
    paddles: Query<&Transform, With<Paddle>>,
) {
    for (ball_t, mut ball) in &mut balls {
        if ball_t.translation.y.abs() + ball.size.x * 0.5 > PlayArea::SIZE.y * 0.5 {
            ball.velocity.y *= -1.0;
        }

        for paddle in &paddles {
            if aabb_2d(
                ball_t.translation,
                ball.size,
                paddle.translation,
                Paddle::SIZE,
            ) {
                ball.velocity *= -1.;
                ball.velocity.y = rand::thread_rng().gen_range(-1.0..1.0) * 100.0;
            }
        }
    }
}

const fn aabb_2d(
    box1_translation: Vec3,
    box1_size: Vec2,
    box2_translation: Vec3,
    box2_size: Vec2,
) -> bool {
    box1_translation.x - box1_size.x * 0.5 < box2_translation.x + box2_size.x * 0.5
        && box1_translation.y - box1_size.x * 0.5 < box2_translation.y + box2_size.y * 0.5
        && box1_translation.x + box1_size.x * 0.5 > box2_translation.x - box2_size.x * 0.5
        && box1_translation.y + box1_size.x * 0.5 > box2_translation.y - box2_size.y * 0.5
}
