use std::ops::{Add, Mul};

pub enum GameAction {
    Merge {
        from: usize,
        to: usize,
        merge_result: i32,
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

pub struct Game2048Matrix {
    storage: Vec<i32>,
    matrix_order: usize,
}

impl Game2048Matrix {
    pub fn new(matrix_order: usize) -> Game2048Matrix {
        Game2048Matrix {
            storage: Vec::with_capacity(matrix_order * matrix_order),
            matrix_order,
        }
    }

    pub fn move_to(&mut self, orientation: MoveOrientation) {
        let length = self.matrix_order as i32;
        let linear_function = &match orientation {
            MoveOrientation::Left => get_linear_function(length, 1, 0),
            MoveOrientation::Up => get_linear_function(1, length, 0),
            MoveOrientation::Right => get_linear_function(length, -1, length - 1),
            MoveOrientation::Down => get_linear_function(1, -length, length * (length - 1)),
        };
        let reporter = |_action| {};

        for i in 0..self.matrix_order {
            let index_translator = |index| linear_function(i as i32, index as i32) as usize;
            self.move_and_merge_array(&index_translator, &reporter);
        }
    }

    fn move_and_merge_array(
        &mut self,
        index_translator: &impl Fn(usize) -> usize,
        reporter: &impl Fn(GameAction) -> (),
    ) {
        let mut p_current = 0;
        for p_next in 1..self.matrix_order {
            let next = self.get(p_next, index_translator);
            if next == 0 {
                continue;
            }

            let current = self.get(p_current, index_translator);
            if current == next {
                let merge_result = next * 2;
                self.set(p_current, merge_result, index_translator);
                self.set(p_next, 0, index_translator);
                reporter(GameAction::Merge {
                    from: p_next,
                    to: p_next,
                    merge_result,
                });
                p_current += 1;
            } else {
                if current == 0 {
                    p_current += 1;
                }

                if p_current == p_next {
                    self.set(p_current, next, index_translator);
                    self.set(p_next, 0, index_translator);
                    reporter(GameAction::Move {
                        from: p_next,
                        to: p_current,
                    });
                }
            }
        }
    }

    fn get(&self, index: usize, index_translator: &impl Fn(usize) -> usize) -> i32 {
        self.storage[index_translator(index)]
    }

    fn set(&mut self, index: usize, value: i32, index_translator: &impl Fn(usize) -> usize) {
        self.storage[index_translator(index)] = value;
    }
}

fn get_linear_function<T>(a: T, b: T, c: T) -> impl Fn(T, T) -> T
where
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    move |x, y| (a * x) + (b * y) + c
}
