use std::ops::{Add, Index, IndexMut, Mul};

pub enum GameAction {
    Merge {
        from: usize,
        to: usize,
        merge_result: u32,
    },
    Move {
        from: usize,
        to: usize,
    },
}

pub enum MoveOrientation {
    Left,
    Up,
    Right,
    Down,
}

pub trait GameActionReporter {
    fn report(&self, action: GameAction);
}

pub struct Game2048Matrix<'a, T: GameActionReporter> {
    storage: Vec<u32>,
    matrix_order: usize,
    reporter: Option<&'a T>,
}

impl<'a, T: GameActionReporter> Game2048Matrix<'a, T> {
    pub fn new(matrix_order: usize, reporter: &'a T) -> Game2048Matrix<'a, T> {
        Game2048Matrix {
            storage: vec![0; matrix_order * matrix_order],
            matrix_order,
            reporter: Some(reporter),
        }
    }

    pub fn get_matrix_order(&self) -> usize {
        self.matrix_order
    }

    pub fn move_to(&mut self, orientation: MoveOrientation) {
        let length = self.matrix_order as i32;
        let linear_function = &match orientation {
            MoveOrientation::Left => get_linear_function(length, 1, 0),
            MoveOrientation::Up => get_linear_function(1, length, 0),
            MoveOrientation::Right => get_linear_function(length, -1, length - 1),
            MoveOrientation::Down => get_linear_function(1, -length, length * (length - 1)),
        };

        for i in 0..self.matrix_order {
            let index_translator = |index| linear_function(i as i32, index as i32) as usize;
            self.move_and_merge_array(index_translator);
        }
    }

    fn move_and_merge_array(&mut self, index_translator: impl Fn(usize) -> usize) {
        let mut p_current = 0;
        for p_next in 1..self.matrix_order {
            let next = self.get(p_next, &index_translator);
            if next == 0 {
                continue;
            }

            let current = self.get(p_current, &index_translator);
            if current == next {
                let merge_result = next * 2;
                self.set(p_current, merge_result, &index_translator);
                self.set(p_next, 0, &index_translator);
                self.raise_action(GameAction::Merge {
                    from: p_next,
                    to: p_current,
                    merge_result,
                });
                p_current += 1;
            } else {
                if current != 0 {
                    p_current += 1;
                }

                if p_current != p_next {
                    self.set(p_current, next, &index_translator);
                    self.set(p_next, 0, &index_translator);
                    self.raise_action(GameAction::Move {
                        from: p_next,
                        to: p_current,
                    });
                }
            }
        }
    }

    fn raise_action(&mut self, action: GameAction) {
        match self.reporter {
            Some(ref mut reporter) => (*reporter).report(action),
            None => (),
        }
    }

    fn get(&self, index: usize, index_translator: impl Fn(usize) -> usize) -> u32 {
        self.storage[index_translator(index)]
    }

    fn set(&mut self, index: usize, value: u32, index_translator: impl Fn(usize) -> usize) {
        self.storage[index_translator(index)] = value;
    }
}

impl<'a, T: GameActionReporter> Index<(usize, usize)> for Game2048Matrix<'a, T> {
    type Output = u32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.storage[index.0 * self.matrix_order + index.1]
    }
}

impl<'a, T: GameActionReporter> IndexMut<(usize, usize)> for Game2048Matrix<'a, T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.storage[index.0 * self.matrix_order + index.1]
    }
}

impl<'a, T: GameActionReporter> PartialEq for Game2048Matrix<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        if self.storage.len() != other.storage.len() {
            return false;
        }

        for i in 0..self.storage.len() {
            if self.storage[i] != other.storage[i] {
                return false;
            }
        }

        return true;
    }
}

impl<'a, T: GameActionReporter> Clone for Game2048Matrix<'a, T> {
    fn clone(&self) -> Self {
        Game2048Matrix {
            storage: self.storage.clone(),
            matrix_order: self.matrix_order,
            reporter: None,
        }
    }
}

fn get_linear_function<T>(a: T, b: T, c: T) -> impl Fn(T, T) -> T
where
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    move |x, y| (a * x) + (b * y) + c
}
