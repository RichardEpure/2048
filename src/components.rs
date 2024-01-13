use bevy::prelude::*;

#[derive(Component)]
pub struct TileColour;

#[derive(Component, Copy, Clone)]
pub struct TilePosition {
    pub i: usize,
    pub j: usize,
}

#[derive(Component)]
pub struct Scoreboard;

#[derive(Component)]
pub struct Popup;

#[derive(Component)]
pub struct Menu;

#[derive(Component, Copy, Clone)]
pub struct ButtonColours {
    pub pressed: Color,
    pub hover: Color,
    pub none: Color,
}

