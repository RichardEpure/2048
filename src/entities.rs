use crate::components::*;
use crate::constants::*;
use crate::enums::*;

use bevy::prelude::*;

pub struct ButtonComponents {
    pub container: (ButtonBundle, ButtonColours, Name),
    pub text: TextBundle,
}

pub struct TileComponents {
    pub container: (NodeBundle, TileColour, TilePosition),
    pub text: (TextBundle, TilePosition),
}

pub struct PopupComponents {
    pub container: (NodeBundle, Popup),
    pub text: TextBundle,
    pub buttons: Vec<ButtonComponents>,
}

pub fn new_button(text: &String, font: &Handle<Font>, colours: ButtonColours) -> ButtonComponents {
    return ButtonComponents {
        container: (
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: colours.none.into(),
                ..default()
            },
            colours,
            Name::new(text.clone()),
        ),
        text: TextBundle {
            text: Text::from_section(
                text.clone(),
                TextStyle {
                    font: font.clone(),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            ..default()
        },
    };
}

pub fn new_board() -> NodeBundle {
    return NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Grid,
            justify_content: JustifyContent::Center,
            align_content: AlignContent::Center,
            grid_template_columns: vec![GridTrack::px(TILE_SIZE.y); GRID_SIZE],
            grid_template_rows: vec![GridTrack::px(TILE_SIZE.x); GRID_SIZE],
            row_gap: Val::Px(TILE_MARGIN),
            column_gap: Val::Px(TILE_MARGIN),
            ..default()
        },
        background_color: BACKGROUND_COLOUR.into(),
        ..default()
    };
}

pub fn new_tile(value: u32, tile_position: TilePosition, font: &Handle<Font>) -> TileComponents {
    return TileComponents {
        container: (
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    width: Val::Px(TILE_SIZE.x),
                    height: Val::Px(TILE_SIZE.y),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            TileColour,
            tile_position,
        ),
        text: (
            TextBundle {
                text: Text::from_section(
                    value.to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 48.0,
                        color: Color::WHITE,
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..default()
            },
            tile_position,
        ),
    };
}

pub fn new_scoreboard(value: u32, font: &Handle<Font>) -> (TextBundle, Scoreboard) {
    return (
        TextBundle {
            text: Text::from_section(
                format!("Score: {}", value.to_string()),
                TextStyle {
                    font: font.clone(),
                    font_size: 48.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(15.0),
                justify_self: JustifySelf::Center,
                ..default()
            },
            z_index: ZIndex::Global(1),
            ..default()
        },
        Scoreboard,
    );
}

pub fn new_popup(
    text: &String,
    font: &Handle<Font>,
    buttons: &Vec<(String, ButtonColours)>,
) -> PopupComponents {
    let mut button_components: Vec<ButtonComponents> = vec![];
    for (text, colours) in buttons.iter() {
        button_components.push(new_button(text, font, *colours))
    }

    return PopupComponents {
        container: (
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_self: JustifySelf::Center,
                    align_self: AlignSelf::Center,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(40.0),
                    ..default()
                },
                background_color: BACKGROUND_COLOUR.into(),
                ..default()
            },
            Popup,
        ),
        text: TextBundle {
            text: Text::from_section(
                text,
                TextStyle {
                    font: font.clone(),
                    font_size: 48.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            ..default()
        },
        buttons: button_components,
    };
}

pub fn new_gameover_popup(font: &Handle<Font>) -> PopupComponents {
    return new_popup(
        &"Game Over".to_string(),
        font,
        &vec![
            (
                ButtonType::Restart.to_string(),
                ButtonColours {
                    pressed: BUTTON_GREEN_PRESSED.into(),
                    hover: BUTTON_GREEN_HOVER.into(),
                    none: BUTTON_GREEN.into(),
                },
            ),
            (
                ButtonType::Exit.to_string(),
                ButtonColours {
                    pressed: BUTTON_RED_PRESSED.into(),
                    hover: BUTTON_RED_HOVER.into(),
                    none: BUTTON_RED.into(),
                },
            ),
        ],
    );
}

pub fn new_menu_popup(font: &Handle<Font>) -> PopupComponents {
    return new_popup(
        &"2048".to_string(),
        font,
        &vec![
            (
                ButtonType::Continue.to_string(),
                ButtonColours {
                    pressed: BUTTON_GREEN_PRESSED.into(),
                    hover: BUTTON_GREEN_HOVER.into(),
                    none: BUTTON_GREEN.into(),
                },
            ),
            (
                ButtonType::Restart.to_string(),
                ButtonColours {
                    pressed: BUTTON_GREEN_PRESSED.into(),
                    hover: BUTTON_GREEN_HOVER.into(),
                    none: BUTTON_GREEN.into(),
                },
            ),
            (
                ButtonType::Exit.to_string(),
                ButtonColours {
                    pressed: BUTTON_RED_PRESSED.into(),
                    hover: BUTTON_RED_HOVER.into(),
                    none: BUTTON_RED.into(),
                },
            ),
        ],
    );
}

