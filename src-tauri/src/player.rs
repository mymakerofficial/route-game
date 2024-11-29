use serde::{Deserialize, Serialize, Serializer};
use lib_tile::{serialize_position_on_tile, shuffle_tiles};
use crate::tile::{Tile, UnwrappedTile};
use crate::game_state::GameBoard;
use crate::lib_tile;

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

    pub(crate) fn place_tile(&mut self, board: &mut GameBoard, tile_index: usize) {
        match board.get(self.position_on_board) {
            Some(tile) => {
                if !tile.is_empty() {
                    // we are trying to place a tile on a non-empty tile
                    return;
                }
            }
            // we are out of bounds
            None => return
        }
        let tile = self.tile_stack.remove(tile_index);
        board[self.position_on_board] = tile;
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