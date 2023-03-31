mod games;
use games::puzzle8;
use games::puzzle15;
use std::time::Instant;

#[test]
fn test8(){
    let start_state: [[u8; 3]; 3] = puzzle8::random_state();
   for _ in 0..25 {
        if let Some(path) = puzzle8::a_star_search(start_state) {
            println!("path length = {}", path.len());
        } else {
            println!("Goal state not found.");
        }
    } 
}

fn main() {
    let start_state: [[u8; 3]; 3] = puzzle8::random_state();
    if let Some(path) = puzzle8::a_star_search(start_state) {
        println!("path length = {}", path.len());
    } else {
        println!("Goal state not found.");
    }

    let start_state: [[u8; 3]; 3] = [[1, 2, 3], [4, 0, 5], [7, 8, 6]];
    if let Some(path) = puzzle8::a_star_search(start_state) {
        println!("path length = {}", path.len());
    } else {
        println!("Goal state not found.");
    }


    let start_state: [[u8; 4]; 4] = puzzle15::n_random_moves_from_goal(100);
    let now = Instant::now();
    if let Some(path) = puzzle15::a_star_search(start_state) {
        println!("path length = {}", path.len());
    } else {
        println!("Goal state not found.");
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
