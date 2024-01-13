use bevy::{
    ecs::{reflect::ReflectResource, system::Resource},
    reflect::Reflect,
};

use rand::Rng;

use crate::enums::*;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Grid {
    size: usize,
    pub state: Vec<Vec<u32>>,
    pub score: u32,
    pub deadlocked: bool,
}

impl Grid {
    pub fn new(size: usize) -> Self {
        return Grid {
            size,
            state: vec![vec![0u32; size]; size],
            score: 0,
            deadlocked: false,
        };
    }

    pub fn reset(&mut self) -> &mut Self {
        self.state = vec![vec![0u32; self.size]; self.size];
        self.score = 0;
        self.deadlocked = false;
        self.add_boxes(2);
        return self;
    }

    pub fn add_boxes(&mut self, quantity: u8) {
        let mut row = rand::thread_rng().gen_range(0..self.size);
        let mut column = rand::thread_rng().gen_range(0..self.size);
        for _i in 0..quantity {
            while self.state[row][column] != 0 {
                row = rand::thread_rng().gen_range(0..self.size);
                column = rand::thread_rng().gen_range(0..self.size);
            }
            let random = rand::thread_rng().gen_range(0..10);
            let value = if random < 9 { 2 } else { 4 };
            self.state[row][column] = value;
        }
    }

    fn compare(&self, matrix: &Vec<Vec<u32>>) -> bool {
        for i in 0..self.size {
            for j in 0..self.size {
                if self.state[i][j] != matrix[i][j] {
                    return true;
                }
            }
        }
        return false;
    }

    fn is_deadlocked(&mut self) -> bool {
        let old_state = self.state.clone();
        let mut deadlocked = true;
        for dir in DIRECTIONS {
            deadlocked = deadlocked && !self.move_in_dir(dir).compare(&old_state);
        }
        self.state = old_state;
        return deadlocked;
    }

    fn transpose(&mut self) -> &mut Self {
        let mut new_matrix = vec![vec![0u32; self.size]; self.size];
        for i in 0..self.size {
            for j in 0..self.size {
                new_matrix[j][i] = self.state[i][j]
            }
        }
        self.state = new_matrix;
        return self;
    }

    fn reverse(&mut self) -> &mut Self {
        let mut new_matrix = vec![vec![0u32; self.size]; self.size];
        for i in 0..self.size {
            for j in 0..self.size {
                new_matrix[i][j] = self.state[i][self.size - 1 - j];
            }
        }
        self.state = new_matrix;
        return self;
    }

    fn compress(&mut self) -> &mut Self {
        let mut new_matrix = vec![vec![0u32; self.size]; self.size];
        for i in 0..self.size {
            let mut index: usize = 0;
            for j in 0..self.size {
                if self.state[i][j] != 0 {
                    new_matrix[i][index] = self.state[i][j];
                    index += 1;
                }
            }
        }
        self.state = new_matrix;
        return self;
    }

    fn merge(&mut self) -> &mut Self {
        let mut new_matrix = vec![vec![0u32; self.size]; self.size];
        for i in 0..self.size {
            let mut pair = false;
            for j in 0..self.size {
                if pair {
                    new_matrix[i][j] = self.state[i][j] * 2;
                    self.score += self.state[i][j] * 2;
                    pair = false;
                } else if j + 1 < self.size && self.state[i][j] == self.state[i][j + 1] {
                    pair = true;
                } else {
                    new_matrix[i][j] = self.state[i][j];
                }
            }
        }
        self.state = new_matrix;
        return self;
    }

    fn move_left(&mut self) -> &mut Self {
        return self.compress().merge().compress();
    }

    pub fn move_in_dir(&mut self, direction: MoveDirection) -> &mut Self {
        match direction {
            MoveDirection::Left => {
                self.move_left();
            }
            MoveDirection::Right => {
                self.reverse().move_left().reverse();
            }
            MoveDirection::Up => {
                self.transpose().move_left().transpose();
            }
            MoveDirection::Down => {
                self.transpose().reverse().move_left().reverse().transpose();
            }
        }
        return self;
    }

    pub fn update(&mut self, direction: MoveDirection) -> bool {
        let old_state = self.state.clone();
        self.move_in_dir(direction);

        let has_changed = self.compare(&old_state);
        if has_changed {
            self.add_boxes(1);
            self.deadlocked = self.is_deadlocked();
        } else {
            self.state = old_state;
        }

        return has_changed;
    }
}

