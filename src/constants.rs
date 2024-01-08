use bevy::prelude::*;

pub const GRID_SIZE: usize = 4;
pub const TILE_SIZE: Vec2 = Vec2::new(128.0, 128.0);
pub const TILE_MARGIN: f32 = 10.0;
pub const BACKGROUND_COLOUR: Color = Color::rgb(51.0 / 255.0, 51.0 / 255.0, 51.0 / 255.0);
pub const BUTTON_GREEN: Color = Color::rgb(31.0 / 255.0, 148.0 / 255.0, 24.0 / 255.0);
pub const BUTTON_GREEN_HOVER: Color = Color::rgb(31.0 / 255.0, 175.0 / 255.0, 24.0 / 255.0);
pub const BUTTON_GREEN_PRESSED: Color = Color::rgb(35.0 / 255.0, 111.0 / 255.0, 31.0 / 255.0);
pub const BUTTON_RED: Color = Color::rgb(148.0 / 255.0, 31.0 / 255.0, 24.0 / 255.0);
pub const BUTTON_RED_HOVER: Color = Color::rgb(175.0 / 255.0, 31.0 / 255.0, 24.0 / 255.0);
pub const BUTTON_RED_PRESSED: Color = Color::rgb(111.0 / 255.0, 35.0 / 255.0, 31.0 / 255.0);
pub const FONT_PATH: &str = "fonts/FiraSans-Bold.ttf";

