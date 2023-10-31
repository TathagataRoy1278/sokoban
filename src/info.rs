#[derive(Clone)]
pub struct Game {
    pub height: usize,
    pub width: usize,
    pub board: Board,

    pub start_player: TileIndex,
    goals: Vec<TileIndex>,

    pub step_fore: [isize; 4],
}

pub fn box_to_char(tile: TileValue) -> char {
    let mut retval = CH_FLOOR;
    let pbgw = tile & (PLAYER + BOX + GOAL + WALL);

    retval = if pbgw == PLAYER {
        CH_PLAYER
    } else if pbgw == PLAYER + GOAL {
        CH_PLAYER_ON_GOAL
    } else if pbgw == BOX || pbgw == PLAYER + BOX {
        CH_BOX
    } else if pbgw == BOX + GOAL || pbgw == PLAYER + BOX + GOAL {
        CH_BOX_ON_GOAL
    } else if pbgw == GOAL {
        println!("DUDUD");
        CH_GOAL
    } else if pbgw == WALL {
        CH_WALL
    } else {
        retval
    };

    return retval;
}

pub fn board_to_string(game: &Game, board: &Board) -> String {
    let mut st = String::new();
    for row in 0..game.height+2 {
        for col in 0..game.width+2 {
            let v = board[col_row_to_tile_index(game.width, col, row)];
            st.push(box_to_char(v));
        }
        st.push('\n');
    }

    return st;
}

pub type TileIndex = usize;
pub type TileValue = usize;
pub type Board = Vec<TileIndex>;

pub type BoxNumber = usize;
pub type Direction = usize;

pub const BOX: TileValue = 1 << 0;
pub const FLOOR: TileValue = 1 << 1;
pub const GOAL: TileValue = 1 << 2;
pub const PLAYER: TileValue = 1 << 3;
pub const WALL: TileValue = 1 << 4;

const CH_BOX: char = 'b';
const CH_BOX_ON_GOAL: char = 'B';
const CH_GOAL: char = '.';
const CH_FLOOR: char = ' ';
const CH_NON_BLANK_FLOOR: char = '-';
const CH_PLAYER: char = 'p';
const CH_PLAYER_ON_GOAL: char = 'P';
const CH_SQUARE_SET: char = '%';
const CH_WALL: char = '#';

pub const DIRECTIONS: [usize; 4] = [0, 1, 2, 3];

#[derive(Clone)]
pub struct State {
    pub player: TileIndex,
    pub boxes_pos: Vec<TileIndex>,
    pub board: Board,
    pub zhash: u64
}

#[derive(Clone, Debug)]
pub struct Push {
    pub box_number: BoxNumber,
    pub dir: Direction,
}

pub fn moveplayer(s: &mut State, game: &Game, a: &Push) {
    let cpos = s.boxes_pos[a.box_number]; //Gets the position of the box
    let newpos = (cpos as isize + game.step_fore[a.dir]) as usize;
    s.board[s.player] &= !PLAYER;
    s.board[cpos] &= !BOX;

    s.board[newpos] |= BOX;
    s.player = cpos;
    s.boxes_pos[a.box_number] = newpos;
    s.board[s.player] |= PLAYER;
}

pub fn unmove(s: &mut State, game: &Game, a: &Push) {
    let cpos = s.boxes_pos[a.box_number];
    let newpos = (cpos as isize - game.step_fore[a.dir]) as usize;

    s.board[s.player] &= !PLAYER;
    s.board[cpos] &= !BOX;

    s.player = (newpos as isize - game.step_fore[a.dir]) as usize;
    s.boxes_pos[a.box_number] = newpos;

    s.board[s.player] |= PLAYER;
    s.board[newpos] |= BOX;
}

pub fn is_solved(game: &Game, s: &State) -> bool {
    for i in &s.boxes_pos {
        if game.board[*i] & GOAL == 0 {
            return false;
        }
    }

    return true;
}

pub fn char_to_square(c: char) -> Option<TileValue> {
    let opt = if c == CH_BOX {
        //box
        BOX + FLOOR
    } else if c == CH_BOX_ON_GOAL
    //BOX ON GOAL
    {
        BOX + GOAL + FLOOR
    } else if c == CH_GOAL {
        GOAL + FLOOR
    } else if c == CH_FLOOR || c == CH_NON_BLANK_FLOOR {
        FLOOR
    } else if c == CH_PLAYER {
        PLAYER + FLOOR
    } else if c == CH_PLAYER_ON_GOAL {
        PLAYER + GOAL + FLOOR
    } else if c == CH_WALL {
        WALL
    } else {
        return None;
    };

    return Some(opt);
}

fn col_row_to_tile_index(board_width: usize, col: usize, row: usize) -> TileIndex {
    return row * (board_width + 2) + col;
}

pub fn load_from_string(code: &str) -> Game {
    let mut running_board_width: usize = 0;
    let mut board_width: usize = 0;
    let mut board_height: usize = 1;

    //makes sure that the width is consistent
    for c in code.chars().into_iter() {
        if c == '\n' {
            if board_width == 0 {
                board_width = running_board_width;
            } else {
                assert_eq!(board_width, running_board_width, "Inconsistent board width");
            }

            running_board_width = 0;
            board_height += 1;
        } else {
            running_board_width += 1;
        }
    }

    let board_size = board_height * board_width;
    println!("boardsize - {}, {}", board_height, board_size);
    let mut start_board = vec![0; board_size];

    board_width -= 2;
    board_height -= 2;

    let mut board = vec![0; board_size];
    for i in 0..(board_width + 2) {
        board[i] = WALL;
    }

    for i in board_size - (board_width + 2)..board_size {
        board[i] = WALL;
    }

    let mut offset = 0;
    for i in 0..board_height {
        offset += board_width + 2;
        board[offset] = WALL;
        board[offset + board_width + 1] = WALL;
    }

    let mut goals = vec![];
    let mut boxes = vec![];

    let mut start_player_tile = 0;
    let mut row = 0;
    let mut col = 0;

    for c in code.chars().into_iter() {
        let v = char_to_square(c);

        if v.is_none() {
            row += 1;
            col = 0;
            continue;
        }
        let v = v.unwrap();

        let a = col_row_to_tile_index(board_width, col, row);
        
        start_board[a] = v;

        if row > 0 && row <= (board_height + 1) && col > 0 && col <= (board_width + 1) {
            board[a] = v;

            if (v & PLAYER) > 0 {
                start_player_tile = a;
            }

            if (v & GOAL) > 0 {
                goals.push(a);
            }

            if (v & BOX) > 0 {
                boxes.push(a);
            }
        }

        col += 1;
    }
    println!("end boardsize - {}, {}", board_height, board_width);
    let step_fore: [isize; 4] = [-(board_width as isize + 2), -1, board_width as isize + 2, 1];
    
    return Game {
        height: board_height,
        width: board_width,
        board: board,

        start_player: start_player_tile,
        goals: goals,
        step_fore: step_fore,
    };
}

pub fn get_state(game: &Game, board: &Board) -> State {
    let mut player = 0;
    let mut boxes = vec![];

    for (i, v) in board.iter().enumerate() {
        if (v & PLAYER) > 0 {
            player = i;
        }

        if v & BOX > 0 {
            boxes.push(i);
        }
    }

    return State {
        player: player,
        boxes_pos: boxes,
        board: board.clone(),
        zhash: 0
    }
}