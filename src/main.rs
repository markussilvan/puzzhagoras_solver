mod piece;
mod puzzle;
mod solver;

use puzzle::{Dimensions, Puzzle, PuzzleBuilder};
use solver::Solver;

use crate::solver::PuzzleState;

fn main() {
    let dimensions = Dimensions::new(3, 3);

    println!(
        "Starting with width {} and height {}...",
        dimensions.width, dimensions.height
    );

    let mut puzzle = PuzzleBuilder::new()
        .with_dimensions(dimensions)
        .with_pieces_from_file("green-pieces.json".to_string())
        //.with_pieces_from_file("yellow-pieces.json".to_string())
        .build();

    let mut solver = Solver::new(&mut puzzle);

    let mut i = 0;
    let mut state = PuzzleState::Progressing;
    while state == PuzzleState::Progressing {
        i += 1;
        println!("Step {i}");
        state = solver.step();
    }

    //puzzle.write_pieces_to_file("pieces.json".to_string());

    println!("Board:");
    println!("{}", puzzle);
    println!("Final state: {state:?}");
}
