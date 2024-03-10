use crate::{
    piece::{Connector, Piece},
    puzzle::{Board, Puzzle},
};

pub struct Solver<'a> {
    puzzle: &'a mut Puzzle,
    position: usize,
}

impl<'a> Solver<'a> {
    pub fn new(puzzle: &'a mut Puzzle) -> Self {
        Self {
            puzzle,
            position: 0,
        }
    }

    pub fn step(&mut self) -> bool {
        println!("Stepping...");

        let size = self.puzzle.board.dimensions.width * self.puzzle.board.dimensions.height;
        println!("POSITION: {} SIZE: {}", self.position, size);
        if self.position >= size {
            println!("INVALID POSITION");
            return false;
        }

        for piece_id in 0..self.puzzle.pieces.len() {
            if self.puzzle.pieces[piece_id].used {
                println!("Piece is already on board, skip it");
                continue;
            }
            println!("Checking piece: {}", self.puzzle.pieces[piece_id]);
            if Self::add_if_fits(&mut self.puzzle, self.position, piece_id) {
                println!("Added piece {}", self.puzzle.pieces[piece_id]);
                self.position += 1;
                break;
            } else {
                println!("Doesn't fit");
            }
        }

        // no piece fits
        // is this the last square?
        if self.position == size {
            // TODO
            // dead end, back track to previous position where all options
            // haven't been exhausted, that is a square where there are still
            // untested pieces further in the list of pieces

            // if backtracking reaches the first square and still nothing fits
            // the puzzle is unsolvable
        }

        println!("State:\n{}", self.puzzle);
        false
    }

    fn add_if_fits(puzzle: &mut Puzzle, position: usize, piece_id: usize) -> bool {
        let mut fits = false;

        let before = puzzle.get_edge_before(position);
        println!("Connector before is {before:?}");
        let piece = &mut puzzle.pieces[piece_id];

        for _ in 0..4 {
            println!(
                "Checking if connector '{:?}' fits the previous piece (if any)",
                piece.get_connector(0)
            );
            if piece.get_connector(0).fits(&before) {
                println!("Piece fits");
                fits = true;
                break;
            }
            println!("Rotating piece");
            piece.rotate();
        }

        if fits {
            println!("Adding piece to the board (position: {position})");
            puzzle.board.add_piece(position, piece_id);
            puzzle.pieces[piece_id].used = true;
        }

        fits
    }
}
