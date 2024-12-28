pub mod screen {
    use bevy::prelude::*;

    pub const PADDING_PX: f32 = 16.0;
    pub const BG_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.9);

    pub fn node() -> Node {
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(PADDING_PX)),
            row_gap: Val::Px(8.0),
            ..default()
        }
    }
}

pub mod button {
    use bevy::prelude::*;

    pub const WIDTH: f32 = 128.0;
    pub const HEIGHT: f32 = 32.0;
    pub const PADDING_HORIZONTAL: f32 = 16.0;
    pub const PADDING_VERTICAL: f32 = 8.0;
    pub const FONT_SIZE: f32 = 16.0;

    pub const BG_COLOR: Color = Color::WHITE;
    pub const TEXT_COLOR: Color = Color::BLACK;

    pub fn node() -> Node {
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            min_width: Val::Px(WIDTH),
            height: Val::Px(HEIGHT),
            padding: UiRect::axes(Val::Px(PADDING_HORIZONTAL), Val::Px(PADDING_VERTICAL)),
            ..default()
        }
    }

    pub fn text_font() -> TextFont {
        TextFont {
            font_size: FONT_SIZE,
            ..default()
        }
    }
}
