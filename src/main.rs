use avian2d::prelude::*;
use bevy::prelude::*;

use settings::{camera::default_orthographic_projection, scale::PHYSICS_LENGTH_UNIT};

mod game;
mod settings;
mod ui;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        app_plugin,
        ui::main_menu::plugin,
        game::plugin,
        ui::in_game::plugin,
        ui::pause_menu::plugin,
    ));

    app.run();
}

fn app_plugin(app: &mut App) {
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(settings::window::primary_window()),
        ..Default::default()
    }));

    // TODO: Run physics world only when the game is running.
    app.add_plugins(PhysicsPlugins::default().with_length_unit(PHYSICS_LENGTH_UNIT));

    #[cfg(feature = "devtools")]
    app.add_plugins(devtools_plugin);

    app.insert_resource(ClearColor(Color::BLACK));
    app.insert_resource(Gravity(Vec2::ZERO));

    app.add_systems(Startup, spawn_camera);
}

#[cfg(feature = "devtools")]
pub fn devtools_plugin(app: &mut App) {
    use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
    app.add_plugins((FrameTimeDiagnosticsPlugin, PhysicsDebugPlugin::default()));
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, default_orthographic_projection()));
}
