use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ConnectorGender {
    Male,
    Female,
    Flat,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ConnectorType {
    Small,
    Large,
    Flat,
}

/// Offset of the connector when looked from the middle of the piece towards the edge.
/// The connector is always a bit of left of center or right of center.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum ConnectorOffset {
    Left,
    Right,
    Flat,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Connector {
    gender: ConnectorGender,
    ctype: ConnectorType,
    offset: ConnectorOffset,
}

impl Connector {
    pub fn new(gender: ConnectorGender, ctype: ConnectorType, offset: ConnectorOffset) -> Self {
        Self {
            gender,
            ctype,
            offset,
        }
    }

    pub fn flat() -> Connector {
        Connector {
            gender: ConnectorGender::Flat,
            ctype: ConnectorType::Flat,
            offset: ConnectorOffset::Flat,
        }
    }

    pub fn fits(&self, other: &Connector) -> bool {
        if self.gender == ConnectorGender::Flat && other.gender == ConnectorGender::Flat {
            true
        } else if (self.gender == other.gender)
            || (self.ctype != other.ctype)
            || (self.offset == other.offset)
        {
            false
        } else {
            true
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Color {
    Green,
    Yellow,
}

/// A single puzzle piece with four edges.
///
/// A None connector means the edge is flat.
/// Also two flat edges are considered to fit together.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Piece {
    connectors: [Connector; 4],
    color: Color,
    pub used: bool,
}

impl Piece {
    pub fn new(connectors: [Connector; 4]) -> Self {
        Self {
            connectors,
            color: Color::Yellow,
            used: false,
        }
    }

    pub fn rotate(&mut self) {
        self.connectors.rotate_right(1);
    }

    pub fn flip(&mut self) {
        //TODO: implement flipping
    }

    pub fn get_connector(&self, index: usize) -> Connector {
        self.connectors[index].clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct OptionalPiece(pub Option<Piece>);

impl From<Piece> for OptionalPiece {
    fn from(value: Piece) -> Self {
        OptionalPiece(Some(value))
    }
}

impl std::fmt::Display for OptionalPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == None {
            write!(f, "-");
        } else {
            write!(f, "{}", self.0.as_ref().unwrap());
        }
        Ok(())
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alphabet: Vec<u8> = (b'a'..b'z').collect();
        let mut index = 0;

        for c in self.connectors.iter() {
            if c.offset == ConnectorOffset::Left {
                index = index + 1;
            } else if c.offset == ConnectorOffset::Right {
                index = index + 2;
            }
            if c.ctype == ConnectorType::Large {
                index = index + 3;
            } else if c.ctype == ConnectorType::Small {
                index = index + 5;
            }
            if c.gender == ConnectorGender::Male {
                index = index + 6;
            } else if c.gender == ConnectorGender::Female {
                index = index + 10;
            }
        }
        let mut character = format!("{}", alphabet[index % 25] as char);
        if self.color == Color::Green {
            character = character.to_uppercase();
        }
        write!(f, "{}", character)
    }
}
