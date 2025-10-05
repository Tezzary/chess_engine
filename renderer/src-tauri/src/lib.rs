// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use chess_game;
use chess_game::{Piece, Team};
use serde::Serialize;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::State;

#[derive(Serialize)]
struct BoardSlot{
    piece: String,
    team: String,
}

#[tauri::command]
fn initial_setup(state: State<Arc<Mutex<chess_game::Board>>>) {
    let mut board = state.lock().unwrap();
    board.setup();
}
#[tauri::command]
fn get_board(state: State<Arc<Mutex<chess_game::Board>>>) -> Vec<BoardSlot>{ 
    let board = state.lock().unwrap();
    let mut parsed_board: Vec<BoardSlot> = Vec::new();
    for row in &board.tiles {
        for piece in row {
            match piece {
                Piece::King(Team::White) => {
                    parsed_board.push(BoardSlot {
                        piece: String::from("King"),
                        team: String::from("White"),
                    });
                },
                Piece::Queen(Team::White) => {
                    parsed_board.push(BoardSlot {
                        piece: String::from("Queen"),
                        team: String::from("White"),
                    });
                },
                Piece::Rook(Team::White) => {
                    parsed_board.push(BoardSlot {
                        piece: String::from("Rook"),
                        team: String::from("White"),
                    });
                }, Piece::Bishop(Team::White) => { 
                    parsed_board.push(BoardSlot { 
                        piece: String::from("Bishop"), 
                        team: String::from("White"), 
                    }); 
                }, 
                Piece::Knight(Team::White) => { 
                    parsed_board.push(BoardSlot { 
                        piece: String::from("Knight"), 
                        team: String::from("White"),
                    });
                },
                Piece::Pawn(Team::White) => {
                    parsed_board.push(BoardSlot {
                        piece: String::from("Pawn"),
                        team: String::from("White"),
                    });
                },
                Piece::King(Team::Black) => {
                    parsed_board.push(BoardSlot {
                        piece: String::from("King"),
                        team: String::from("Black"),
                    });
                },
                Piece::Queen(Team::Black) => {
                    parsed_board.push(BoardSlot {
                        piece: String::from("Queen"),
                        team: String::from("Black"),
                    });
                },
                Piece::Rook(Team::Black) => {
                    parsed_board.push(BoardSlot {
                        piece: String::from("Rook"),
                        team: String::from("Black"),
                    });
                },
                Piece::Bishop(Team::Black) => {
                    parsed_board.push(BoardSlot {
                        piece: String::from("Bishop"),
                        team: String::from("Black"),
                    });
                },
                Piece::Knight(Team::Black) => {
                    parsed_board.push(BoardSlot {
                        piece: String::from("Knight"),
                        team: String::from("Black"),
                    });
                },
                Piece::Pawn(Team::Black) => {
                    parsed_board.push(BoardSlot {
                        piece: String::from("Pawn"), 
                        team: String::from("Black"), 
                    });
                },
                _ => {
                    parsed_board.push(BoardSlot {
                        piece: String::from("Empty"),
                        team: String::from("None"),
                    })
                }, 
            }
        }
    }
    return parsed_board;
}

#[tauri::command]
fn move_piece(state: State<Arc<Mutex<chess_game::Board>>>, start_x: usize, start_y: usize, end_x: usize, end_y: usize) {
    if start_x == end_x && start_y == end_y {
        return;
    }
    let mut board = state.lock().unwrap();
    let game_moves = board.get_game_moves(start_x, start_y);
    for game_move in game_moves {
        if game_move.end_x != end_x {
            continue;
        }
        if game_move.end_y != end_y {
            continue;
        }
        //valid move requested
        board.make_move(game_move);
        return;
    }
    println!("moved piece");
    println!("{}", board.to_string())
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let board = Arc::new(Mutex::new(chess_game::Board::new()));
    tauri::Builder::default()
        .manage(board)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![initial_setup, get_board, move_piece])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
