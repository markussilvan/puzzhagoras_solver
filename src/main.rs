mod piece;
mod puzzle;
mod solver;

use puzzle::{Dimensions, Puzzle, PuzzleBuilder};
use solver::Solver;

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
    let mut solved = false;
    while !solved {
        i += 1;
        println!("Step {i}");
        solved = solver.step();
    }
}
