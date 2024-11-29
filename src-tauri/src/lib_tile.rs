use rand::{thread_rng, Rng};
use rand::prelude::SliceRandom;
use crate::tile::{Tile, TileConnection, TileConnections, UnwrappedTileConnection, UnwrappedTileConnections};

// turns a connections binary representation into a tuple of its points numeric position
fn serialize_connection(connection: TileConnection) -> UnwrappedTileConnection {
    if connection == 0 {
        return (0, 0);
    }
    let lowest = connection.leading_zeros() as u8;
    let highest = 7 - (connection.trailing_zeros() as u8);
    (lowest, highest)
}

pub fn serialize_connections(connections: TileConnections) -> UnwrappedTileConnections {
    connections.map(|connection| serialize_connection(connection))
}

pub fn serialize_position_on_tile(tile_position_mask: u8) -> u8 {
    tile_position_mask.leading_zeros() as u8
}

fn deserialize_connection(connection: UnwrappedTileConnection) -> TileConnection {
    let (low, high) = connection;
    // set the bits at the low and high positions to 1
    let mut res = 0;
    res |= 1 << low;
    res |= 1 << high;
    res
}

pub fn deserialize_connections(connections: UnwrappedTileConnections) -> TileConnections {
    connections.map(|connection| deserialize_connection(connection))
}

pub fn deserialize_position_on_tile(tile_position: u8) -> u8 {
    1 << (7 - tile_position)
}

pub fn parse_standard_notation(notation: &str) -> UnwrappedTileConnections {
    let mut connections = [(0, 0); 4];
    let mut i = 0;
    for connection in notation.split('-') {
        let (from, to) = connection.split_at(1);
        connections[i] = (from.parse::<u8>().unwrap() - 1, to.parse::<u8>().unwrap() - 1);
        i += 1;
    }
    connections
}

// rotate the points on a tile by 90 degrees
fn rotate_points_90(points: TileConnection) -> TileConnection {
    points << 2 | points >> 6
}

// rotate the points on a tile by 90 degrees
pub fn rotate_connections_90(connections: TileConnections) -> TileConnections {
    connections.map(|connection| rotate_points_90(connection))
}

// rotate the points on a tile by 180 degrees
fn rotate_points_180(points: TileConnection) -> TileConnection {
    points << 4 | points >> 4
}

// rotate the points on a tile by 180 degrees
fn rotate_connections_180(connections: TileConnections) -> TileConnections {
    connections.map(|connection| rotate_points_180(connection))
}

// mirrors points on a diagonal axis based on their position on a tile
fn mirror_points(points: TileConnection) -> TileConnection {
    // AB-CD-EF-GH -> EF-GH-AB-CD
    let bottom = points & 0b0000_0011;
    let right = points & 0b0000_1100;
    let top = points & 0b0011_0000;
    let left = points & 0b1100_0000;
    bottom << 6 | right << 2 | top >> 2 | left >> 6
}

// mirrors points on a diagonal axis based on their position on a tile
fn mirror_connections(connections: TileConnections) -> TileConnections {
    connections.map(|connection| mirror_points(connection))
}

// flips points so that they would match points in the same position on an adjacent tile
pub fn flip_points(points: TileConnection) -> TileConnection {
    // AB-CD-EF-GH -> FE-HG-BA-DC
    let rotated = rotate_points_180(points);
    let odd = rotated & 0b1010_1010;
    let even = rotated & 0b0101_0101;
    odd >> 1 | even << 1
}

// flips points so that they would match points in the same position on an adjacent tile
fn flip_connections(connections: TileConnections) -> TileConnections {
    connections.map(|connection| flip_points(connection))
}

fn get_tile_set() -> Vec<Tile> {
    [
        "12-34-56-78",
        "14-27-36-58",
        "15-26-37-48",
        "16-25-38-47",
        "18-23-45-67",
        "12-37-48-56",
        "12-38-47-56",
        "16-25-37-48",
        "17-24-35-68",
        "15-27-36-48",
        "17-28-35-46",
        "18-26-37-45",
        "18-27-36-45",
        "13-26-48-57",
        "15-28-37-46",
        "12-35-47-68",
        "12-36-47-58",
        "12-38-45-67",
        "12-38-46-57",
        "17-24-36-58",
        "18-23-46-57",
        "12-34-57-68",
        "12-34-58-67",
        "16-23-47-58",
        "16-28-35-47",
        "17-23-46-58",
        "17-28-36-45",
        "12-36-48-57",
        "12-37-46-58",
        "12-37-45-68",
        "12-37-45-68",
        "12-37-45-68",
        "15-28-36-47",
        "13-25-48-67",
        "16-28-37-45",
    ]
        .map(|it| parse_standard_notation(it))
        .map(|it| deserialize_connections(it))
        .map(|connections| Tile { connections })
        .to_vec()
}

pub fn shuffle_tiles(tiles: &mut Vec<Tile>) {
    tiles.shuffle(&mut thread_rng());
    tiles.iter_mut().for_each(|tile| {
        for _ in 0..thread_rng().gen_range(0, 4) {
            tile.rotate();
        }
    });
}

pub fn generate_tile_stack() -> Vec<Tile> {
    let mut tiles = get_tile_set();
    shuffle_tiles(&mut tiles);

    tiles
}