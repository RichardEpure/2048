use bevy::reflect::Reflect;

#[derive(Debug)]
pub enum MoveDirection {
    Up,
    Left,
    Down,
    Right,
}

pub const DIRECTIONS: [MoveDirection; 4] = [
    MoveDirection::Up,
    MoveDirection::Left,
    MoveDirection::Down,
    MoveDirection::Right,
];

pub enum ButtonType {
    Continue,
    Restart,
    Exit,
}

impl ButtonType {
    pub fn to_string(&self) -> String {
        match self {
            ButtonType::Continue => "Continue".to_string(),
            ButtonType::Restart => "Restart".to_string(),
            ButtonType::Exit => "Exit".to_string(),
        }
    }
}

#[derive(Reflect, PartialEq)]
pub enum GameState {
    Play,
    Menu,
    Gameover,
}

impl Default for GameState {
    fn default() -> Self {
        return GameState::Play;
    }
}

