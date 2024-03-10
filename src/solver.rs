use crate::{
    piece::{Connector, Piece},
    puzzle::{Board, Puzzle},
};

#[derive(Debug, PartialEq, Eq)]
pub enum PuzzleState {
    Progressing,
    Backtrack,
    Solved,
    Unsolvable,
}

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

    /// Make the next step in the puzzle, backtrack if there is no way forward.
    ///
    /// # Returns
    ///
    /// `true` if a step was made
    /// `false` if there is no play to make (puzzle is either solved or unsolvable)
    ///
    pub fn step(&mut self) -> PuzzleState {
        println!("Stepping...");

        let start_piece_id = {
            let mut piece_id = self.puzzle.board.get_piece(self.position);
            if piece_id != 0 {
                // we are backtracking, take the old piece off the board
                println!("Backtracking...");
                self.puzzle.board.remove_piece(self.position, piece_id);
                self.puzzle.pieces[piece_id].used = false;
                piece_id += 1;
            }
            piece_id
        };

        match self.forward(start_piece_id) {
            PuzzleState::Progressing => {
                println!("State:\n{}", self.puzzle);
                return PuzzleState::Progressing;
            }
            PuzzleState::Backtrack => {
                if self.position == 0 {
                    // backtracking has reached back to the beginning
                    // all options have been exhausted
                    // puzzle is unsolvable
                    return PuzzleState::Unsolvable;
                }
                self.position -= 1;
                return PuzzleState::Progressing;
            }
            PuzzleState::Solved => {
                return PuzzleState::Solved;
            }
            PuzzleState::Unsolvable => {
                return PuzzleState::Unsolvable;
            }
        }
    }

    /// Try to make the next step in the puzzle.
    pub fn forward(&mut self, start_piece_id: usize) -> PuzzleState {
        let added = self.check_pieces(start_piece_id);
        let size = self.puzzle.board.get_size();

        if added {
            if self.position >= size {
                println!("Puzzle solved!");
                //TODO: how to handle this properly?
                PuzzleState::Solved
            } else {
                PuzzleState::Progressing
            }
        } else {
            // no piece fits
            // backtrack
            PuzzleState::Backtrack
        }
    }

    /// Check, in order, if any free piece fits as the next piece in the puzzle.
    /// If so, make the play
    ///
    /// # Returns
    ///
    /// `true` if a move was made
    /// `false` if no free piece fits the puzzle
    ///
    fn check_pieces(&mut self, start_piece_id: usize) -> bool {
        for piece_id in start_piece_id..self.puzzle.pieces.len() {
            if self.puzzle.pieces[piece_id].used {
                println!("Piece is already on board, skip it");
                continue;
            }
            println!("Checking piece: {}", piece_id);
            if Self::add_if_fits(&mut self.puzzle, self.position, piece_id) {
                println!("Added piece {}", piece_id);
                self.position += 1;
                return true;
            } else {
                println!("Doesn't fit");
            }
        }

        false
    }

    /// Check if a single piece fits as the next piece in the puzzle.
    ///
    /// Add it to the puzzle if it fits.
    ///
    /// # Returns
    ///
    /// `true`  if a piece was added to the puzzle
    /// `false` if the piece doesn't fit
    ///
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
