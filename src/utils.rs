use crate::{
    info::{Game, Push, State, BOX, DIRECTIONS, FLOOR},
    reach::{is_reachable, ReachableTiles, is_reachable_box},
};

pub fn get_pushes(game: &Game, s: &State, reach: &ReachableTiles) -> Vec<Push>{
    let mut pushes: Vec<Push> = vec![];

    for (box_number, pos) in s.boxes_pos.iter().enumerate() {
        if is_reachable_box(*pos, reach) {
            for dir in DIRECTIONS {
                let newt = (*pos as isize + game.step_fore[dir]) as usize;
                let ppos = (*pos as isize - game.step_fore[dir]) as usize;

                if is_reachable(ppos, reach) && ((s.board[newt] & (FLOOR + BOX)) == FLOOR) {
                    pushes.push(Push { box_number, dir })
                }
            }
        }
    }

    return pushes;
}
