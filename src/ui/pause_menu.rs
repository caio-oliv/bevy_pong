use bevy::prelude::*;

use crate::game::state::{GameActiveState, GameState, InGame};
use crate::ui::component::{button_node, button_text, screen_node};

const BG_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.1);

#[derive(Default, Component)]
#[require(Node)]
pub struct PauseMenu;

#[derive(Default, Component)]
#[require(Button)]
pub struct ResumeGameButton;

impl ResumeGameButton {
    const TEXT: &str = "Resume";
    const TEXT_COLOR: Color = Color::BLACK;
}

#[derive(Default, Component)]
#[require(Button)]
pub struct ExitToMainMenuButton;

impl ExitToMainMenuButton {
    const BG_COLOR: Color = Color::WHITE;
    const TEXT: &str = "Exit to main menu";
    const TEXT_COLOR: Color = Color::BLACK;
}

pub fn toggle_game_pause(
    input: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match game_state.get() {
            GameState::MainMenu => {}
            GameState::GameActive { playing } => {
                next_game_state.set(GameState::GameActive { playing: !playing });
            }
        }
    }
}

pub fn spawn_pause_menu(mut commands: Commands) {
    commands
        .spawn((PauseMenu, screen_node(), BackgroundColor(BG_COLOR)))
        .with_children(|builder| {
            builder
                .spawn((
                    ResumeGameButton,
                    button_node(),
                    BackgroundColor(Color::WHITE),
                ))
                .with_child((
                    Text::new(ResumeGameButton::TEXT),
                    button_text(),
                    TextColor(ResumeGameButton::TEXT_COLOR),
                ));
            builder
                .spawn((
                    ExitToMainMenuButton,
                    button_node(),
                    BackgroundColor(ExitToMainMenuButton::BG_COLOR),
                ))
                .with_child((
                    Text::new(ExitToMainMenuButton::TEXT),
                    button_text(),
                    TextColor(ExitToMainMenuButton::TEXT_COLOR),
                ));
        });
}

pub fn despawn_pause_menu(query: Single<Entity, With<PauseMenu>>, mut commands: Commands) {
    let entity = query.into_inner();
    commands.entity(entity).despawn_recursive();
}

pub fn resume_game_button(
    button: Single<&Interaction, (Changed<Interaction>, With<ResumeGameButton>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let interaction = button.into_inner();

    if *interaction == Interaction::Pressed {
        next_game_state.set(GameState::GameActive { playing: true });
    }
}

pub fn exit_to_main_menu_button(
    button: Single<&Interaction, (Changed<Interaction>, With<ExitToMainMenuButton>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let interaction = button.into_inner();

    if *interaction == Interaction::Pressed {
        next_game_state.set(GameState::MainMenu);
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameActiveState::Pause), spawn_pause_menu);
    app.add_systems(OnExit(GameActiveState::Pause), despawn_pause_menu);

    app.add_systems(Update, toggle_game_pause.run_if(in_state(InGame)));
    app.add_systems(
        Update,
        (resume_game_button, exit_to_main_menu_button).run_if(in_state(GameActiveState::Pause)),
    );
}
