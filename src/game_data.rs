use std::cell::Cell;

use crate::game_2048_matrix::{GameAction, GameActionReporter};

pub struct GameData {
    pub score: Cell<u32>,
    pub max_number: Cell<u32>,
}

impl GameData {
    pub fn new() -> GameData {
        GameData {
            score: Cell::new(0),
            max_number: Cell::new(0),
        }
    }
}

impl GameActionReporter for GameData {
    fn report(&self, action: GameAction) {
        if let GameAction::Merge { merge_result, .. } = action {
            self.score.set(self.score.get() + merge_result);
            if self.max_number.get() < merge_result {
                self.max_number.set(merge_result);
            }
        }
    }
}
