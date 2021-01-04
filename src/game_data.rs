use std::cell::RefCell;

use crate::game_2048_matrix::{GameAction, GameActionReporter};

pub struct GameData {
    score: RefCell<u32>,
    max_number: RefCell<u32>,
}

impl GameData {
    pub fn new() -> GameData {
        GameData {
            score: RefCell::new(0),
            max_number: RefCell::new(0),
        }
    }

    pub fn get_score(&self) -> u32 {
        *self.score.borrow_mut()
    }

    pub fn set_score(&self, value: u32) {
        *self.score.borrow_mut() = value;
    }

    pub fn get_max_number(&self) -> u32 {
        *self.max_number.borrow_mut()
    }

    pub fn set_max_number(&self, value: u32) {
        *self.max_number.borrow_mut() = value;
    }
}

impl GameActionReporter for GameData {
    fn report(&self, action: GameAction) {
        match action {
            GameAction::Merge { merge_result, .. } => {
                self.set_score(self.get_score() + merge_result);
                if self.get_max_number() < merge_result {
                    self.set_max_number(merge_result);
                }
            }
            _ => (),
        }
    }
}
