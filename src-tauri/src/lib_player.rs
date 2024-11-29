use lib_tile::flip_points;
use crate::game_state::GameBoard;
use crate::lib_tile;
use crate::player::Player;
use crate::tile::{Tile, TileConnections};

// finds the connection that goes from the specified position on the tile
//  and returns the position on the tile when the connection is walked across
fn walk_connection(connections: TileConnections, source_position_mask: u8) -> u8 {
    // find the connection that has a bit high in the position given
    for connection in connections.iter() {
        if connection & source_position_mask != 0 {
            // we found the connection,
            //  the other high bit is the position on the tile we end up in
            return connection ^ source_position_mask;
        }
    }
    // this should never happen but no connection was found.
    //  this means the player has reached an empty tile
    //  and should not be moved
    source_position_mask
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

pub fn update_player_position(player: &mut Player, board: &GameBoard, tile_stack: &mut Vec<Tile>) {
    // get the tile the player is standing on
    let tile = board[player.position_on_board];

    if tile.is_empty() {
        return;
    }

    // walk the connection, stay on the same tile
    let new_tile_position = walk_connection(tile.connections, player.tile_position_mask);

    // check what tile we are moving to
    let transition = transition_tile(new_tile_position);

    // check if we are about to move out of bounds
    if
        (player.position_on_board % 6 == 0 && transition == -1) || (player.position_on_board % 6 == 5 && transition == 1)
        || (player.position_on_board < 6 && transition == -6) || (player.position_on_board > 29 && transition == 6)
    {
        // we dead

        // even though we are dead we still need to update the player's position
        player.tile_position_mask = new_tile_position;

        player.set_dead(tile_stack);

        return;
    }

    // because we just moved to a new tile,
    //  we need to flip or position to appear in the same position on the new tile
    let new_tile_position = flip_points(new_tile_position);

    player.tile_position_mask = new_tile_position;
    player.position_on_board = (player.position_on_board as isize + transition) as usize;

    // we might have ended up at another tile so let's go again
    update_player_position(player, board, tile_stack);
}