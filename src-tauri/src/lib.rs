
// a u8 where each lit up bit represents a point on a tile
//  starting from the least significant bit on the bottom edge left point continuing anti-clockwise
type TileConnection = u8;
// an array containing the binary representation for each connection on a tile
type TileConnections = [TileConnection; 4];

// a tuple of numerically indexed points forming a connection on a tile,
//  starting at 0, representing the bottom edge left point continuing anti-clockwise
type UnwrappedTileConnection = (u8, u8);
// an array containing the numeric representation for each connection on a tile
type UnwrappedTileConnections = [UnwrappedTileConnection; 4];

struct Tile {
    connections: TileConnections
}

// turns a connections binary representation into a tuple of its points numeric position
fn serialize_connection(connection: TileConnection) -> UnwrappedTileConnection {
    let lowest = connection.trailing_zeros() as u8;
    let highest = 7 - (connection.leading_zeros() as u8);
    (lowest, highest)
}

fn serialize_connections(connections: TileConnections) -> UnwrappedTileConnections {
    connections.map(|connection| serialize_connection(connection))
}

fn deserialize_connection(connection: UnwrappedTileConnection) -> TileConnection {
    let (lowest, highest) = connection;
    let mut result = 0;
    for i in lowest..highest + 1 {
        result |= 1 << i;
    }
    result
}

fn deserialize_connections(connections: UnwrappedTileConnections) -> TileConnections {
    connections.map(|connection| deserialize_connection(connection))
}

fn parse_standard_notation(notation: &str) -> UnwrappedTileConnections {
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
fn rotate_points(points: TileConnection) -> TileConnection {
    points << 2 | points >> 6
}

// rotate the points on a tile by 90 degrees
fn rotate_connections(connections: TileConnections) -> TileConnections {
    connections.map(|connection| rotate_points(connection))
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
fn flip_points(points: TileConnection) -> TileConnection {
    // AB-CD-EF-GH -> FE-HG-BA-DC
    let rotated = rotate_points(rotate_points(points));
    let odd = rotated & 0b1010_1010;
    let even = rotated & 0b0101_0101;
    odd >> 1 | even << 1
}

// flips points so that they would match points in the same position on an adjacent tile
fn flip_connections(connections: TileConnections) -> TileConnections {
    connections.map(|connection| flip_points(connection))
}

#[tauri::command]
fn test() -> UnwrappedTileConnections {
    let tile = Tile {
        // [0b0000_0011,0b0010_0100,0b1000_1000,0b0101_0000]
        connections: deserialize_connections([(0, 1), (2, 5), (3, 7), (4, 6)])
    };

    serialize_connections(tile.connections)
}

#[tauri::command]
fn get_possible_connections() -> Vec<UnwrappedTileConnections> {
    let possible_connections = [
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
        .map(|it| deserialize_connections(it));

    possible_connections.map(|it| serialize_connections(it)).to_vec()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![test, get_possible_connections])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
