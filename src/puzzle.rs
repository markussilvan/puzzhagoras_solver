use serde::{Deserialize, Serialize};

use super::piece::{Connector, ConnectorGender, ConnectorOffset, ConnectorType, Piece};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

impl Dimensions {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

#[derive(Clone, Debug)]
pub struct Square {
    empty: bool,
    piece_id: usize,
}

impl Square {
    fn empty() -> Self {
        Self {
            empty: true,
            piece_id: 0,
        }
    }
}

#[derive(Debug)]
pub struct Board {
    squares: Vec<Square>,
    pub dimensions: Dimensions,
}

impl Board {
    pub fn new(dimensions: Dimensions) -> Self {
        let size = dimensions.width * dimensions.height;
        Self {
            squares: vec![Square::empty(); size],
            dimensions,
        }
    }

    pub fn add_piece(&mut self, position: usize, piece_id: usize) {
        self.squares[position].piece_id = piece_id;
        self.squares[position].empty = false;
    }

    pub fn remove_piece(&mut self, position: usize, piece_id: usize) {
        self.squares[position].piece_id = 0;
        self.squares[position].empty = true;
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.dimensions.height {
            for x in 0..self.dimensions.width {
                let position = y * self.dimensions.width + x;
                write!(f, "| {:>2} |", self.squares[position].piece_id)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct Puzzle {
    pub pieces: Vec<Piece>,
    pub board: Board,
}

impl Puzzle {
    pub fn new(dimensions: Dimensions) -> Self {
        Self {
            pieces: Vec::new(),
            board: Board::new(dimensions),
        }
    }

    pub fn get_edge_before(&self, position: usize) -> Connector {
        let remainder = position % self.board.dimensions.width;
        if remainder == 0 {
            // is a beginning of a row
            return Connector::flat();
        }
        //if remainder == self.dimensions.width - 1 {
        //    // is the end of a row
        //    return Connector::flat();
        //}

        let right_facing_connector_index = 2;

        let piece_id = self.board.squares[position - 1].piece_id;
        let connector = self.pieces[piece_id].get_connector(right_facing_connector_index);

        connector
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board);
        //TODO: also print a list of available pieces
        Ok(())
    }
}

pub struct PuzzleBuilder {
    pieces: Option<Vec<Piece>>,
    dimensions: Option<Dimensions>,
}

impl PuzzleBuilder {
    pub fn new() -> Self {
        Self {
            pieces: None,
            dimensions: None,
        }
    }

    pub fn with_dimensions(&mut self, dimensions: Dimensions) -> &mut Self {
        self.dimensions = Some(dimensions);
        self
    }

    pub fn with_yellow_corner_pieces(&mut self) -> &mut Self {
        let mut pieces = Vec::new();

        let flat = Connector::flat();

        let female_small_left = Connector::new(
            ConnectorGender::Female,
            ConnectorType::Small,
            ConnectorOffset::Left,
        );
        let male_large_right = Connector::new(
            ConnectorGender::Male,
            ConnectorType::Large,
            ConnectorOffset::Right,
        );
        let connectors = [
            flat.clone(),
            flat.clone(),
            female_small_left.clone(),
            male_large_right.clone(),
        ];
        let piece = Piece::new(connectors);
        pieces.push(piece);

        let male_small_right = Connector::new(
            ConnectorGender::Male,
            ConnectorType::Small,
            ConnectorOffset::Right,
        );
        let connectors = [
            flat.clone(),
            flat.clone(),
            female_small_left.clone(),
            male_small_right.clone(),
        ];
        let piece = Piece::new(connectors);
        pieces.push(piece);

        let female_large_left = Connector::new(
            ConnectorGender::Female,
            ConnectorType::Large,
            ConnectorOffset::Left,
        );
        let connectors = [
            flat.clone(),
            flat.clone(),
            female_large_left.clone(),
            male_small_right.clone(),
        ];
        let piece = Piece::new(connectors);
        pieces.push(piece);

        let connectors = [
            flat.clone(),
            flat.clone(),
            male_small_right.clone(),
            female_small_left.clone(),
        ];
        let piece = Piece::new(connectors);
        pieces.push(piece);

        self.pieces = Some(pieces);

        self
    }

    pub fn build(&mut self) -> Puzzle {
        let dimensions = self.dimensions.as_ref().unwrap().clone();
        let mut puzzle = Puzzle::new(dimensions);
        puzzle.pieces = self.pieces.as_ref().unwrap().clone();
        puzzle
    }
}
