#![windows_subsystem = "windows"]

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

#[cfg(debug_assertions)]
fn main() {
    use bevy_inspector_egui::quick::ResourceInspectorPlugin;
    use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
                handle_menu,
                update_button_colours,
                handle_popup_buttons,
                debug_commands,
            ),
        )
        .run();
}

#[cfg(not(debug_assertions))]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
                handle_menu,
                update_button_colours,
                handle_popup_buttons,
            ),
        )
        .run();
}
