use std::time::Instant;

use crate::{
    info::{is_solved, moveplayer, unmove, Game, Push, State},
    reach::{calc_reach, clear_reach, ReachableTiles},
    utils::get_pushes,
};

pub struct DepthFirstSearch {
    pub d_max: usize
}

#[derive(Debug)]
pub struct DeapthFirstSearchData {
    pub solved: bool,
    max_depth: usize,
    n_pushes_evaled: usize,

    solve_time: u128,
    pushes: Vec<Push>,
    sol_depth: usize,
}

fn solve_aux(
    dfs: &DepthFirstSearch,
    data: &mut DeapthFirstSearchData,
    game: &Game,
    mut s: &mut State,
    reach: &mut ReachableTiles,
    d: usize,
) -> bool {
    if is_solved(game, s) {
        data.solved = true;
        data.sol_depth = d;
        return true;
    }

    if d >= dfs.d_max {
        return false;
    }

    calc_reach(reach, &game, &s.player);
    for push in get_pushes(game, &s, &reach) {
        data.n_pushes_evaled += 1;
        moveplayer(&mut s, &game, &push);

        if solve_aux(dfs, data, game, s, reach, d + 1) {
            data.pushes[d + 1] = push;
            return true;
        }
        unmove(&mut s, game, &push);
    }

    return true;
}

pub fn solve(
    dfs: DepthFirstSearch,
    game: Game,
    mut s: State,
    mut reach: ReachableTiles,
) -> (DeapthFirstSearchData, State) {
    let tstart = Instant::now();

    let mut data = DeapthFirstSearchData {
        solved: false,
        max_depth: 0,
        n_pushes_evaled: 0,

        solve_time: 0,
        pushes: vec![
            Push {
                box_number: 0,
                dir: 0
            };
            dfs.d_max
        ],
        sol_depth: 0,
    };

    clear_reach(&mut reach, &game.board);
    solve_aux(&dfs, &mut data, &game, &mut s, &mut reach, 0);

    data.solve_time = tstart.elapsed().as_micros();
    return (data, s);
}
