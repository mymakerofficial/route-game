
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

// rotate the connections of a tile by 90 degrees
fn rotate_connection(connection: u8) -> u8 {
    connection << 2 | connection >> 6
}

fn rotate_connections(connections: [u8; 4]) -> [u8; 4] {
    connections.map(|connection| rotate_connection(connection))
}

#[tauri::command]
fn greet() -> [(u8, u8); 4] {
    let tile = Tile {
        connections: rotate_connections([0b0000_0011,0b0000_1100,0b0011_0000,0b1100_0000])
    };

    serialize_connections(tile.connections)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
