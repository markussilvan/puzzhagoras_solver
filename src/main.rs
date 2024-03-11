mod piece;
mod puzzle;
mod solver;

use puzzle::{Dimensions, Puzzle, PuzzleBuilder};
use solver::Solver;

use crate::solver::PuzzleState;

fn main() {
    let dimensions = Dimensions::new(2, 2);

    println!(
        "Starting with width {} and height {}...",
        dimensions.width, dimensions.height
    );

    let mut puzzle = PuzzleBuilder::new()
        .with_dimensions(dimensions)
        .with_yellow_corner_pieces()
        .build();

    let mut solver = Solver::new(&mut puzzle);

    let mut i = 0;
    let mut state = PuzzleState::Progressing;
    while state == PuzzleState::Progressing {
        i += 1;
        println!("Step {i}");
        state = solver.step();
    }

    println!("Final state: {state:?}");
}
