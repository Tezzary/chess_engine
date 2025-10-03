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
fn get_board(state: State<Arc<Mutex<chess_game::Board>>>) { 
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
,
                Piece::Rook(Team::White) => {
                    parsed_board.push(BoardSlot {
                        piece: String::from("Rook"),
                        team: String::from("White"),
                    });
                }, , Piece::Bishop(Team::White) => { 
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
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let board = Arc::new(Mutex::new(chess_game::Board::new()));
    tauri::Builder::default()
        .manage(board)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_board])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
