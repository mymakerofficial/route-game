use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::Mutex;
use tauri::{Manager, State};
use game_state::GameState;
mod tile;
mod player;
mod game_state;
mod lib_tile;
mod lib_player;

#[tauri::command]
fn get_game_state(state: State<'_, Mutex<GameState>>) -> GameState {
    let state = state.lock().unwrap();
    state.clone()
}

#[tauri::command]
fn add_player(state: State<'_, Mutex<GameState>>, position_on_board: usize, position_on_tile: u8) {
    let mut state = state.lock().unwrap();
    state.add_player(position_on_board, position_on_tile);
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
            get_game_state,
            add_player,
            rotate_player_tile,
            place_player_tile,
            reset_game_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
