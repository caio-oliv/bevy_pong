use bevy::prelude::*;

const SCREEN_MARGIN: Val = Val::Px(16.0);

pub fn screen_node() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        padding: UiRect::all(SCREEN_MARGIN),
        row_gap: Val::Px(8.0),
        ..default()
    }
}

pub fn button_node() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        min_width: Val::Px(64.0),
        height: Val::Px(32.0),
        padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
        ..default()
    }
}

pub fn button_text() -> TextFont {
    TextFont {
        font_size: 16.0,
        ..default()
    }
}
