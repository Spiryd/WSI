mod games;
use games::puzzle8;
use games::puzzle15;
use games::walking_distance;
use std::time::Instant;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};

fn main() {
    let items = vec!["8 Puzzle form shuffle", "15 puzzle from n moves", "15 Puzzle form shuffle", "IDA* from shuffle", "walking distance", "Exit"];
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
            4 => ida(),
            5 => println!("{:?}", walking_distance::simulation()),
            _ => break
        }
    }
}

fn puzzle8_from_random() {
    let start_state: [[u8; 3]; 3] = puzzle8::random_state();
    if let Some(path) = puzzle8::a_star_search(start_state) {
        for node in &path{
            for row in node{
                println!("{:?}", row);
            }
            println!("");
        }
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
            for row in node{
                println!("{:?}", row);
            }
            println!("");
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
    let start_state: [[u8; 4]; 4] = puzzle15::n_random_moves_from_goal(50);
    let now = Instant::now();
    if let Some(path) = puzzle15::a_star_search(start_state) {
        for node in &path{
            for row in node{
                println!("{:?}", row);
            }
            println!("");
        }
        println!("path length = {}", path.len());
    } else {
        println!("Goal state not found.");
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("\n");
}

fn ida() {
    let start_state: [[u8; 4]; 4] = puzzle15::random_state();
    let now = Instant::now();
    if let Some(path) = puzzle15::ida_star_search(start_state) {
        for node in &path{
            for row in node{
                println!("{:?}", row);
            }
            println!("");
        }
        println!("path length = {}", path.len());
    } else {
        println!("Goal state not found.");
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed); 
    println!("\n");
}
