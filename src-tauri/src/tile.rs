use serde::{Deserialize, Serialize, Serializer};
use lib_tile::{rotate_connections_90, serialize_connections};
use crate::lib_tile;

// a u8 where each lit up bit represents a point on a tile
//  starting from the least significant bit on the bottom edge left point continuing anti-clockwise
pub type TileConnection = u8;
// an array containing the binary representation for each connection on a tile
pub type TileConnections = [TileConnection; 4];
// a tuple of numerically indexed points forming a connection on a tile,
//  starting at 0, representing the bottom edge left point continuing anti-clockwise
pub type UnwrappedTileConnection = (u8, u8);
// an array containing the numeric representation for each connection on a tile
pub type UnwrappedTileConnections = [UnwrappedTileConnection; 4];

#[derive(Debug, Default, Copy, Clone)]
pub struct Tile {
    pub(crate) connections: TileConnections
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnwrappedTile {
    is_empty: bool,
    connections: UnwrappedTileConnections
}

impl Serialize for Tile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.unwrap().serialize(serializer)
    }
}

impl Tile {
    pub(crate) fn unwrap(&self) -> UnwrappedTile {
        UnwrappedTile {
            is_empty: self.is_empty(),
            connections: serialize_connections(self.connections)
        }
    }

    pub(crate) fn rotate(&mut self) {
        self.connections = rotate_connections_90(self.connections);
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.connections.iter().all(|&connection| connection == 0)
    }
}