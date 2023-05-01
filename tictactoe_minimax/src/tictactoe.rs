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

/// Representation of a player in a game of Tic Tac Toe
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Player1,
    Player2    
}

/// Implementation for [`Player`]
impl Player {
    pub fn other(self) -> Player{
        match self {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1
        }
    }
}

/// Representation of a State in a our Game
#[derive(Debug, Clone, Copy)]
pub struct GameState {
    board: [[Option<Player>; BOARD_SIZE]; BOARD_SIZE],
    turn: Player,
}

/// Implementation for [`GameState`]
impl GameState {
    /// Produces a new instant of [`GameState`]
    pub fn new() -> Self {
        GameState { board: [[None; BOARD_SIZE]; BOARD_SIZE], turn: Player::Player1 }
    }
    pub fn print_board(&self) {
        for row in &self.board {
            for cell in row {
                match cell {
                    Some(p) => {
                        match p {
                            Player::Player1 => println!("X "),
                            Player::Player2 => println!("O "),
                        }
                    },
                    None => print!("_ "),
                }
            }
            println!();
        }
    }
    pub fn make_move(&mut self, destination: (usize, usize)){
        self.board[destination.0][destination.1] = Some(self.turn);
        self.turn = self.turn.other();
    }
    pub fn check_win(&self, player: Player) -> bool{
        for case in WIN_STATES{
            match self.board[case[0].0][case[0].1] {
                Some(p) => {
                    if p == player {
                        match self.board[case[1].0][case[1].1] {
                            Some(p) => {
                                if p == player {
                                    match self.board[case[2].0][case[2].1] {
                                        Some(p) => {
                                            if p == player {
                                                match self.board[case[3].0][case[3].1] {
                                                    Some(p) => {
                                                        if p == player {
                                                            return  true;
                                                        }
                                                    },
                                                    None => continue,
                                                }
                                            }
                                        },
                                        None => continue,
                                    }
                                }
                            },
                            None => continue,
                        }
                    }
                },
                None => continue,
            }
        }
        false
    }
    pub fn check_loose(&self, player: Player) -> bool{
        for case in LOOSE_STATES{
            match self.board[case[0].0][case[0].1] {
                Some(p) => {
                    if p == player {
                        match self.board[case[1].0][case[1].1] {
                            Some(p) => {
                                if p == player {
                                    match self.board[case[2].0][case[2].1] {
                                        Some(p) => {
                                            if p == player {
                                                return true;
                                            }
                                        },
                                        None => continue,
                                    }
                                }
                            },
                            None => continue,
                        }
                    }
                },
                None => continue,
            }
        }
        false
    }

    pub fn succesors(&self) -> Vec<GameState>{
        let mut succesors: Vec<GameState> = Vec::new();
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.board[i][j].is_none(){
                    let mut successor_board: [[Option<Player>; BOARD_SIZE]; BOARD_SIZE] = self.board;
                    successor_board[i][j] = Some(self.turn);
                    succesors.push(GameState { board: successor_board, turn: self.turn.other()  });
                }
            }
        }
        succesors
    }
    /// Evaluates if this node is terminal
    pub fn is_terminal(&self) -> bool{
        if self.check_win(Player::Player1) || self.check_loose(Player::Player1) || self.check_win(Player::Player2) || self.check_loose(Player::Player2) {
            return true;
        }
        false
    }
    /// Evaluates the [`GameState`] on a scale -100 to 100 
    /// Where -100 is a loss and 100 a win
    pub fn evaluate(&self) -> i32{
        let p = self.turn.other();
        // We return max heuristic value if a player wins or the other player looses
        if self.check_win(p) || self.check_loose(p.other()){
            return 100;
        }
        if self.check_win(p.other()) || self.check_loose(p){
            return -100;
        }
        0
    }
}
/// Returns the best move evaluated by our [`minimax`] algorithm
pub fn move_with_minimax(state: &GameState, depth: u8) -> (usize, usize) {
    let mut options: Vec<(i32, (usize, usize))> = Vec::new();
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            if state.board[i][j].is_none(){
                let mut tmp_game_state = *state;
                tmp_game_state.make_move((i, j));
                options.push((minimax(&tmp_game_state, depth, true),(i, j)));
            }
        }
    }
    options.sort_by(|a, b| (b.0).cmp(&a.0));
    options[0].1
}
/// Minimax algorithm for our modified Tic Tac Toe
pub fn minimax(state: &GameState, depth: u8, maximize: bool) -> i32 {
    //println!("new minimax");
    if depth == 0 || state.is_terminal() {
        return state.evaluate();
    }
    if maximize {
        let mut val = i32::MIN;
        for succesor in state.succesors() {
            val = Ord::max(val, minimax(&succesor, depth - 1, false));
        }
        val
    } else {
        let mut val = i32::MAX;
        for succesor in state.succesors() {
            val = Ord::min(val, minimax(&succesor, depth - 1, false));
        }
        val
    }
}
