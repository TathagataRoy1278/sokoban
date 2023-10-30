use dfs::{solve, DepthFirstSearch};
use info::get_state;
use levels::BOARD_EASY;
use reach::{calc_reach, ReachableTiles};

use crate::{info::{board_to_string, load_from_string}, reach::clear_reach};

mod info;
mod dfs;
mod reach;
mod utils;
mod levels;

fn main() {
    let game_string = BOARD_EASY;

    let game = load_from_string(game_string);
    let dfs = DepthFirstSearch {
        d_max: 100000
    };

    let mut freach = ReachableTiles::new(game.board.len());
    
    let board = board_to_string(&game, &game.board);
    println!("{}", board);

    clear_reach(&mut freach, &game.board);
    calc_reach(&mut freach, &game, &game.start_player);

    let state = get_state(&game, &game.board);
    
    let board = board_to_string(&game, &game.board);
    println!("{}", board);

    let (data, state) = solve(dfs, game.clone(), state, freach);
    
    let board = board_to_string(&game, &game.board);
    println!("{:?}", data.solved);
    
}
