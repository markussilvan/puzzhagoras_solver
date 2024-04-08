use tracing::{debug, info, trace};

use crate::{
    piece::Piece,
    puzzle::{Puzzle, Square},
};

#[derive(Debug, PartialEq, Eq)]
pub enum PuzzleState {
    Idle,
    Progressing,
    Backtrack,
    Solved,
    Unsolvable,
}

pub struct Solver {
    puzzle: Puzzle,
    position: usize,
}

impl Solver {
    pub fn new(puzzle: Puzzle) -> Self {
        Self {
            puzzle,
            position: 0,
        }
    }

    pub fn get_board_squares(&self) -> Vec<Square> {
        self.puzzle.board.get_squares()
    }

    pub fn get_piece(&self, piece_id: usize) -> Piece {
        self.puzzle.pieces[piece_id]
    }

    /// Make the next step in the puzzle, backtrack if there is no way forward.
    ///
    /// # Returns
    ///
    /// `true` if a step was made
    /// `false` if there is no play to make (puzzle is either solved or unsolvable)
    ///
    pub fn step(&mut self) -> PuzzleState {
        trace!("Stepping...");

        let start_piece_id = {
            let (mut piece_id, empty) = self.puzzle.board.get_piece(self.position);
            if !empty {
                // we are backtracking, take the old piece off the board
                trace!("Backtracking...");
                self.puzzle.board.remove_piece(self.position);
                self.puzzle.pieces[piece_id].used = false;
                piece_id += 1;
            }
            piece_id
        };

        let state = self.forward(start_piece_id);
        match state {
            PuzzleState::Progressing => {
                trace!("State:\n{}", self.puzzle);
            }
            PuzzleState::Backtrack => {
                if self.position == 0 {
                    // backtracking has reached back to the beginning
                    // all options have been exhausted
                    // puzzle is unsolvable
                    return PuzzleState::Unsolvable;
                }
                self.position -= 1;
            }
            _ => {
                // nothing special to do for the other states
            }
        }

        state
    }

    /// Try to make the next step in the puzzle.
    ///
    /// # Arguments
    ///
    /// `start_piece_id`  - piece to continue from, previous pieces are ignored
    ///
    /// # Returns
    ///
    /// `PuzzleState` of the puzzle after this step.
    ///
    fn forward(&mut self, start_piece_id: usize) -> PuzzleState {
        let added = self.check_pieces(start_piece_id);
        let size = self.puzzle.board.get_size();

        if added {
            if self.position >= size {
                info!("Puzzle solved!");
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
    /// If so, make the play.
    ///
    /// # Arguments
    ///
    /// `start_piece_id`  - piece to continue from, previous pieces are ignored
    ///
    /// # Returns
    ///
    /// `true` if a move was made
    /// `false` if no free piece fits the puzzle
    ///
    fn check_pieces(&mut self, start_piece_id: usize) -> bool {
        for piece_id in start_piece_id..self.puzzle.pieces.len() {
            if self.puzzle.pieces[piece_id].used {
                trace!("Piece is already on board, skip it");
                continue;
            }
            trace!("Checking piece: {}", piece_id);
            if self.add_if_fits(piece_id) {
                debug!("Added piece {} to position {}", piece_id, self.position);
                self.position += 1;
                return true;
            } else {
                trace!("Doesn't fit");
            }
        }

        false
    }

    /// Check if a single piece fits as the next piece in the puzzle.
    ///
    /// Add it to the puzzle if it fits.
    ///
    /// # Arguments
    ///
    /// `piece_id`  - piece to check if it fits
    ///
    /// # Returns
    ///
    /// `true`  if a piece was added to the puzzle
    /// `false` if the piece doesn't fit
    ///
    fn add_if_fits(&mut self, piece_id: usize) -> bool {
        let mut fits = false;

        let connectors_around = self.puzzle.get_connectors_around(self.position);
        let piece = &mut self.puzzle.pieces[piece_id];

        // try the piece in all it's rotations (including flipping it)
        for _ in 0..2 {
            for _ in 0..4 {
                if piece.fits(&connectors_around) {
                    trace!("Piece fits on all sides");
                    fits = true;
                    break;
                }
                trace!("Rotating the piece");
                piece.rotate();
            }

            if fits {
                break;
            }

            trace!("Flipping the piece");
            piece.flip();
        }

        if fits {
            trace!(
                "Adding the piece to the board (position: {})",
                self.position
            );
            self.puzzle.board.add_piece(self.position, piece_id);
            self.puzzle.pieces[piece_id].used = true;
        }

        fits
    }
}
