use serde::{Serialize, Serializer};
use lib_player::update_player_position;
use lib_tile::{deserialize_position_on_tile, generate_tile_stack};
use crate::{lib_player, lib_tile};
use crate::player::{Player, UnwrappedPlayer};
use crate::tile::{Tile, UnwrappedTile};

// a 6x6 array of tiles representing the game board
pub type GameBoard = [Tile; 6*6];

#[derive(Debug, Clone)]
pub struct GameState {
    board: GameBoard,
    tile_stack: Vec<Tile>,
    players: Vec<Player>,
    player_turn: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct UnwrappedGameState {
    board: Vec<UnwrappedTile>,
    tile_stack: Vec<UnwrappedTile>,
    players: Vec<UnwrappedPlayer>,
    player_turn: usize,
}

impl Serialize for GameState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.unwrap().serialize(serializer)
    }
}

impl GameState {
    fn unwrap(&self) -> UnwrappedGameState {
        UnwrappedGameState {
            board: self.board.iter().map(|tile| tile.unwrap()).collect(),
            tile_stack: self.tile_stack.iter().map(|tile| tile.unwrap()).collect(),
            players: self.players.iter().map(|player| player.unwrap()).collect(),
            player_turn: self.player_turn,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.tile_stack = generate_tile_stack();
        self.board = [Tile::default(); 36];
        self.players = vec![];
        self.player_turn = 0;
    }

    pub(crate) fn add_player(&mut self, position_on_board: usize, position_on_tile: u8) {
        if self.tile_stack.len() < 3 {
            return;
        }

        let mut new_player = Player::default();

        new_player.set_position(position_on_board, deserialize_position_on_tile(position_on_tile));

        new_player.draw_from(&mut self.tile_stack);
        new_player.draw_from(&mut self.tile_stack);
        new_player.draw_from(&mut self.tile_stack);

        self.players.push(new_player);
    }

    pub(crate) fn get_player(&mut self, player_index: usize) -> &mut Player {
        &mut self.players[player_index]
    }

    fn get_current_player(&mut self) -> &mut Player {
        &mut self.players[self.player_turn]
    }

    pub(crate) fn place_tile(&mut self, player_index: usize, tile_index: usize) {
        let player = &mut self.players[player_index];
        let board = &mut self.board;
        player.place_tile(board, tile_index);

        let tile_stack = &mut self.tile_stack;
        player.draw_from(tile_stack);

        // update player positions also handles player death
        self.update_positions();

        self.next_player();
    }

    fn update_positions(&mut self) {
        let board = &mut self.board;
        let tile_stack = &mut self.tile_stack;
        for player in self.players.iter_mut() {
            if player.is_dead {
                continue;
            }
            update_player_position(player, board, tile_stack);
        }
    }

    fn get_is_game_over(&self) -> bool {
        self.players.iter().all(|player| player.is_dead)
    }

    fn next_player(&mut self) {
        if self.get_is_game_over() {
            return;
        }
        // skip dead players
        loop {
            self.player_turn = (self.player_turn + 1) % self.players.len();
            if !self.get_current_player().is_dead {
                break;
            }
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            tile_stack: generate_tile_stack(),
            board: [Tile::default(); 36],
            players: vec![],
            player_turn: 0,
        }
    }
}