use bevy::prelude::*;

use crate::game::{
    player::SecondPlayerType,
    resource::SecondPlayer,
    state::{GameActiveState, GameState, InGame},
};

use super::component::{button_node, button_text};

const SCREEN_MARGIN: Val = Val::Px(16.0);
const PLAY_BUTTON_BG_COLOR: Color = Color::WHITE;
const PLAY_BUTTON_TEXT_COLOR: Color = Color::BLACK;
const CHANGE_PLAYER_BUTTON_BG_COLOR: Color = Color::WHITE;
const CHANGE_PLAYER_TEXT_COLOR: Color = Color::BLACK;

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

pub fn spawn_main_menu(mut commands: Commands, opponent: Res<SecondPlayer>) {
    commands
        .spawn((
            MainMenu,
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                padding: UiRect::all(SCREEN_MARGIN),
                row_gap: Val::Px(8.0),
                ..default()
            },
            BackgroundColor(Color::BLACK.with_alpha(0.3)),
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    PlayButton,
                    button_node(),
                    BackgroundColor(PLAY_BUTTON_BG_COLOR),
                ))
                .with_child((
                    Text::new("Play"),
                    button_text(),
                    TextColor(PLAY_BUTTON_TEXT_COLOR),
                ));
            builder
                .spawn((
                    ChangePlayerButton,
                    button_node(),
                    BackgroundColor(CHANGE_PLAYER_BUTTON_BG_COLOR),
                ))
                .with_child((
                    Text::new(ChangePlayerButton::get_text(opponent.opponent)),
                    button_text(),
                    TextColor(CHANGE_PLAYER_TEXT_COLOR),
                ));
        });
}

pub fn despawn_main_menu(query: Single<Entity, With<MainMenu>>, mut commands: Commands) {
    let entity = query.into_inner();
    commands.entity(entity).despawn_recursive();
}

#[expect(clippy::type_complexity)]
pub fn change_player_button(
    button: Single<(&Interaction, &Children), (Changed<Interaction>, With<ChangePlayerButton>)>,
    mut text_query: Query<&mut Text>,
    mut second_player: ResMut<SecondPlayer>,
) {
    let (interaction, children) = button.into_inner();

    if *interaction == Interaction::Pressed {
        let new_opponent = second_player.opponent.change_opponent();
        second_player.opponent = new_opponent;

        let mut text = text_query
            .get_mut(children[0])
            .expect("ChangePlayerButton must have a text as first children");
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

pub fn plugin(app: &mut App) {
    app.init_state::<GameState>();
    app.add_computed_state::<GameActiveState>();
    app.add_computed_state::<InGame>();

    app.init_resource::<SecondPlayer>();

    app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu);
    app.add_systems(OnExit(GameState::MainMenu), despawn_main_menu);

    app.add_systems(
        Update,
        (change_player_button, play_button).run_if(in_state(GameState::MainMenu)),
    );
}
