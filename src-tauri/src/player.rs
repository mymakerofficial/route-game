use rand::thread_rng;
use crate::game_state::GameBoard;
use crate::lib_player::update_player_position;
use crate::lib_tile;
use crate::tile::{Tile, UnwrappedTile};
use lib_tile::{serialize_position_on_tile, shuffle_tiles};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Default, Clone)]
pub struct Player {
    pub(crate) position_on_board: usize,
    pub(crate) tile_position_mask: u8,
    tile_stack: Vec<Tile>,
    pub(crate) is_dead: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnwrappedPlayer {
    position_on_board: usize,
    position_on_tile: u8,
    tile_stack: Vec<UnwrappedTile>,
    is_dead: bool,
}

impl Serialize for Player {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.unwrap().serialize(serializer)
    }
}

impl Player {
    pub(crate) fn unwrap(&self) -> UnwrappedPlayer {
        UnwrappedPlayer {
            position_on_board: self.position_on_board,
            position_on_tile: serialize_position_on_tile(self.tile_position_mask),
            tile_stack: self.tile_stack.iter().map(|tile| tile.unwrap()).collect(),
            is_dead: self.is_dead,
        }
    }

    pub(crate) fn draw_from(&mut self, from_stack: &mut Vec<Tile>) {
        let tile = match from_stack.pop() {
            Some(tile) => tile,
            None => return
        };
        self.tile_stack.push(tile);
    }

    pub(crate) fn set_position(&mut self, position_on_board: usize, tile_position_mask: u8) {
        self.position_on_board = position_on_board;
        self.tile_position_mask = tile_position_mask;
    }

    // check if we are about to die
    pub(crate) fn check_place_tile_game_over(&mut self, board: &mut GameBoard, tile: Tile) -> bool {
        let mut player_clone = self.clone();
        let mut board_clone = board.clone();
        let tile_clone = tile.clone();
        board_clone[player_clone.position_on_board] = tile_clone;

        update_player_position(&mut player_clone, &board_clone, &mut vec![] /* fake global tile stack */);

        player_clone.is_dead
    }

    pub(crate) fn check_has_possible_moves(&mut self, board: &mut GameBoard) -> bool {
        // check if we can place any tile on the board
        for tile in &self.clone().tile_stack {
            let mut tile_clone = tile.clone();
            // rotate the tile to check all possible orientations
            for _ in 0..4 {
                // check if placing this tile would not cause game over
                if !self.check_place_tile_game_over(board, tile_clone) {
                    return true;
                }

                tile_clone.rotate();
            }
        }

        // no possible moves left
        false
    }

    pub(crate) fn place_tile(&mut self, board: &mut GameBoard, tile_index: usize) -> Result<(), String> {
        match board.get(self.position_on_board) {
            Some(tile) => {
                if !tile.is_empty() {
                    // we are trying to place a tile on a non-empty tile
                   return Err("Failed to place tile. Position already occupied.".to_string());
                }
            }
            // we are out of bounds
            None => return Ok(())
        }

        if !self.check_has_possible_moves(board) {
            return Err("Failed to place tile. Player has no possible remaining moves.".to_string());
        }

        if self.check_place_tile_game_over(board, self.tile_stack[tile_index]) {
            // we just caused our own death, which is not allowed
            return Err("Failed to place tile. Placing tile would result in game over.".to_string());
        }

        let tile = self.tile_stack.remove(tile_index);
        board[self.position_on_board] = tile;

        // tile successfully placed
        Ok(())
    }

    pub(crate) fn set_dead(&mut self, to_tile_stack: &mut Vec<Tile>) {
        self.is_dead = true;
        shuffle_tiles(&mut self.tile_stack);
        to_tile_stack.append(&mut self.tile_stack);
        self.tile_stack.drain(..);
    }

    pub(crate) fn get_tile(&mut self, tile_index: usize) -> &mut Tile {
        &mut self.tile_stack[tile_index]
    }
}