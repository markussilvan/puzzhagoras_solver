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

    pub fn fits(&self, connectors_around: &[Option<Connector>; 4]) -> bool {
        for i in 0..4 {
            if let Some(conn) = connectors_around[i] {
                if !self.get_connector(i).fits(&conn) {
                    println!("Piece doesn't fit on {i} side");
                    return false;
                }
            }
        }

        true
    }
}
