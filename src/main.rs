mod components;
mod constants;
mod entities;
mod enums;
mod grid;
mod systems;

use constants::*;
use grid::Grid;
use systems::*;

use bevy::prelude::*;

use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ResourceInspectorPlugin::<Grid>::default())
        .insert_resource(Grid::new(GRID_SIZE))
        .add_event::<GridUpdatedEvent>()
        .add_event::<GameoverEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_grid,
                update_scoreboard,
                update_box_values,
                update_box_colours,
                handle_game_over,
                update_button_colours,
                handle_gameover_popup_buttons,
            ),
        )
        .run();
}
