
struct Tile {
    // each connection between points is a u8 with the bits representing the connections
    // the least significant bit is the left point on the bottom edge continuing anti-clockwise
    connections: [u8; 4]
}

// turns a connections binary representation into a tuple of its points numeric position
fn serialize_connection(connection: u8) -> (u8, u8) {
    let lowest = connection.trailing_zeros() as u8;
    let highest = 7 - (connection.leading_zeros() as u8);
    (lowest, highest)
}

fn serialize_connections(connections: [u8; 4]) -> [(u8, u8); 4] {
    connections.map(|connection| serialize_connection(connection))
}

fn deserialize_connection(connection: (u8, u8)) -> u8 {
    let (lowest, highest) = connection;
    let mut result = 0;
    for i in lowest..highest + 1 {
        result |= 1 << i;
    }
    result
}

fn deserialize_connections(connections: [(u8, u8); 4]) -> [u8; 4] {
    connections.map(|connection| deserialize_connection(connection))
}

// rotate the points on a tile by 90 degrees
fn rotate_points(points: u8) -> u8 {
    points << 2 | points >> 6
}

fn rotate_connections(connections: [u8; 4]) -> [u8; 4] {
    connections.map(|connection| rotate_points(connection))
}

// mirrors points on a diagonal axis based on their position on a tile
fn mirror_points(points: u8) -> u8 {
    // AB-CD-EF-GH -> EF-GH-AB-CD
    let bottom = points & 0b0000_0011;
    let right = points & 0b0000_1100;
    let top = points & 0b0011_0000;
    let left = points & 0b1100_0000;
    bottom << 6 | right << 2 | top >> 2 | left >> 6
}

fn mirror_connections(connections: [u8; 4]) -> [u8; 4] {
    connections.map(|connection| mirror_points(connection))
}

fn flip_points(points: u8) -> u8 {
    // AB-CD-EF-GH -> FE-HG-BA-DC
    let rotated = rotate_points(rotate_points(points));
    let odd = rotated & 0b1010_1010;
    let even = rotated & 0b0101_0101;
    odd >> 1 | even << 1
}

fn flip_connections(connections: [u8; 4]) -> [u8; 4] {
    connections.map(|connection| flip_points(connection))
}

#[tauri::command]
fn test() -> [(u8, u8); 4] {
    let tile = Tile {
        // [0b0000_0011,0b0010_0100,0b1000_1000,0b0101_0000]
        connections: deserialize_connections([(0, 1), (2, 5), (3, 7), (4, 6)])
    };

    serialize_connections(tile.connections)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
