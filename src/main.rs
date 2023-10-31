use std::process::exit;

use dfs::{solve, DepthFirstSearch};
use info::{get_state, State, Game, Push};
use levels::BOARD_EASY;
use reach::{calc_reach, ReachableTiles};

use crate::{info::{board_to_string, load_from_string, moveplayer}, reach::clear_reach, levels::{BOARD_EASY2, BOARD_DOOR_PATTERN_3, BOARD_PITFALL}, dfs::isolve};

mod info;
mod dfs;
mod reach;
mod utils;
mod levels;
//mod zobrist;

fn draw_solution(pushes: Vec<Option<Push>>, mut s: State, game: &Game) {
    println!("Initial - {}", board_to_string(game, &s.board));
    for push in pushes.iter() {
        if push.is_none() {
            continue;
        }

        let push = push.as_ref().unwrap();
        moveplayer(&mut s, game, &push);
        println!("Move - {:?}", push);
        println!("{}", board_to_string(game, &s.board));
        println!();
    }
}

fn main() {
    let game_string = BOARD_PITFALL;

    let game = load_from_string(game_string);
    let dfs = DepthFirstSearch {
        d_max: 12
    };

    let mut freach = ReachableTiles::new(game.board.len());
    
    let board = board_to_string(&game, &game.board);
    println!("{}", board);

    clear_reach(&mut freach, &game.board);
    calc_reach(&mut freach, &game, &game.board, &game.start_player);

    let state = get_state(&game, &game.board);
    
    let board = board_to_string(&game, &game.board);
    println!("{}", board);

    let data = solve(dfs, game.clone(), state.clone(), freach);
    if !data.solved {
        println!("Solution not found.");
        exit(0);
    }
    draw_solution(data.pushes, state, &game);
    
}
