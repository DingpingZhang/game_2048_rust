mod game_2048_matrix;

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{Clear, ClearType},
};
use game_2048_matrix::{Game2048Matrix, MoveOrientation};
use rand::prelude::*;
use std::io::stdout;

const MATRIX_ORDER: usize = 4;

fn main() {
    let mut rng = rand::thread_rng();
    let mut matrix = create_matrix(MATRIX_ORDER, &mut rng);
    let mut score = 0_u32;
    let mut max_number = 0_u32;
    let mut backup: Game2048Matrix;

    loop {
        print_header_info(score, max_number);
        print_matrix(&matrix);

        backup = matrix.clone();

        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
            }) => {
                matrix.move_to(MoveOrientation::Left);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
            }) => {
                matrix.move_to(MoveOrientation::Up);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
            }) => {
                matrix.move_to(MoveOrientation::Right);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
            }) => {
                matrix.move_to(MoveOrientation::Down);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(' '),
                modifiers: KeyModifiers::NONE,
            }) => {
                matrix = create_matrix(MATRIX_ORDER, &mut rng);
                backup = matrix.clone();
            }
            _ => (),
        }

        if matrix != backup {
            fill_matrix_once(&mut matrix, &mut rng);
        }

        execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    }
}

fn create_matrix(matrix_order: usize, rng: &mut ThreadRng) -> Game2048Matrix {
    let mut matrix = Game2048Matrix::new(matrix_order);
    fill_matrix_once(&mut matrix, rng);
    fill_matrix_once(&mut matrix, rng);
    matrix
}

fn fill_matrix_once(matrix: &mut Game2048Matrix, rng: &mut ThreadRng) {
    let index = get_exclusive_index(matrix, rng);
    matrix[index] = if rng.gen() { 2 } else { 4 };
}

fn get_exclusive_index(matrix: &Game2048Matrix, rng: &mut ThreadRng) -> (usize, usize) {
    let mut result;
    let length = matrix.get_matrix_order();

    loop {
        result = (rng.gen_range(0..length), rng.gen_range(0..length));

        if matrix[result] == 0 {
            break;
        }
    }

    result
}

fn print_header_info(score: u32, max_number: u32) {
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃ Score: {:-8} Max Number: {:-6} ┃", score, max_number);
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛");
    println!();
}

fn print_matrix(matrix: &Game2048Matrix) {
    let order = matrix.get_matrix_order();
    for i in 0..order {
        for j in 0..order {
            let value = matrix[(i, j)];
            let value_string = value.to_string();
            let value_str = value_string.as_str();
            print!("{:-5} ", if value == 0 { "." } else { value_str });
        }

        println!();
        println!();
        println!();
    }
}
