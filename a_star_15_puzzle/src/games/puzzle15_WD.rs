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
    walking_distance: u8,
    parent: Option<Box<State>>,
}

impl State {
    fn new(state: [[u8; 4]; 4], cost: u8, parent: Option<Box<State>>) -> Self {
        let walking_distance = walking_distance(&state);
        Self {
            state,
            cost,
            walking_distance,
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
        self.cost + self.walking_distance
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
            println!("number of visited states = {:?}", visited.len());
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
    // If the queue is empty and the goal state has not been found, return None
    None
}

fn get_invetrsion_count(linear_state: [u8; 16]) -> u16{
    let mut invetrsion_count = 0;
    for i in 0..15 {
        for j in (i+1)..16 {
            if linear_state[j] != 0 && linear_state[i] != 0 && linear_state[i] > linear_state[j] {
                invetrsion_count += 1;
            }
        }
    }
    return invetrsion_count;
}

fn find_slot_position(state: [[u8; 4]; 4]) -> usize{
    // start from bottom-right corner of matrix
    for i in (0..4).rev() {
        for j in (0..4).rev() {
            if state[i][j] == 0 {
                return 4 - i;
            }
        }        
    }
    return 0;
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
    let pos_x = find_slot_position(state);
    if pos_x % 2 == 1{
        return inversion_count % 2 == 0;
    } else {
        return inversion_count % 2 == 1;
    }
}

//Shuffle state with Fisher–Yates shuffle
pub fn random_state() -> [[u8; 4]; 4]{
    let mut state:[[u8; 4]; 4];
    loop {
        let mut list: Vec<u8> = (0..16).collect();
        list.shuffle(&mut Pcg64::from_entropy());
        state = [list[0..4].try_into().expect("Something went Wrong!"), list[4..8].try_into().expect("Something went Wrong!"), list[8..12].try_into().expect("Something went Wrong!"), list[12..16].try_into().expect("Something went Wrong!")];
        if is_solvable(state) {
            break;
        }
        //println!("pool");
    }
    return state;
}

//Shuffle state with n random moves
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

// Define the IDA* search function
pub fn ida_star_search(start_state: [[u8; 4]; 4]) -> Option<Vec<[[u8; 4]; 4]>> {
    // Define the initial bound as the heuristic value of the starting state
    let mut bound = State::new(start_state, 0, None).total_cost();
    // Define the set to store the visited states
    let mut visited = HashSet::new();
    // Trace back the path from the start state
    let mut path = vec![start_state];
    // Loop until a solution is found or the maximum bound is exceeded
    loop {
        let t = search(&mut path, 0, bound as u32, &mut visited);
        if t == 0 {
            // If the search returns 0, it means a solution was found
            println!("number of visited states = {:?}", visited.len());
            return Some(path);
        } else if t == std::u32::MAX {
            // If the search returns the maximum u32 value, it means the maximum bound was exceeded
            return None;
        } else {
            // If the search returns a non-zero value, it means the bound needs to be increased
            bound = t as u8;
        }
    }
}

// Define the search function that performs IDDFS with a given bound
fn search(path: &mut Vec<[[u8; 4]; 4]>, g: u8, bound: u32, visited: &mut HashSet<State>) -> u32 {
    //println!("{:?}", path);
    let current_state = State::new(path.last().unwrap().clone(), g, None);
    //println!("{:?}", current_state.total_cost());
    let f = current_state.total_cost();
    if f > bound as u8 {
        return f as u32;
    }
    if current_state.is_goal_state() {
        return 0;
    }
    let mut min_cost = std::u32::MAX;
    visited.insert(current_state.clone());
    for successor_state in current_state.successors() {
        if !visited.contains(&successor_state) {
            path.push(successor_state.state);
            let t = search(path, g + 1, bound, visited);
            if t == 0 {
                // If a solution was found, return 0
                return 0;
            } else if t < min_cost {
                // Otherwise, update the minimum cost
                min_cost = t;
            }
            path.pop();
        }
    }
    min_cost
}
