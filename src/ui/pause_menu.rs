use bevy::prelude::*;

use crate::game::resource::UserGamepad;
use crate::game::state::{GameActiveState, GameState, InGame};
use crate::ui::component::{button, screen};

#[derive(Default, Component)]
#[require(Node)]
pub struct PauseMenu;

#[derive(Default, Component)]
#[require(Button)]
pub struct ResumeGameButton;

impl ResumeGameButton {
    const TEXT: &str = "Resume";
}

#[derive(Default, Component)]
#[require(Button)]
pub struct ExitToMainMenuButton;

impl ExitToMainMenuButton {
    const TEXT: &str = "Exit to main menu";
}

pub fn toggle_game_pause(
    gamepads: Query<&Gamepad>,
    user_gamepad: Res<UserGamepad>,
    keyboard: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let gamepad = user_gamepad
        .get_main()
        .and_then(|entity| gamepads.get(entity).ok());

    if keyboard.just_pressed(KeyCode::Escape)
        || gamepad.is_some_and(|gpad| gpad.just_pressed(GamepadButton::Start))
    {
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
        .spawn((PauseMenu, screen::node(), BackgroundColor(screen::BG_COLOR)))
        .with_children(|builder| {
            builder
                .spawn((
                    ResumeGameButton,
                    button::node(),
                    BackgroundColor(button::BG_COLOR),
                ))
                .with_child((
                    Text::new(ResumeGameButton::TEXT),
                    button::text_font(),
                    TextColor(button::TEXT_COLOR),
                ));
            builder
                .spawn((
                    ExitToMainMenuButton,
                    button::node(),
                    BackgroundColor(button::BG_COLOR),
                ))
                .with_child((
                    Text::new(ExitToMainMenuButton::TEXT),
                    button::text_font(),
                    TextColor(button::TEXT_COLOR),
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
