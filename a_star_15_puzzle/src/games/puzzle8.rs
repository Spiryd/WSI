use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use rand::prelude::*;
use rand_pcg::Pcg64;

// Define the goal state
const GOAL_STATE: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 0]];

// Define the Manhattan distance heuristic function
fn manhattan_distance(state: &[[u8; 3]; 3]) -> u8 {
    let mut distance = 0;
    for i in 0..3 {
        for j in 0..3 {
            if state[i][j] != 0 {
                let x = (state[i][j] - 1) / 3;
                let y = (state[i][j] - 1) % 3;
                distance += (i as i8 - x as i8).abs() as u8 + (j as i8 - y as i8).abs() as u8;
            }
        }
    }
    distance
}

// Define the Linear Conflict distance heuristic function
fn linear_conflict(state: &[[u8; 3]; 3]) -> u8 {
    let mut count = 0;
    for i in 0..3 {
        count += linear_conflict_row(&state[i]);
        count += linear_conflict_column(&[state[0][i], state[1][i], state[2][i]]);
    }
    count
}

fn linear_conflict_row(row: &[u8; 3]) -> u8 {
    let mut count = 0;
    for i in 0..3 {
        if row[i] != 0 && row[i] / 3 == i as u8 {
            for j in (i + 1)..2 {
                if row[j] != 0 && row[j] / 3 == i as u8 && row[i] > row[j] {
                    count += 2;
                }
            }
        }
    }
    count
}

fn linear_conflict_column(column: &[u8; 3]) -> u8 {
    linear_conflict_row(&[column[0], column[1], column[2]])
}

// Define the State struct to represent a state in the search
#[derive(Clone, Eq, Hash, PartialEq)]
struct State {
    state: [[u8; 3]; 3],
    cost: u8,
    manhattan: u8,
    linear_conflict: u8,
    parent: Option<Box<State>>,
}

impl State {
    fn new(state: [[u8; 3]; 3], cost: u8, parent: Option<Box<State>>) -> Self {
        let manhattan = manhattan_distance(&state);
        let linear_conflict = linear_conflict(&state);
        Self {
            state,
            cost,
            manhattan,
            linear_conflict,
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
        for i in 0..3 {
            for j in 0..3 {
                if self.state[i][j] == 0 {
                    x = i;
                    y = j;
                }
            }
        }
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let nx = (x as i8 + dx) as usize;
            let ny = (y as i8 + dy) as usize;
            if nx < 3 && ny < 3 {
                let mut new_state = self.state.clone();
                new_state[x][y] = new_state[nx][ny];
                new_state[nx][ny] = 0;
                successors.push(Self::new(new_state, self.cost + 1, Some(Box::new(self.clone()))));
            }
        }
        successors
    }

    fn total_cost(&self) -> u8 {
        self.cost + self.manhattan + 2*self.linear_conflict
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
pub fn a_star_search(start_state: [[u8; 3]; 3]) -> Option<Vec<[[u8; 3]; 3]>> {
    // Define the priority queue to
    // store the states to be expanded, with the starting state as the first element
    let mut queue = BinaryHeap::new();
    queue.push(State::new(start_state, 0, None));
    // Define the set to store the visited states
    let mut visited = HashSet::new();
    while let Some(current_state) = queue.pop() {
        // Check if the current state is the goal state
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
    }
    // If the queue is empty and the goal state has not been found, return None
    None
}

fn get_invetrsion_count(linear_state: [u8; 9]) -> u16{
    let mut invetrsion_count = 0;
    for i in 0..9 {
        for j in (i+1)..9 {
            if linear_state[i] > 0 && linear_state[j] > 0 && linear_state[i] > linear_state[j] {
                invetrsion_count += 1;
            }
        }
    }
    return invetrsion_count;
}

fn is_solvable(state: [[u8; 3]; 3]) -> bool{
    let mut linear_state: [u8; 9] = [0; 9];
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
pub fn random_state() -> [[u8; 3]; 3]{
    let mut state:[[u8; 3]; 3];
    loop {
        let mut list: Vec<u8> = (0..=8).collect();
        list.shuffle(&mut Pcg64::from_entropy());
        state = [list[0..3].try_into().expect("Something went Wrong!"), list[3..6].try_into().expect("Something went Wrong!"), list[6..9].try_into().expect("Something went Wrong!")];
        if is_solvable(state) {
            break;
        }
    }
    return state;
}
