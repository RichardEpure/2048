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

