#![allow(dead_code)]
use crate::components::*;
use crate::constants::*;
// use crate::grid::Grid;

use bevy::prelude::*;

pub struct ButtonComponents {
    pub container: (ButtonBundle, ButtonColours),
    pub text: TextBundle,
}

pub struct TileComponents {
    pub container: (NodeBundle, TileColour, TilePosition),
    pub text: (TextBundle, TilePosition),
}

pub struct GameoverPopupComponents {
    pub container: (NodeBundle, GameoverPopup),
    pub text: TextBundle,
    pub restart_button: ButtonComponents,
    pub exit_button: ButtonComponents,
}

pub fn new_button(text: String, font: &Handle<Font>, colours: ButtonColours) -> ButtonComponents {
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
        ),
        text: TextBundle {
            text: Text::from_section(
                text,
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

pub fn new_gameover_popup(font: &Handle<Font>) -> GameoverPopupComponents {
    return GameoverPopupComponents {
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
            GameoverPopup,
        ),
        text: TextBundle {
            text: Text::from_section(
                "Game Over".to_string(),
                TextStyle {
                    font: font.clone(),
                    font_size: 48.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            ..default()
        },
        restart_button: new_button(
            "Restart".to_string(),
            font,
            ButtonColours {
                pressed: BUTTON_GREEN_PRESSED,
                hover: BUTTON_GREEN_HOVER,
                none: BUTTON_GREEN,
            },
        ),
        exit_button: new_button(
            "Exit".to_string(),
            font,
            ButtonColours {
                pressed: BUTTON_GREEN_PRESSED,
                hover: BUTTON_GREEN_HOVER,
                none: BUTTON_GREEN,
            },
        ),
    };
}

