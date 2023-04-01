mod games;
use games::puzzle8;
use games::puzzle15;
use games::walking_distance;
use std::time::Instant;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use std::mem;

fn main() {
    let items = vec!["8 Puzzle form shuffle", "15 puzzle from n moves", "15 Puzzle form shuffle", "walking distance", "State Size", "Exit"];
    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .expect("failed");

        match selection.unwrap() + 1 {
            1 => puzzle8_from_random(),
            2 => puzzle15_from_n_moves(),
            3 => puzzle15_from_random(),
            4 => println!("{:?}", walking_distance::simulation()),
            5 => println!("{:?}", mem::size_of::<puzzle15::State>()),
            _ => break
        }
    }
}

fn puzzle8_from_random() {
    let start_state: [[u8; 3]; 3] = puzzle8::random_state();
    if let Some(path) = puzzle8::a_star_search(start_state) {
        println!("path length = {}", path.len());
    } else {
        println!("Goal state not found.");
    }
    println!("\n");
}

fn puzzle15_from_random() {
    let start_state: [[u8; 4]; 4] = puzzle15::random_state();
    let now = Instant::now();
    if let Some(path) = puzzle15::a_star_search(start_state) {
        for node in &path{
            println!("{:?}", node);
        }
        println!("path length = {}", path.len());
    } else {
        println!("Goal state not found.");
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed); 
    println!("\n");
}

fn puzzle15_from_n_moves() {
    let start_state: [[u8; 4]; 4] = puzzle15::n_random_moves_from_goal(60);
    let now = Instant::now();
    if let Some(path) = puzzle15::a_star_search(start_state) {
        println!("path length = {}", path.len());
    } else {
        println!("Goal state not found.");
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("\n");
}
