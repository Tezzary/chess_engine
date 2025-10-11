use chess_game::types::{Board};
pub enum PlayerType {
    Human,
    Minimax,
}

struct Game {
    white: PlayerType,
    black: PlayerType,
    board: Board,
}

impl Game {
    fn new(white_player: PlayerType, black_player: PlayerType) -> Game {
        let mut game = Game {
            white: whitePlayer,
            black: blackPlayer,
            board: Board::new(),
        };

        game.board.setup();
        
        return game;
    }
    fn reset_game(&mut self) {
        self.board = Board::new();
    }
}
