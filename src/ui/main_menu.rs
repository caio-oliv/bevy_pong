use bevy::prelude::*;

use crate::game::{
    player::SecondPlayerType,
    resource::{SecondPlayer, UserGamepad},
    state::{GameActiveState, GameState, InGame},
};
use crate::ui::component::{button, screen};

#[derive(Default, Component)]
#[require(Node)]
pub struct MainMenu;

#[derive(Default, Component)]
#[require(Button)]
pub struct PlayButton;

#[derive(Default, Component)]
#[require(Button)]
pub struct ChangePlayerButton;

impl ChangePlayerButton {
    const TWO_PLAYERS_TEXT: &str = "2 Players";
    const AI_TEXT: &str = "AI";

    const fn get_text(player: SecondPlayerType) -> &'static str {
        match player {
            SecondPlayerType::Player => Self::TWO_PLAYERS_TEXT,
            SecondPlayerType::AI => Self::AI_TEXT,
        }
    }
}

#[derive(Default, Component)]
#[require(Text)]
pub struct ChangePlayerButtonText;

#[derive(Default, Component)]
#[require(Button)]
pub struct ExitGameButton;

impl ExitGameButton {
    const TEXT: &str = "Exit";

    fn node() -> Node {
        let mut node = button::node();
        node.margin.top = Val::Px(32.0);
        node
    }
}

pub fn spawn_main_menu(mut commands: Commands, second_player: Res<SecondPlayer>) {
    commands
        .spawn((MainMenu, screen::node(), BackgroundColor(screen::BG_COLOR)))
        .with_children(|builder| {
            build_play_button(builder);
            build_change_player_button(builder, &second_player);
            build_exit_game_button(builder);
        });
}

pub fn build_play_button(builder: &mut ChildBuilder<'_>) {
    builder
        .spawn((
            PlayButton,
            button::node(),
            BackgroundColor(button::BG_COLOR),
        ))
        .with_child((
            Text::new("Play"),
            button::text_font(),
            TextColor(button::TEXT_COLOR),
        ));
}

pub fn build_change_player_button(builder: &mut ChildBuilder<'_>, second_player: &SecondPlayer) {
    builder
        .spawn((
            ChangePlayerButton,
            button::node(),
            BackgroundColor(button::BG_COLOR),
        ))
        .with_child((
            ChangePlayerButtonText,
            Text::new(ChangePlayerButton::get_text(second_player.opponent)),
            button::text_font(),
            TextColor(button::TEXT_COLOR),
        ));
}

pub fn build_exit_game_button(builder: &mut ChildBuilder<'_>) {
    builder
        .spawn((
            ExitGameButton,
            ExitGameButton::node(),
            BackgroundColor(button::BG_COLOR),
        ))
        .with_child((
            Text::new(ExitGameButton::TEXT),
            button::text_font(),
            TextColor(button::TEXT_COLOR),
        ));
}

pub fn despawn_main_menu(query: Single<Entity, With<MainMenu>>, mut commands: Commands) {
    let entity = query.into_inner();
    commands.entity(entity).despawn_recursive();
}

pub fn change_player_button(
    button: Single<&Interaction, (Changed<Interaction>, With<ChangePlayerButton>)>,
    button_text: Single<&mut Text, With<ChangePlayerButtonText>>,
    mut second_player: ResMut<SecondPlayer>,
) {
    let interaction = button.into_inner();
    let mut text = button_text.into_inner();

    if *interaction == Interaction::Pressed {
        let new_opponent = second_player.opponent.change_opponent();
        second_player.opponent = new_opponent;

        text.0 = ChangePlayerButton::get_text(new_opponent).to_string();
    }
}

pub fn change_player_with_gamepad(
    button_text: Single<&mut Text, With<ChangePlayerButtonText>>,
    gamepads: Query<&Gamepad>,
    user_gamepad: Res<UserGamepad>,
    mut second_player: ResMut<SecondPlayer>,
) {
    let mut text = button_text.into_inner();

    let gamepad = user_gamepad
        .get_main()
        .and_then(|entity| gamepads.get(entity).ok());

    if gamepad.is_some_and(|gpad| {
        gpad.any_just_pressed([GamepadButton::DPadLeft, GamepadButton::DPadRight])
    }) {
        let new_opponent = second_player.opponent.change_opponent();
        second_player.opponent = new_opponent;

        text.0 = ChangePlayerButton::get_text(new_opponent).to_string();
    }
}

pub fn play_button(
    button: Single<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let interaction = button.into_inner();

    if *interaction == Interaction::Pressed {
        next_game_state.set(GameState::playing());
    }
}

pub fn start_game_with_gamepad(
    gamepads: Query<&Gamepad>,
    user_gamepad: Res<UserGamepad>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let gamepad = user_gamepad
        .get_main()
        .and_then(|entity| gamepads.get(entity).ok());

    if gamepad.is_some_and(|gpad| gpad.just_pressed(GamepadButton::Start)) {
        next_game_state.set(GameState::playing());
    }
}

pub fn exit_game_button(
    button: Single<&Interaction, (Changed<Interaction>, With<ExitGameButton>)>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    let interaction = button.into_inner();

    if *interaction == Interaction::Pressed {
        app_exit_events.send(AppExit::Success);
    }
}

pub fn plugin(app: &mut App) {
    app.init_state::<GameState>();
    app.add_computed_state::<GameActiveState>();
    app.add_computed_state::<InGame>();

    app.init_resource::<SecondPlayer>();

    app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu);
    app.add_systems(OnExit(GameState::MainMenu), despawn_main_menu);

    app.add_systems(
        Update,
        (
            change_player_button,
            change_player_with_gamepad,
            play_button,
            start_game_with_gamepad,
            exit_game_button,
        )
            .run_if(in_state(GameState::MainMenu)),
    );
}
