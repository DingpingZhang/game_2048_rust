mod game_2048_matrix;

use game_2048_matrix::{Game2048Matrix, MoveOrientation};

fn main() {
    let mut game = Game2048Matrix::new(4);
    game.move_to(MoveOrientation::Left);
}
