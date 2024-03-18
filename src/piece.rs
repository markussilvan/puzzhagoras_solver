use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConnectorGender {
    Male,
    Female,
    Flat,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConnectorType {
    Small,
    Large,
    Flat,
}

/// Offset of the connector when looked from the middle of the piece towards the edge.
/// The connector is always a bit of left of center or right of center.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConnectorOffset {
    Left,
    Right,
    Flat,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Connector {
    gender: ConnectorGender,
    ctype: ConnectorType,
    offset: ConnectorOffset,
}

impl Connector {
    pub fn flat() -> Connector {
        Connector {
            gender: ConnectorGender::Flat,
            ctype: ConnectorType::Flat,
            offset: ConnectorOffset::Flat,
        }
    }

    pub fn fits(&self, other: &Connector) -> bool {
        #[allow(clippy::needless_bool)]
        #[allow(clippy::if_same_then_else)]
        if self.gender == ConnectorGender::Flat && other.gender == ConnectorGender::Flat {
            true
        } else if (self.gender != other.gender)
            && (self.ctype == other.ctype)
            && (self.offset != other.offset)
        {
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Green,
    Yellow,
}

/// A single puzzle piece with four edges.
///
/// A None connector means the edge is flat.
/// Also two flat edges are considered to fit together.
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Piece {
    connectors: [Connector; 4],
    color: Color,
    #[serde(skip)]
    pub used: bool,
    #[serde(skip)]
    pub flipped: bool,
    #[serde(skip)]
    pub rotations: usize,
}

impl Piece {
    pub fn rotate(&mut self) {
        self.connectors.rotate_right(1);
        self.rotations += 1;
    }

    pub fn flip(&mut self) {
        // swap connector offets
        for connector in self.connectors.iter_mut() {
            connector.offset = match connector.offset {
                ConnectorOffset::Left => ConnectorOffset::Right,
                ConnectorOffset::Right => ConnectorOffset::Left,
                ConnectorOffset::Flat => ConnectorOffset::Flat,
            };
        }

        // swap left and right side connectors
        self.connectors.swap(0, 2);

        // mark that piece is flipped (or returned to original orientation)
        self.flipped = !self.flipped;
    }

    pub fn get_connector(&self, index: usize) -> Connector {
        self.connectors[index]
    }

    pub fn fits(&self, connectors_around: &[Option<Connector>; 4]) -> bool {
        for (i, connector_opt) in connectors_around.iter().enumerate() {
            if let Some(conn) = connector_opt {
                if !self.get_connector(i).fits(conn) {
                    println!("Piece doesn't fit on {i} side");
                    return false;
                }
            }
        }

        true
    }
}
