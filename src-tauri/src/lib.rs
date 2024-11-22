use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::Mutex;
use tauri::{Manager, State};

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

// a 6x6 array of tiles representing the game board
type GameBoard = [Tile; 6*6];

#[derive(Debug, Default, Copy, Clone)]
struct Tile {
    connections: TileConnections
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UnwrappedTile {
    is_empty: bool,
    connections: UnwrappedTileConnections
}

#[derive(Debug, Default, Clone)]
struct Player {
    position_on_board: usize,
    tile_position_mask: u8,
    tile_stack: Vec<Tile>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UnwrappedPlayer {
    position_on_board: usize,
    position_on_tile: u8,
    tile_stack: Vec<UnwrappedTile>,
}

struct GameState {
    board: GameBoard,
    tile_stack: Vec<Tile>,
    players: Vec<Player>,
}

// turns a connections binary representation into a tuple of its points numeric position
fn serialize_connection(connection: TileConnection) -> UnwrappedTileConnection {
    if connection == 0 {
        return (0, 0);
    }
    let lowest = connection.leading_zeros() as u8;
    let highest = 7 - (connection.trailing_zeros() as u8);
    (lowest, highest)
}

fn serialize_connections(connections: TileConnections) -> UnwrappedTileConnections {
    connections.map(|connection| serialize_connection(connection))
}

fn serialize_position_on_tile(tile_position_mask: u8) -> u8 {
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

fn deserialize_connections(connections: UnwrappedTileConnections) -> TileConnections {
    connections.map(|connection| deserialize_connection(connection))
}

fn deserialize_position_on_tile(tile_position: u8) -> u8 {
    1 << (7 - tile_position)
}

impl Serialize for Tile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let unwrapped_connections = serialize_connections(self.connections);
        let unwrapped_tile = UnwrappedTile {
            is_empty: self.connections.iter().all(|&connection| connection == 0),
            connections: unwrapped_connections
        };
        unwrapped_tile.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Tile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let unwrapped_connections = UnwrappedTileConnections::deserialize(deserializer)?;
        let connections = deserialize_connections(unwrapped_connections);
        Ok(Tile { connections })
    }
}

impl Serialize for Player {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let unwrapped_player = UnwrappedPlayer {
            position_on_board: self.position_on_board,
            position_on_tile: serialize_position_on_tile(self.tile_position_mask),
            tile_stack: self.tile_stack.clone().iter().map(|tile| {
                let unwrapped_connections = serialize_connections(tile.connections);
                UnwrappedTile {
                    is_empty: tile.connections.iter().all(|&connection| connection == 0),
                    connections: unwrapped_connections
                }
            }).collect()
        };
        unwrapped_player.serialize(serializer)
    }
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
fn rotate_points_90(points: TileConnection) -> TileConnection {
    points << 2 | points >> 6
}

// rotate the points on a tile by 90 degrees
fn rotate_connections_90(connections: TileConnections) -> TileConnections {
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
fn flip_points(points: TileConnection) -> TileConnection {
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

// finds the connection that goes from the specified position on the tile
//  and returns the position on the tile when the connection is walked across
fn walk_connection(connections: TileConnections, source_position_mask: u8) -> u8 {
    println!("walking source: {} {:#010b}", serialize_position_on_tile(source_position_mask), source_position_mask);
    // find the connection that has a bit high in the position given
    for connection in connections.iter() {
        if connection & source_position_mask != 0 {
            // we found the connection,
            //  the other high bit is the position on the tile we end up in
            let destination_mask = connection ^ source_position_mask;
            println!(
                "walking connections: {:?} {:#010b} {:?} {:#010b} {:?} {:#010b} {:?} {:#010b}",
                serialize_connection(connections[0]),
                connections[0],
                serialize_connection(connections[1]),
                connections[1],
                serialize_connection(connections[2]),
                connections[2],
                serialize_connection(connections[3]),
                connections[3]
            );
            return destination_mask;
        }
    }
    panic!("no connection found");
}

fn transition_tile(tile_position_mask: u8) -> isize {
    // todo this can probably be optimized
    match tile_position_mask {
        0b0000_0001 => -1,
        0b0000_0010 => -1,
        0b0000_0100 => -6,
        0b0000_1000 => -6,
        0b0001_0000 => 1,
        0b0010_0000 => 1,
        0b0100_0000 => 6,
        0b1000_0000 => 6,
        _ => panic!("invalid tile position mask")
    }
}

impl Tile {
    fn rotate(&mut self) {
        self.connections = rotate_connections_90(self.connections);
    }

    fn mirror(&mut self) {
        self.connections = mirror_connections(self.connections);
    }

    fn flip(&mut self) {
        self.connections = flip_connections(self.connections);
    }
}

impl GameState {
    fn reset(&mut self) {
        self.tile_stack = generate_tile_stack();
        self.board = [Tile::default(); 36];
        self.players = vec![];
    }

    fn get_player(&mut self, player_index: usize) -> &mut Player {
        &mut self.players[player_index]
    }

    fn place_tile(&mut self, player_index: usize, tile_index: usize) {
        let player = &mut self.players[player_index];
        let board = &mut self.board;
        player.place_tile(board, tile_index);

        let tile_stack = &mut self.tile_stack;
        player.draw_from(tile_stack);

        self.update_positions();
    }

    fn update_positions(&mut self) {
        for player in self.players.iter_mut() {
            // get the tile the player is standing on
            let tile = self.board[player.position_on_board];

            let new_position = walk_connection(tile.connections, player.tile_position_mask);

            println!("walked: {} {:#010b}", serialize_position_on_tile(new_position), new_position);

            let transition = transition_tile(new_position);

            println!("moved: {} {}", player.position_on_board, transition);

            let new_position = flip_points(new_position);

            println!("flipped: {} {:#010b}", serialize_position_on_tile(new_position), new_position);

            player.tile_position_mask = new_position;

            player.position_on_board = (player.position_on_board as isize + transition) as usize;
        }
    }
}

impl Player {
    fn draw_from(&mut self, from_stack: &mut Vec<Tile>) {
        let tile = from_stack.pop().unwrap();
        self.tile_stack.push(tile);
    }

    fn set_position(&mut self, position_on_board: usize, tile_position_mask: u8) {
        self.position_on_board = position_on_board;
        self.tile_position_mask = tile_position_mask;
    }

    fn place_tile(&mut self, board: &mut GameBoard, tile_index: usize) {
        let tile = self.tile_stack.remove(tile_index);
        board[self.position_on_board] = tile;
    }

    fn get_tile(&mut self, tile_index: usize) -> &mut Tile {
        &mut self.tile_stack[tile_index]
    }
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

fn shuffle_tiles(tiles: &mut Vec<Tile>) {
    tiles.shuffle(&mut thread_rng());
    tiles.iter_mut().for_each(|tile| {
        for _ in 0..thread_rng().gen_range(0, 4) {
            tile.rotate();
        }
    });
}

fn generate_tile_stack() -> Vec<Tile> {
    let mut tiles = get_tile_set();
    shuffle_tiles(&mut tiles);

    tiles
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            tile_stack: generate_tile_stack(),
            board: [Tile::default(); 36],
            players: vec![],
        }
    }
}

#[tauri::command]
fn add_player(state: State<'_, Mutex<GameState>>, position_on_board: usize, position_on_tile: u8) {
    let mut state = state.lock().unwrap();

    let mut new_player = Player::default();

    println!("added player: {} {:#010b}", position_on_tile, deserialize_position_on_tile(position_on_tile));

    new_player.set_position(position_on_board, deserialize_position_on_tile(position_on_tile));

    new_player.draw_from(&mut state.tile_stack);
    new_player.draw_from(&mut state.tile_stack);
    new_player.draw_from(&mut state.tile_stack);

    state.players.push(new_player);
}

#[tauri::command]
fn rotate_player_tile(state: State<'_, Mutex<GameState>>, player_index: usize, tile_index: usize) {
    let mut state = state.lock().unwrap();
    state.get_player(player_index).get_tile(tile_index).rotate();
}

#[tauri::command]
fn place_player_tile(state: State<'_, Mutex<GameState>>, player_index: usize, tile_index: usize) {
    let mut state = state.lock().unwrap();
    state.place_tile(player_index, tile_index);
}

#[tauri::command]
fn get_tile_stack(state: State<'_, Mutex<GameState>>) -> Vec<Tile> {
    let state = state.lock().unwrap();
    state.tile_stack.clone()
}

#[tauri::command]
fn get_players(state: State<'_, Mutex<GameState>>) -> Vec<Player> {
    let state = state.lock().unwrap();
    state.players.clone()
}

#[tauri::command]
fn get_game_board(state: State<'_, Mutex<GameState>>) -> Vec<Tile> {
    let state = state.lock().unwrap();
    state.board.clone().to_vec()
}

#[tauri::command]
fn reset_game_state(state: State<'_, Mutex<GameState>>) {
    let mut state = state.lock().unwrap();
    state.reset();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            app.manage(Mutex::new(GameState::default()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_tile_stack,
            get_players,
            get_game_board,
            add_player,
            rotate_player_tile,
            place_player_tile,
            reset_game_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
