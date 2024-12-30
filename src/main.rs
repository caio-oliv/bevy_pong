use bevy::prelude::*;

use settings::camera::orthographic_projection;

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

    #[cfg(feature = "devtools")]
    app.add_plugins(devtools_plugin);

    app.insert_resource(ClearColor(Color::BLACK));

    app.add_systems(Startup, spawn_camera);
}

#[cfg(feature = "devtools")]
pub fn devtools_plugin(app: &mut App) {
    use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

    app.add_plugins(FrameTimeDiagnosticsPlugin);
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d, orthographic_projection()));
}
