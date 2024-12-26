use bevy::prelude::*;

use crate::game::{
    event::GameDataUpdated,
    player::PlayerSide,
    resource::{GameActiveData, StartMatchTimer},
    state::InGame,
};

#[derive(Default, Component)]
#[require(Node)]
pub struct GameOSD;

impl GameOSD {
    pub fn node() -> Node {
        Node {
            left: Val::ZERO,
            right: Val::ZERO,
            top: Val::ZERO,
            bottom: Val::ZERO,
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Stretch,
            aspect_ratio: Some(16.0 / 9.0),
            height: Val::Vh(100.0),
            margin: UiRect::axes(Val::Auto, Val::ZERO),
            ..Node::DEFAULT
        }
    }
}

#[derive(Default, Component)]
#[require(Node)]
pub struct GameScore;

impl GameScore {
    pub fn node() -> Node {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            width: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(24.0)),
            ..Node::DEFAULT
        }
    }
}

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

#[derive(Default, Component)]
#[require(Node)]
pub struct StartMatchCountdown;

impl StartMatchCountdown {
    pub fn node() -> Node {
        Node {
            position_type: PositionType::Absolute,
            left: Val::ZERO,
            right: Val::ZERO,
            top: Val::ZERO,
            bottom: Val::ZERO,
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Auto,
            padding: UiRect::bottom(Val::Px(96.0)),
            ..Node::DEFAULT
        }
    }
}

#[derive(Default, Component)]
#[require(Text)]
pub struct StartMatchCountdownText;

pub fn spawn_osd(mut commands: Commands) {
    commands
        .spawn((GameOSD, GameOSD::node()))
        .with_children(|builder| {
            build_game_score(builder);
            build_start_match_countdown(builder);
        });
}

pub fn despawn_osd(osd: Single<Entity, With<GameOSD>>, mut commands: Commands) {
    let entity = osd.into_inner();
    commands.entity(entity).despawn_recursive();
}

pub fn build_game_score(builder: &mut ChildBuilder<'_>) {
    builder
        .spawn((GameScore, GameScore::node()))
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

pub fn build_start_match_countdown(builder: &mut ChildBuilder<'_>) {
    builder
        .spawn((StartMatchCountdown, StartMatchCountdown::node()))
        .with_child((
            StartMatchCountdownText,
            Text::new(String::new()),
            TextFont {
                font_size: 64.0,
                ..default()
            },
        ));
}

pub fn update_start_match_countdown(
    countdown_text: Single<&mut Text, With<StartMatchCountdownText>>,
    timer: Res<StartMatchTimer>,
) {
    let mut text = countdown_text.into_inner();
    let num: u32 = timer.0.elapsed_secs().trunc() as u32;
    text.0 = (StartMatchTimer::SECONDS - num).to_string();
}

pub fn hide_start_match_countdown(
    countdown_text: Single<&mut Text, With<StartMatchCountdownText>>,
) {
    let mut text = countdown_text.into_inner();
    text.0 = String::new();
}

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(InGame), spawn_osd);
    app.add_systems(OnExit(InGame), despawn_osd);

    app.add_systems(
        Update,
        update_game_score.run_if(on_event::<GameDataUpdated>),
    );
    app.add_systems(
        Update,
        update_start_match_countdown.run_if(resource_exists::<StartMatchTimer>),
    );
    app.add_systems(
        Update,
        hide_start_match_countdown.run_if(resource_removed::<StartMatchTimer>),
    );
}
