pub mod grid;

use bevy::{
    ecs::{reflect::ReflectResource, system::Resource},
    reflect::Reflect,
};

use crate::enums::GameState;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Game {
    pub state: GameState,
}

