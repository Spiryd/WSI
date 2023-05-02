use rand::seq::SliceRandom;
use rand::prelude::*;
use rand_pcg::Pcg64;

/// Edge length of the game board
const BOARD_SIZE: usize = 5;

/// States where the game is won
const WIN_STATES: [[(usize, usize); 4]; 28] = [ 
    [ (0,0), (0,1), (0,2), (0,3) ],
    [ (1,0), (1,1), (1,2), (1,3) ],
    [ (2,0), (2,1), (2,2), (2,3) ],
    [ (3,0), (3,1), (3,2), (3,3) ],
    [ (4,0), (4,1), (4,2), (4,3) ],
    [ (0,1), (0,2), (0,3), (0,4) ],
    [ (1,1), (1,2), (1,3), (1,4) ],
    [ (2,1), (2,2), (2,3), (2,4) ],
    [ (3,1), (3,2), (3,3), (3,4) ],
    [ (4,1), (4,2), (4,3), (4,4) ],
    [ (0,0), (1,0), (2,0), (3,0) ],
    [ (0,1), (1,1), (2,1), (3,1) ],
    [ (0,2), (1,2), (2,2), (3,2) ],
    [ (0,3), (1,3), (2,3), (3,3) ],
    [ (0,4), (1,4), (2,4), (3,4) ],
    [ (1,0), (2,0), (3,0), (4,0) ],
    [ (1,1), (2,1), (3,1), (4,1) ],
    [ (1,2), (2,2), (3,2), (4,2) ],
    [ (1,3), (2,3), (3,3), (4,3) ],
    [ (1,4), (2,4), (3,4), (4,4) ],
    [ (0,1), (1,2), (2,3), (3,4) ],
    [ (0,0), (1,1), (2,2), (3,3) ],
    [ (1,1), (2,2), (3,3), (4,4) ],
    [ (1,0), (2,1), (3,2), (4,3) ],
    [ (0,3), (1,2), (2,1), (3,0) ],
    [ (0,4), (1,3), (2,2), (3,1) ],
    [ (1,3), (2,2), (3,1), (4,0) ],
    [ (1,4), (2,3), (3,2), (4,1) ]
];

/// States where the game is lost
const LOOSE_STATES: [[(usize, usize); 3]; 48] = [
    [ (0,0), (0,1), (0,2) ], [ (0,1), (0,2), (0,3) ], [ (0,2), (0,3), (0,4) ], 
    [ (1,0), (1,1), (1,2) ], [ (1,1), (1,2), (1,3) ], [ (1,2), (1,3), (1,4) ], 
    [ (2,0), (2,1), (2,2) ], [ (2,1), (2,2), (2,3) ], [ (2,2), (2,3), (2,4) ], 
    [ (3,0), (3,1), (3,2) ], [ (3,1), (3,2), (3,3) ], [ (3,2), (3,3), (3,4) ], 
    [ (4,0), (4,1), (4,2) ], [ (4,1), (4,2), (4,3) ], [ (4,2), (4,3), (4,4) ], 
    [ (0,0), (1,0), (2,0) ], [ (1,0), (2,0), (3,0) ], [ (2,0), (3,0), (4,0) ], 
    [ (0,1), (1,1), (2,1) ], [ (1,1), (2,1), (3,1) ], [ (2,1), (3,1), (4,1) ], 
    [ (0,2), (1,2), (2,2) ], [ (1,2), (2,2), (3,2) ], [ (2,2), (3,2), (4,2) ], 
    [ (0,3), (1,3), (2,3) ], [ (1,3), (2,3), (3,3) ], [ (2,3), (3,3), (4,3) ], 
    [ (0,4), (1,4), (2,4) ], [ (1,4), (2,4), (3,4) ], [ (2,4), (3,4), (4,4) ], 
    [ (0,2), (1,3), (2,4) ], [ (0,1), (1,2), (2,3) ], [ (1,2), (2,3), (3,4) ], 
    [ (0,0), (1,1), (2,2) ], [ (1,1), (2,2), (3,3) ], [ (2,2), (3,3), (4,4) ], 
    [ (1,0), (2,1), (3,2) ], [ (2,1), (3,2), (4,3) ], [ (2,0), (3,1), (4,2) ], 
    [ (0,2), (1,1), (2,0) ], [ (0,3), (1,2), (2,1) ], [ (1,2), (2,1), (3,0) ], 
    [ (0,4), (1,3), (2,2) ], [ (1,3), (2,2), (3,1) ], [ (2,2), (3,1), (4,0) ], 
    [ (1,4), (2,3), (3,2) ], [ (2,3), (3,2), (4,1) ], [ (2,4), (3,3), (4,2) ]
];

fn other(p: u8) -> u8{
    if p == 1{
        2
    } else {
        1
    }
}

/// Representation of a State in a our Game
#[derive(Debug, Clone, Copy)]
pub struct GameState {
    board: [[u8; BOARD_SIZE]; BOARD_SIZE],
    turn: u8,
}

/// Implementation for [`GameState`]
impl GameState {
    /// Produces a new instant of [`GameState`]
    pub fn new() -> Self {
        GameState { board: [[0; BOARD_SIZE]; BOARD_SIZE], turn: 1 }
    }
    pub fn print_board(&self) {
        for row in &self.board {
            for cell in row {
                match cell {
                    1 => print!("X "),
                    2 => print!("O "),
                    _ => print!("_ "),
                }
            }
            println!();
        }
    }
    pub fn make_move(&mut self, destination: (usize, usize)){
        self.board[destination.0][destination.1] = self.turn;
        if self.turn == 1{
            self.turn = 2;
        } else {
            self.turn = 1;
        }
    }
    pub fn check_win(&self, player: u8) -> bool{
        for case in WIN_STATES{
            if self.board[case[0].0][case[0].1] == player && self.board[case[1].0][case[1].1] == player && self.board[case[2].0][case[2].1] == player && self.board[case[3].0][case[3].1] == player {
                return true;
            }
        }
        false
    }
    pub fn check_loose(&self, player: u8) -> bool{
        for case in LOOSE_STATES{
            if self.board[case[0].0][case[0].1] == player && self.board[case[1].0][case[1].1] == player && self.board[case[2].0][case[2].1] == player {
                return true;
            }
        }
        false
    }

    pub fn succesors(&self) -> Vec<GameState>{
        let mut succesors: Vec<GameState> = Vec::new();
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.board[i][j] == 0 {
                    let mut successor_board: [[u8; BOARD_SIZE]; BOARD_SIZE] = self.board;
                    let succesor_turn: u8 = other(self.turn);
                    successor_board[i][j] = self.turn;
                    succesors.push(GameState { board: successor_board, turn: succesor_turn });
                }
            }
        }
        succesors
    }
    /// Evaluates if this node is terminal
    pub fn is_terminal(&self) -> bool{
        if self.check_win(1) || self.check_loose(1) || self.check_win(2) || self.check_loose(2) {
            return true;
        }
        false
    }
    /// Evaluates the [`GameState`] on a scale -100 to 100 
    /// Where -100 is a loss and 100 a win
    pub fn evaluate(&self) -> i32{
        let p: u8 = other(self.turn);
        // We return max heuristic value if a player wins or the other player looses
        if self.check_win(p) || self.check_loose(other(p)){
            return 100;
        }
        if self.check_win(other(p)) || self.check_loose(p){
            return -100;
        }
        0
    }
}

/// Returns the best move evaluated by our [`minimax`] algorithm
pub fn move_with_minimax(state: &GameState, depth: u8) -> (usize, usize) {
    let mut options: Vec<(i32, (usize, usize))> = Vec::new();
    let mut mv: (usize, usize) = (0, 0);
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if state.board[i][j] == 0 {
                mv = (i, j);
                let mut tmp_game_state = *state;
                tmp_game_state.make_move((i, j));
                if !tmp_game_state.check_loose(state.turn) {
                    options.push((minimax(&tmp_game_state, depth, true, i32::MIN, i32::MAX),(i, j)));
                }           
            }
        }
    }
    options.sort_by(|a, b| (b.0).cmp(&a.0));
    println!("{:?}", options);
    if options.is_empty(){
        return mv;
    }
    let min = options[0].0;
    let mut chooices: Vec<(usize, usize)> = Vec::new();
    for option in options{
        if option.0 == min {
            chooices.push(option.1);
        }
    }
    *chooices.choose(&mut Pcg64::from_entropy()).unwrap()
}
/// Minimax algorithm for our modified Tic Tac Toe
pub fn minimax(state: &GameState, depth: u8, maximize: bool, mut alpha: i32, mut beta: i32) -> i32 {
    //println!("new minimax");
    if depth == 0 || state.is_terminal() {
        let eval = state.evaluate();
        /* 
        if eval == -100 || eval == 100 {
            println!();
            state.print_board();
            println!("{eval}");
            println!("{}", state.turn);
            println!();
        }
        */
        return eval;
    }
    if maximize {
        let mut val = i32::MIN;
        for succesor in state.succesors() {
            val = val.max(minimax(&succesor, depth - 1, false, alpha, beta));
            alpha = alpha.max(val);
            if val > beta {
                break;
            }
        }
        val
    } else {
        let mut val = i32::MAX;
        for succesor in state.succesors() {
            val = val.min(minimax(&succesor, depth - 1, true, alpha, beta));
            beta = beta.min(val);
            if val < alpha {
                break;
            }
        }
        val
    }
}
