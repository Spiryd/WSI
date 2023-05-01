use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

mod tictactoe;
use tictactoe::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        panic!("Wrong args supplied");
    }
    let ip = args[1].clone();
    let port = args[2].clone();
    let player_id = args[3].clone();
    let player: Player = match player_id.parse().unwrap() {
        1 => Player::Player1,
        2 => Player::Player2,
        _ => panic!("Incorrect player id")
    };
    let depth: u8 = args[4].clone().parse().unwrap();
    dbg!(&ip, &port, &player, &depth);
    match TcpStream::connect(format!("{ip}:{port}").as_str()) {
        Ok(mut stream) => {
            println!("Successfully connected to server");
            let mut data;
            let mut game_board: GameState = GameState::new();
            loop {
                data = [9; 3];
                let mut msg_in: &str = "";
                match stream.read(&mut data) {
                    Ok(_) => {
                        msg_in = from_utf8(&data).unwrap();
                        println!("{}", msg_in)
                    },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
                if data[2] == 9 {
                    println!("{}, {}", data[0], data[1]);
                    game_board.make_move((data[0] as usize - 49 , data[1] as usize - 49));
                    game_board.print_board();
                    let mv =  move_with_minimax(&game_board, depth);
                    game_board.make_move(mv);
                    game_board.print_board();
                    stream.write_all(format!("{}{}", mv.0 + 1, mv.1 + 1).as_bytes()).unwrap();
                } else if msg_in == "600" {
                    let mv =  move_with_minimax(&game_board, depth);
                    game_board.make_move(mv);
                    game_board.print_board();
                    stream.write_all(format!("{}{}", mv.0 + 1, mv.1 + 1).as_bytes()).unwrap();
                } else if msg_in == "700" {
                    stream.write_all(player_id.as_bytes()).unwrap();
                } else {
                    break;
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
