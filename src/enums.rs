#[derive(Debug)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Left,
    Direction::Down,
    Direction::Right,
];

