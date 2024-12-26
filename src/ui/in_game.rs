use bevy::prelude::*;

use crate::game::{
    event::GameDataUpdated,
    player::PlayerSide,
    resource::GameActiveData,
    state::{GameActiveState, GameState, InGame},
};
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
#[require(Node)]
pub struct GameScore;

#[derive(Default, Component)]
#[require(Text)]
pub struct PlayerScore {
    side: PlayerSide,
}

impl PlayerScore {
    pub const fn new_main() -> Self {
        Self {
            side: PlayerSide::Main,
        }
    }
    pub const fn new_second() -> Self {
        Self {
            side: PlayerSide::Other,
        }
    }
}

pub fn spawn_pause_menu(mut commands: Commands) {
    commands
        .spawn((PauseMenu, screen_node(), BackgroundColor(BG_COLOR)))
        .with_children(|builder| {
            builder
                .spawn((ResumeGameButton, button_node()))
                .with_child((
                    Text::new(ResumeGameButton::TEXT),
                    button_text(),
                    TextColor(ResumeGameButton::TEXT_COLOR),
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

pub fn spawn_game_score(mut commands: Commands) {
    commands
        .spawn((
            GameScore,
            Node {
                top: Val::ZERO,
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                PlayerScore::new_main(),
                Text::new("0"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
            ));
            builder.spawn((
                PlayerScore::new_second(),
                Text::new("0"),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
            ));
        });
}

pub fn despawn_game_score(query: Single<Entity, With<GameScore>>, mut commands: Commands) {
    let entity = query.into_inner();
    commands.entity(entity).despawn_recursive();
}

pub fn update_game_score(
    mut score_text: Query<(&mut Text, &PlayerScore)>,
    game_data: Res<GameActiveData>,
) {
    for (mut text, player_score) in &mut score_text {
        match player_score.side {
            PlayerSide::Main => {
                text.0 = game_data.score().player1().to_string();
            }
            PlayerSide::Other => {
                text.0 = game_data.score().player2().to_string();
            }
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(InGame), spawn_game_score);
    app.add_systems(OnExit(InGame), despawn_game_score);

    app.add_systems(OnEnter(GameActiveState::Pause), spawn_pause_menu);
    app.add_systems(OnExit(GameActiveState::Pause), despawn_pause_menu);

    app.add_systems(
        Update,
        update_game_score.run_if(on_event::<GameDataUpdated>),
    );
    app.add_systems(
        Update,
        resume_game_button.run_if(in_state(GameActiveState::Pause)),
    );
}
