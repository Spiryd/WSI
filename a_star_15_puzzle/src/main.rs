mod games;
use games::puzzle8;
use games::puzzle15;

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

    let start_state: [[u8; 4]; 4] = puzzle15::random_state();
    if let Some(path) = puzzle15::a_star_search(start_state) {
        println!("path length = {}", path.len());
    } else {
        println!("Goal state not found.");
    }
}
