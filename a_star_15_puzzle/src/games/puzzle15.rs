use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, HashMap};
use rand::prelude::*;
use rand_pcg::Pcg64;
use rand::seq::SliceRandom;
use once_cell::sync::Lazy;

use crate::games::walking_distance::simulation;

static LOOKUP_WD_TABLE: Lazy<HashMap<[[u8; 4]; 4], u8>> = Lazy::new(|| simulation());

// Define the goal state
const GOAL_STATE: [[u8; 4]; 4] = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 0]];

// Define the Manhattan distance heuristic function
fn manhattan_distance(state: &[[u8; 4]; 4]) -> u8 {
    let mut distance = 0;
    for i in 0..4 {
        for j in 0..4 {
            if state[i][j] != 0 {
                let x = (state[i][j] - 1) / 4;
                let y = (state[i][j] - 1) % 4;
                distance += (i as i8 - x as i8).abs() as u8 + (j as i8 - y as i8).abs() as u8;
            }
        }
    }
    distance
}

// Define the Linear Conflict distance heuristic function
fn linear_conflict(state: &[[u8; 4]; 4]) -> u8 {
    let mut count = 0;
    for i in 0..4 {
        count += linear_conflict_row(&state[i]);
        count += linear_conflict_column(&[state[0][i], state[1][i], state[2][i], state[3][i]]);
    }
    return count;
}

fn linear_conflict_row(row: &[u8; 4]) -> u8 {
    let mut count = 0;
    for i in 0..3 {
        if row[i] != 0 && row[i] / 4 == i as u8 {
            for j in (i + 1)..4 {
                if row[j] != 0 && row[j] / 4 == i as u8 && row[i] > row[j] {
                    count += 2;
                }
            }
        }
    }
    return count;
}

fn linear_conflict_column(column: &[u8; 4]) -> u8 {
    linear_conflict_row(&[column[0], column[1], column[2], column[3]])
}

fn walking_distance(state: &[[u8; 4]; 4]) -> u8 {
    let mut h_wd_board: [[u8; 4]; 4] = [[0; 4]; 4];
    let mut v_wd_board: [[u8; 4]; 4] = [[0; 4]; 4];
    let goal =  GOAL_STATE;
    for i in 0..4 {
        for j in 0..4 {
            if state[i][j] != 0 {
                if state[i][j] == goal[i][j]{
                    h_wd_board[i][i] += 1;
                }else {
                    let val = (((state[i][j] as i8 - 1) / 4) as f32).floor() as usize;
                    h_wd_board[i][val] += 1;
                }
            }
            if state[j][i] != 0 {
                if state[j][i] == goal[j][i]{
                    v_wd_board[i][i] += 1;
                }else {
                    let val = ((state[j][i] + 3) %  4) as usize;
                    v_wd_board[i][val] += 1;
                }
            }
        }
    }
    let mut count = 0;
    count += LOOKUP_WD_TABLE.get(&h_wd_board).unwrap();
    count += LOOKUP_WD_TABLE.get(&v_wd_board).unwrap();
    return count;
}

// Define the State struct to represent a state in the search
#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub struct State {
    state: [[u8; 4]; 4],
    cost: u8,
    linear_conflict: u8,
    walking_distance: u8,
    manhattan: u8,
    parent: Option<Box<State>>,
}

impl State {
    fn new(state: [[u8; 4]; 4], cost: u8, parent: Option<Box<State>>) -> Self {
        let linear_conflict = linear_conflict(&state);
        let walking_distance = walking_distance(&state);
        let manhattan = manhattan_distance(&state);
        Self {
            state,
            cost,
            linear_conflict,
            walking_distance,
            manhattan,
            parent,
        }
    }

    fn is_goal_state(&self) -> bool {
        self.state == GOAL_STATE
    }

    fn successors(&self) -> Vec<Self> {
        let mut successors = vec![];
        let mut x = 0;
        let mut y = 0;
        for i in 0..4 {
            for j in 0..4 {
                if self.state[i][j] == 0 {
                    x = i;
                    y = j;
                }
            }
        }
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let nx = (x as i8 + dx) as usize;
            let ny = (y as i8 + dy) as usize;
            if nx < 4 && ny < 4 {
                let mut new_state = self.state.clone();
                new_state[x][y] = new_state[nx][ny];
                new_state[nx][ny] = 0;
                successors.push(Self::new(new_state,  self.cost + 1, Some(Box::new(self.clone()))));
            }
        }
        successors
    }

    fn total_cost(&self) -> u8 {
        self.cost + (self.manhattan / 3) + self.linear_conflict +self.walking_distance
    }
}

// Implement the Ord and PartialOrd traits for the State struct, to enable ordering in BinaryHeap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.total_cost().cmp(&self.total_cost())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Define the A* search function
pub fn a_star_search(start_state: [[u8; 4]; 4]) -> Option<Vec<[[u8; 4]; 4]>> {
    // Define the priority queue to
    // store the states to be expanded, with the starting state as the first element
    let mut queue = BinaryHeap::new();
    queue.push(State::new(start_state, 0, None));
    // Define the set to store the visited states
    let mut visited = HashSet::new();
    while let Some(current_state) = queue.pop() {
        // Check if the current state is the goal state
        //println!("{:?}", current_state.total_cost());
        if current_state.is_goal_state() {
            // Trace back the path from the goal state to the start state
            let mut path = vec![];
            let mut current = &current_state;
            while let Some(parent) = &current.parent {
                path.push(current.state);
                current = parent;
            }
            path.push(current.state);
            path.reverse();
            return Some(path);
        }
    
        // Add the current state to the visited set
        visited.insert(current_state.clone());
        
        // Generate the successor states and add them to the priority queue
        for successor_state in current_state.successors() {
            // Check if the successor state has already been visited
            if !visited.contains(&successor_state) {
                queue.push(successor_state);
            }
        }
        //println!("{:?}", &queue);
    }
    println!("visited nodes = {:?}", visited.len());
    // If the queue is empty and the goal state has not been found, return None
    None
}

fn get_invetrsion_count(linear_state: [u8; 16]) -> u16{
    let mut invetrsion_count = 0;
    for i in 0..16 {
        for j in (i+1)..16 {
            if linear_state[i] > 0 && linear_state[j] > 0 && linear_state[i] > linear_state[j] {
                invetrsion_count += 1;
            }
        }
    }
    return invetrsion_count;
}

//Checks if state is solvable
fn is_solvable(state: [[u8; 4]; 4]) -> bool{
    let mut linear_state: [u8; 16] = [0; 16];
    let mut counter = 0;
    for row in state {
        for val in row {
            linear_state[counter] = val;
            counter += 1;
        }
    }

    let inversion_count = get_invetrsion_count(linear_state);
    return inversion_count % 2 == 0;
}

//Shuffle state with n random moves
pub fn random_state() -> [[u8; 4]; 4]{
    let mut state:[[u8; 4]; 4];
    loop {
        let mut list: Vec<u8> = (0..16).collect();
        list.shuffle(&mut Pcg64::from_entropy());
        state = [list[0..4].try_into().expect("Something went Wrong!"), list[4..8].try_into().expect("Something went Wrong!"), list[8..12].try_into().expect("Something went Wrong!"), list[12..16].try_into().expect("Something went Wrong!")];
        if is_solvable(state) {
            break;
        }
    }
    return state;
}

//Shuffle state with Fisher–Yates shuffle
pub fn n_random_moves_from_goal(n: u16) -> [[u8; 4]; 4]{
    let mut state:[[u8; 4]; 4] = GOAL_STATE;
    let mut x = 0;
    let mut y = 0;
    for _ in 0..n {
        for i in 0..4 {
            for j in 0..4 {
                if state[i][j] == 0 {
                    x = i;
                    y = j;
                }
            }
        }
        let mut move_coords: Vec<(u8, u8)> = Vec::new();
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let nx = (x as i8 + dx) as usize;
            let ny = (y as i8 + dy) as usize;
            if nx < 4 && ny < 4 {
                move_coords.push((nx as u8, ny as u8));
            }
        }
        let coord = move_coords.choose(&mut Pcg64::from_entropy()).unwrap();
        state[x][y] = state[coord.0 as usize][coord.1 as usize];
        state[coord.0 as usize][coord.1 as usize] = 0;
    }
    return state;
}
