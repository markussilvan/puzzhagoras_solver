use std::{
    fs::File,
    io::{BufReader, BufWriter, Write},
};

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

    pub fn get_size(&self) -> usize {
        self.dimensions.width * self.dimensions.height
    }

    pub fn get_piece(&self, position: usize) -> (usize, bool) {
        (
            self.squares[position].piece_id,
            self.squares[position].empty,
        )
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

    pub fn get_connectors_around(&self, position: usize) -> [Option<Connector>; 4] {
        let mut connectors = [None; 4];

        let remainder = position % self.board.dimensions.width;

        //TODO: remove the repetition and put it in a loop/function/macro instead

        if remainder == 0 {
            // is in the beginning of a row
            connectors[0] = Some(Connector::flat());
        } else {
            // is not in the beginning of a row
            let (piece_id, empty) = self.board.get_piece(position - 1);
            if empty {
                connectors[0] = Some(Connector::flat());
            } else {
                connectors[0] = Some(self.pieces[piece_id].get_connector(2));
            }
        }

        if remainder == self.board.dimensions.width - 1 {
            // is the end of a row
            connectors[2] = Some(Connector::flat());
        } else {
            // is not in the end of a row
            let (piece_id, empty) = self.board.get_piece(position + 1);
            if empty {
                connectors[2] = Some(Connector::flat());
            } else {
                connectors[2] = Some(self.pieces[piece_id].get_connector(0));
            }
        }

        if position < self.board.dimensions.width {
            // is on the first row
            connectors[1] = Some(Connector::flat());
        } else {
            // is not on the first row
            let (piece_id, empty) = self.board.get_piece(position - self.board.dimensions.width);
            if empty {
                connectors[1] = Some(Connector::flat());
            } else {
                connectors[1] = Some(self.pieces[piece_id].get_connector(3));
            }
        }

        if position >= (self.board.dimensions.width * (self.board.dimensions.height - 1)) {
            // is on the last row
            connectors[3] = Some(Connector::flat());
        } else {
            // is not on the last row
            let (piece_id, empty) = self.board.get_piece(position + self.board.dimensions.width);
            if empty {
                connectors[3] = Some(Connector::flat());
            } else {
                connectors[3] = Some(self.pieces[piece_id].get_connector(1));
            }
        }

        connectors
    }

    pub fn read_pieces_from_file(&mut self, filename: String) -> std::io::Result<()> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        self.pieces = serde_json::from_reader(reader)?;
        Ok(())
    }

    pub fn write_pieces_to_file(&self, filename: String) -> std::io::Result<()> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &self.pieces)?;
        writer.flush()?;
        Ok(())
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
    filename: Option<String>,
}

impl PuzzleBuilder {
    pub fn new() -> Self {
        Self {
            pieces: None,
            dimensions: None,
            filename: None,
        }
    }

    pub fn with_dimensions(&mut self, dimensions: Dimensions) -> &mut Self {
        self.dimensions = Some(dimensions);
        self
    }

    pub fn with_pieces_from_file(&mut self, filename: String) -> &mut Self {
        self.filename = Some(filename);
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

        if self.pieces.is_some() {
            puzzle.pieces = self.pieces.as_ref().unwrap().clone();
        }

        if self.filename.is_some() {
            puzzle.read_pieces_from_file(self.filename.as_ref().unwrap().clone());
        }

        puzzle
    }
}
