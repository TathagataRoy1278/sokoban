use std::usize::MAX;

use crate::info::{Board, Game, TileIndex, BOX, DIRECTIONS, FLOOR};

pub struct ReachableTiles {
    calculated: bool,
    min_reachable_tile: TileIndex,
    calc_counter: ReachabilityCount,

    pub tiles: Vec<ReachabilityCount>,
}

impl ReachableTiles {
    pub fn new(len: usize) -> ReachableTiles {
        return ReachableTiles {
            calculated: false,
            min_reachable_tile: 0,
            calc_counter: 0,
            tiles: vec![0; len],
        };
    }
}

type ReachabilityCount = usize;

pub fn is_reachable(tile: TileIndex, reach: &ReachableTiles) -> bool {
    reach.tiles[tile] == reach.calc_counter
}
pub fn is_reachable_box(tile: TileIndex, reach: &ReachableTiles) -> bool {
    reach.tiles[tile] == reach.calc_counter + 1
}

pub fn clear_reach(reach: &mut ReachableTiles, board: &Board) {
    reach.calculated = false;
    reach.min_reachable_tile = 0;
    reach.calc_counter = 0;

    for (index, tile_value) in board.iter().enumerate() {
        if tile_value & FLOOR > 0 {
            reach.tiles[index] = 0;
        } else {
            reach.tiles[index] = MAX;
        }
    }
}

pub fn calc_reach(reach: &mut ReachableTiles, game: &Game, start_tile: &TileIndex) -> usize {
    if reach.calc_counter >= MAX - 2 {
        clear_reach(reach, &game.board);
    }

    let board = &game.board;
    let start_tile = *start_tile;

    reach.calc_counter += 2;
    let mut n_reachable_tiles = 0;

    reach.min_reachable_tile = start_tile.clone();
    reach.tiles[start_tile] = reach.calc_counter;

    let mut stack: Vec<TileIndex> = vec![];
    stack.push(start_tile);

    while stack.len() > 0 {
        let tile = stack.pop().unwrap();

        for dir in DIRECTIONS {
            let newt = (tile as isize + game.step_fore[dir]) as usize;
            
            if reach.tiles[newt] < reach.calc_counter {
                //only floor as all walls have typemax value
                if board[newt] & BOX == 0 {
                    n_reachable_tiles += 1;
                    stack.push(newt);

                    reach.tiles[newt] = reach.calc_counter;
                    reach.min_reachable_tile = std::cmp::min(newt, reach.min_reachable_tile);
                } else {
                    reach.tiles[newt] = reach.calc_counter + 1;
                }
            }
        }
    }

    reach.calculated = true;
    return n_reachable_tiles;
}
