use crate::components::*;
use crate::constants::*;
use crate::entities::*;
use crate::enums::*;
use crate::grid::Grid;

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

#[derive(Event)]
pub struct GridUpdatedEvent();

#[derive(Event)]
pub struct GameoverEvent();

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut grid: ResMut<Grid>,
    mut grid_updated_event: EventWriter<GridUpdatedEvent>,
) {
    grid.add_boxes(2);

    commands.spawn(Camera2dBundle::default());

    let board = commands.spawn(new_board()).insert(Name::new("Board")).id();

    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            let tile_components = new_tile(
                grid.state[i][j],
                TilePosition { i, j },
                &asset_server.load(FONT_PATH),
            );

            let tile_container = commands
                .spawn(tile_components.container)
                .insert(Name::new(format!("Tile ({i}, {j}")))
                .id();

            let tile_text = commands
                .spawn(tile_components.text)
                .insert(Name::new(format!(
                    "Tile Value ({i}, {j}, {})",
                    grid.state[i][j]
                )))
                .id();

            commands.entity(tile_container).push_children(&[tile_text]);
            commands.entity(board).push_children(&[tile_container]);
        }
    }

    let _score = commands
        .spawn(new_scoreboard(0, &asset_server.load(FONT_PATH)))
        .insert(Name::new("Score"))
        .id();

    grid_updated_event.send(GridUpdatedEvent());
}

pub fn update_grid(
    mut grid: ResMut<Grid>,
    mut key_evr: EventReader<KeyboardInput>,
    mut grid_updated_event: EventWriter<GridUpdatedEvent>,
    mut gameover_event: EventWriter<GameoverEvent>,
) {
    if grid.deadlocked {
        return;
    }

    let mut direction: Option<MoveDirection> = None;

    for event in key_evr.read() {
        match event.state {
            ButtonState::Pressed => match event.key_code {
                Some(KeyCode::W) | Some(KeyCode::Up) => {
                    direction = Some(MoveDirection::Up);
                }
                Some(KeyCode::D) | Some(KeyCode::Right) => {
                    direction = Some(MoveDirection::Right);
                }
                Some(KeyCode::S) | Some(KeyCode::Down) => {
                    direction = Some(MoveDirection::Down);
                }
                Some(KeyCode::A) | Some(KeyCode::Left) => {
                    direction = Some(MoveDirection::Left);
                }
                _ => (),
            },
            _ => (),
        }
    }

    if let Some(direction) = direction {
        let has_changed = grid.update(direction);
        if has_changed {
            grid_updated_event.send(GridUpdatedEvent());
            if grid.deadlocked {
                gameover_event.send(GameoverEvent());
            }
        }
    }
}

pub fn handle_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut gameover_event: EventReader<GameoverEvent>,
) {
    for _ in gameover_event.read() {
        let gameover_popup_components = new_gameover_popup(&asset_server.load(FONT_PATH));
        let game_over_popup = commands
            .spawn(gameover_popup_components.container)
            .insert(Name::new("Game Over Popup"))
            .id();
        let game_over_text = commands
            .spawn(gameover_popup_components.text)
            .insert(Name::new("Game Over Text"))
            .id();

        commands
            .entity(game_over_popup)
            .push_children(&[game_over_text]);

        for ButtonComponents { container, text } in gameover_popup_components.buttons.into_iter() {
            let button_container = commands.spawn(container).id();
            let button_text = commands.spawn(text).id();

            commands
                .entity(button_container)
                .push_children(&[button_text]);
            commands
                .entity(game_over_popup)
                .push_children(&[button_container]);
        }
    }
}

pub fn handle_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keys: Res<Input<KeyCode>>,
    mut menu_query: Query<Entity, With<Menu>>,
) {
    if keys.just_released(KeyCode::Escape) {
        if menu_query.iter().count() > 0 {
            for entity in &mut menu_query {
                commands.entity(entity).despawn_recursive();
            }
            return;
        }

        let menu_popup_components = new_menu_popup(&asset_server.load(FONT_PATH));
        let menu_popup = commands
            .spawn(menu_popup_components.container)
            .insert(Name::new("Menu Popup"))
            .insert(Menu)
            .id();
        let menu_text = commands
            .spawn(menu_popup_components.text)
            .insert(Name::new("Menu text"))
            .id();

        commands.entity(menu_popup).push_children(&[menu_text]);

        for ButtonComponents { container, text } in menu_popup_components.buttons.into_iter() {
            let button_container = commands.spawn(container).id();
            let button_text = commands.spawn(text).id();

            commands
                .entity(button_container)
                .push_children(&[button_text]);
            commands
                .entity(menu_popup)
                .push_children(&[button_container]);
        }
    }
}

pub fn update_scoreboard(
    grid: Res<Grid>,
    mut grid_updated_event: EventReader<GridUpdatedEvent>,
    mut query: Query<(&mut Text, &Scoreboard)>,
) {
    for _ in grid_updated_event.read() {
        for (mut text, _) in &mut query {
            text.sections[0].value = format!("Score: {}", grid.score.to_string());
        }
    }
}

pub fn update_box_values(grid: Res<Grid>, mut query: Query<(&mut Text, &TilePosition)>) {
    for (mut text, box_pos) in &mut query {
        let value = grid.state[box_pos.i][box_pos.j];
        if value != 0 {
            text.sections[0].value = value.to_string();
        } else {
            text.sections[0].value = "".to_string();
        }
    }
}

pub fn update_box_colours(
    grid: Res<Grid>,
    mut query: Query<(&mut BackgroundColor, &TilePosition), With<TileColour>>,
    mut grid_updated_event: EventReader<GridUpdatedEvent>,
) {
    for _ in grid_updated_event.read() {
        for (mut colour, box_pos) in &mut query {
            let value = grid.state[box_pos.i][box_pos.j];
            if value > 16384 {
                *colour = Color::rgb(160.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0).into();
                return;
            }
            match value {
                2 => *colour = Color::rgb(209.0 / 255.0, 205.0 / 255.0, 100.0 / 255.0).into(),
                4 => *colour = Color::rgb(209.0 / 255.0, 160.0 / 255.0, 69.0 / 255.0).into(),
                8 => *colour = Color::rgb(209.0 / 255.0, 93.0 / 255.0, 54.0 / 255.0).into(),
                16 => *colour = Color::rgb(209.0 / 255.0, 54.0 / 255.0, 54.0 / 255.0).into(),
                32 => *colour = Color::rgb(209.0 / 255.0, 65.0 / 255.0, 144.0 / 255.0).into(),
                64 => *colour = Color::rgb(162.0 / 255.0, 65.0 / 255.0, 191.0 / 255.0).into(),
                128 => *colour = Color::rgb(100.0 / 255.0, 57.0 / 255.0, 191.0 / 255.0).into(),
                256 => *colour = Color::rgb(57.0 / 255.0, 60.0 / 255.0, 191.0 / 255.0).into(),
                512 => *colour = Color::rgb(86.0 / 255.0, 127.0 / 255.0, 270.0 / 255.0).into(),
                1024 => *colour = Color::rgb(46.0 / 255.0, 164.0 / 255.0, 232.0 / 255.0).into(),
                2048 => *colour = Color::rgb(41.0 / 255.0, 212.0 / 255.0, 184.0 / 255.0).into(),
                4096 => *colour = Color::rgb(47.0 / 255.0, 194.0 / 255.0, 81.0 / 255.0).into(),
                8192 => *colour = Color::rgb(66.0 / 255.0, 143.0 / 255.0, 30.0 / 255.0).into(),
                16384 => *colour = Color::rgb(182.0 / 255.0, 227.0 / 255.0, 68.0 / 255.0).into(),
                _ => *colour = Color::rgb(25.0 / 255.0, 25.0 / 255.0, 25.0 / 255.0).into(),
            }
        }
    }
}

pub fn update_button_colours(
    mut query: Query<(&mut BackgroundColor, &ButtonColours, &Interaction), With<Button>>,
) {
    for (mut colour, button_colours, interaction) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                colour.0 = button_colours.pressed.into();
            }
            Interaction::Hovered => {
                colour.0 = button_colours.hover.into();
            }
            Interaction::None => {
                colour.0 = button_colours.none.into();
            }
        }
    }
}

pub fn handle_popup_buttons(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    mut button_query: Query<(&Interaction, &Name), (Changed<Interaction>, With<Button>)>,
    mut popup_query: Query<Entity, With<Popup>>,
    mut grid_updated_event: EventWriter<GridUpdatedEvent>,
) {
    for (interaction, name) in &mut button_query {
        if name.to_string() == ButtonType::Continue.to_string() {
            match *interaction {
                Interaction::Pressed => {
                    for entity in &mut popup_query {
                        commands.entity(entity).despawn_recursive()
                    }
                }
                _ => (),
            }
        } else if name.to_string() == ButtonType::Restart.to_string() {
            match *interaction {
                Interaction::Pressed => {
                    grid.reset();
                    grid_updated_event.send(GridUpdatedEvent());
                    for entity in &mut popup_query {
                        commands.entity(entity).despawn_recursive()
                    }
                }
                _ => (),
            }
        } else if name.to_string() == ButtonType::Exit.to_string() {
            match *interaction {
                Interaction::Pressed => {
                    std::process::exit(0);
                }
                _ => (),
            }
        }
    }
}

pub fn debug_commands(
    mut evr_char: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
    mut grid: ResMut<Grid>,
) {
    if keys.just_pressed(KeyCode::Return) {
        println!("{}", &*string);
        if &*string == "gameover" {
            grid.state = vec![
                vec![2u32, 4u32, 8u32, 16u32],
                vec![32u32, 64u32, 128u32, 256u32],
                vec![512u32, 1024u32, 2048u32, 4096u32],
                vec![8192u32, 16384u32, 32768u32, 0u32],
            ];
        }
        string.clear();
    }
    if keys.just_pressed(KeyCode::Back) {
        string.pop();
    }
    for ev in evr_char.read() {
        if !ev.char.is_control() {
            string.push(ev.char);
        }
    }
}

