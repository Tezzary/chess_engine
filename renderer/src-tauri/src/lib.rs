// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use chess_game;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::State;


#[tauri::command]
fn get_board(state: State<Arc<Mutex<chess_game::Board>>>) -> Vec<[chess_game::Piece; 8]>{
    let board = state.lock().unwrap();
    return board.tiles.clone().to_vec();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let board = Arc::new(Mutex::new(chess_game::Board::new()));
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_board])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
