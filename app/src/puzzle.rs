use serde::{Deserialize, Serialize};

use super::piece::{Color, Connector, Direction, OptionalConnectors, Piece};

#[derive(Deserialize, Serialize, PartialEq, Clone, Copy)]
pub enum PieceSet {
    Yellow,
    Green,
    Both,
}

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

    pub fn is_empty(&self) -> bool {
        self.empty
    }

    pub fn piece_id(&self) -> usize {
        self.piece_id
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

    pub fn get_piece_towards(&self, position: usize, direction: Direction) -> (usize, bool) {
        let pos = match direction {
            Direction::Up => position - self.dimensions.width,
            Direction::Down => position + self.dimensions.width,
            Direction::Left => position - 1,
            Direction::Right => position + 1,
        };
        self.get_piece(pos)
    }

    pub fn add_piece(&mut self, position: usize, piece_id: usize) {
        self.squares[position].piece_id = piece_id;
        self.squares[position].empty = false;
    }

    pub fn remove_piece(&mut self, position: usize) {
        self.squares[position].piece_id = 0;
        self.squares[position].empty = true;
    }

    pub fn get_squares(&self) -> Vec<Square> {
        self.squares.clone()
    }

    pub fn is_on_edge(&self, position: usize, direction: Direction) -> bool {
        match direction {
            Direction::Up => {
                if position < self.dimensions.width {
                    return true;
                }
            }
            Direction::Down => {
                if position >= (self.dimensions.width * (self.dimensions.height - 1)) {
                    return true;
                }
            }
            Direction::Left => {
                if position % self.dimensions.width == 0 {
                    return true;
                }
            }
            Direction::Right => {
                if position % self.dimensions.width == self.dimensions.width - 1 {
                    return true;
                }
            }
        }
        false
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.dimensions.height {
            for x in 0..self.dimensions.width {
                let position = y * self.dimensions.width + x;
                write!(f, "| {:>2} |", self.squares[position].piece_id)?;
            }
            writeln!(f)?;
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

    pub fn get_connectors_around(&self, position: usize) -> OptionalConnectors {
        let mut connectors = [None; Direction::count()];

        self.get_connector_in_direction(&mut connectors, position, Direction::Left);
        self.get_connector_in_direction(&mut connectors, position, Direction::Up);
        self.get_connector_in_direction(&mut connectors, position, Direction::Right);
        self.get_connector_in_direction(&mut connectors, position, Direction::Down);

        connectors
    }

    fn get_connector_in_direction(
        &self,
        connectors: &mut OptionalConnectors,
        position: usize,
        direction: Direction,
    ) {
        if self.board.is_on_edge(position, direction) {
            // the outside edges must always be flat
            connectors[direction as usize] = Some(Connector::flat());
        } else {
            let (piece_id, empty) = self.board.get_piece_towards(position, direction);
            if !empty {
                connectors[direction as usize] =
                    Some(self.pieces[piece_id].get_connector(direction.opposite() as usize));
            }
        }
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board)?;
        Ok(())
    }
}

pub struct PuzzleBuilder {
    dimensions: Option<Dimensions>,
    json: Option<&'static str>,
    piece_set: Option<PieceSet>,
}

impl PuzzleBuilder {
    pub fn new() -> Self {
        Self {
            dimensions: None,
            json: None,
            piece_set: None,
        }
    }

    pub fn with_dimensions(&mut self, dimensions: Dimensions) -> &mut Self {
        self.dimensions = Some(dimensions);
        self
    }

    pub fn with_piece_set(&mut self, piece_set: PieceSet) -> &mut Self {
        self.piece_set = Some(piece_set);
        self
    }

    pub fn with_pieces_from_json(&mut self, json_str: &'static str) -> &mut Self {
        self.json = Some(json_str);
        self
    }

    pub fn build(&mut self) -> Puzzle {
        let dimensions = self.dimensions.as_ref().unwrap().clone();
        let mut puzzle = Puzzle::new(dimensions);

        if self.json.is_some() {
            let Ok(mut pieces) = serde_json::from_str::<Vec<Piece>>(self.json.as_ref().unwrap())
            else {
                panic!("Invalid pieces JSON");
            };

            match self.piece_set.as_ref().unwrap() {
                PieceSet::Yellow => pieces.retain(|x| x.color == Color::Yellow),
                PieceSet::Green => pieces.retain(|x| x.color == Color::Green),
                PieceSet::Both => {}
            };

            puzzle.pieces = pieces;
        }

        puzzle
    }
}
