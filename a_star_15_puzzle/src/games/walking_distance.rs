use std::collections::{VecDeque, HashSet, HashMap};  

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Node{
    board: [[u8; 4]; 4],
    h: u8
}

impl Node {
    fn successors(&self) -> Vec<Self> {
        let mut empty_slot: usize = 0;
        let mut succesors: Vec<Self> = Vec::new();
        let mut counter: u8 = 0;
        for i in 0..4 {
            for j in 0..4 {
                counter += self.board[i][j];
            }
            if counter == 3{
                empty_slot = i;
                break;
            }
            counter = 0;
        }
        if empty_slot != 3 {
            for i in 0..4 {
                let mut tmp = self.board.clone();
                if tmp[empty_slot + 1][i] != 0 {
                    tmp[empty_slot + 1][i] -= 1;
                    tmp[empty_slot][i] += 1;
                    succesors.push(Self{board: tmp, h: self.h+1});
                }
            }
        }
        if empty_slot != 0{
            for i in 0..4 {
                let mut tmp = self.board.clone();
                if tmp[empty_slot - 1][i] != 0 {
                    tmp[empty_slot - 1][i] -= 1;
                    tmp[empty_slot][i] += 1;
                    succesors.push(Self{board: tmp, h: self.h+1});
                }
            }
        }
        return succesors;
    }
}

pub fn simulation() -> HashMap<[[u8; 4]; 4], u8>{
    let mut goal_board: [[u8; 4]; 4] = [[0 ;4]; 4];
    // Initialize the board
    goal_board[0][0] = 4;
    goal_board[1][1] = 4;
    goal_board[2][2] = 4;
    goal_board[3][3] = 3;
    let starting_node = Node{board: goal_board, h: 0};
    // Perform breadth-first search
    let mut queue = VecDeque::new();
    let mut explored = HashSet::new();
    // Add the initial state to the queue and the explored set
    queue.push_back(starting_node);
    explored.insert(goal_board);
    let mut unique_boards: Vec<Node> = Vec::new(); 
    unique_boards.push(starting_node);
    while let Some(board) = queue.pop_front() {

        // Generate the next states and add them to the queue and the explored set
        for next_board in board.successors() {
            if !explored.contains(&next_board.board) {
                queue.push_back(next_board);
                explored.insert(next_board.board);
                //println!("{:?}", &next_board);
                unique_boards.push(next_board);
            }
        }
    }
    //println!("{:?}", unique_boards.len());
    let mut lookup_table = HashMap::new();
    for entry in unique_boards {
        lookup_table.insert(entry.board, entry.h);
    }
    return lookup_table;
}
