use chess_game;

fn main() {
    let mut board = chess_game::Board::new();
    board.setup();
    println!("{}", board.to_string());
}
