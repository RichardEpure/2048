#![windows_subsystem = "windows"]

mod components;
mod constants;
mod entities;
mod enums;
mod resources;
mod systems;

use constants::*;
use enums::GameState;
use resources::grid::Grid;
use resources::Game;
use systems::*;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    #[cfg(feature = "debug-inspector")]
    {
        use bevy_inspector_egui::quick::ResourceInspectorPlugin;
        use bevy_inspector_egui::quick::WorldInspectorPlugin;

        app.add_plugins(WorldInspectorPlugin::new())
            .add_plugins(ResourceInspectorPlugin::<Grid>::default())
            .add_plugins(ResourceInspectorPlugin::<Game>::default());
    }

    #[cfg(feature = "debug-commands")]
    {
        app.add_systems(Update, debug_commands);
    }

    app.insert_resource(Grid::new(GRID_SIZE))
        .insert_resource(Game {
            state: GameState::Play,
        })
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
