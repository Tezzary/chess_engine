mod types;
use types::{Board, Team, GameMove};
mod pieces;
use pieces::Piece;

use std::result::Result;
impl Board {
    pub fn new() -> Board {
        Board {
            tiles: [[Piece::Blank; 8]; 8], //fills tiles with 0's
        }
    }
    pub fn setup(&mut self) {
        self.tiles[0][0] = Piece::Rook(Team::Black);
        self.tiles[0][1] = Piece::Knight(Team::Black);
        self.tiles[0][2] = Piece::Bishop(Team::Black);
        self.tiles[0][3] = Piece::Queen(Team::Black);
        self.tiles[0][4] = Piece::King(Team::Black);
        self.tiles[0][5] = Piece::Bishop(Team::Black);
        self.tiles[0][6] = Piece::Knight(Team::Black);
        self.tiles[0][7] = Piece::Rook(Team::Black);

        self.tiles[1][0] = Piece::Pawn(Team::Black);
        self.tiles[1][1] = Piece::Pawn(Team::Black);
        self.tiles[1][2] = Piece::Pawn(Team::Black);
        self.tiles[1][3] = Piece::Pawn(Team::Black);
        self.tiles[1][4] = Piece::Pawn(Team::Black);
        self.tiles[1][5] = Piece::Pawn(Team::Black);
        self.tiles[1][6] = Piece::Pawn(Team::Black);
        self.tiles[1][7] = Piece::Pawn(Team::Black);
        
        self.tiles[6][0] = Piece::Pawn(Team::White);
        self.tiles[6][1] = Piece::Pawn(Team::White);
        self.tiles[6][2] = Piece::Pawn(Team::White);
        self.tiles[6][3] = Piece::Pawn(Team::White);
        self.tiles[6][4] = Piece::Pawn(Team::White);
        self.tiles[6][5] = Piece::Pawn(Team::White);
        self.tiles[6][6] = Piece::Pawn(Team::White);
        self.tiles[6][7] = Piece::Pawn(Team::White);
        
        self.tiles[7][0] = Piece::Rook(Team::White);
        self.tiles[7][1] = Piece::Knight(Team::White);
        self.tiles[7][2] = Piece::Bishop(Team::White);
        self.tiles[7][3] = Piece::Queen(Team::White);
        self.tiles[7][4] = Piece::King(Team::White);
        self.tiles[7][5] = Piece::Bishop(Team::White);
        self.tiles[7][6] = Piece::Knight(Team::White);
        self.tiles[7][7] = Piece::Rook(Team::White);
    }
    pub fn get_piece(&self, x: usize, y: usize) -> Piece {
        self.tiles[y][x]
    }
    pub fn get_all_game_moves(&self) -> Vec<GameMove> {
        let valid_moves = Vec::new();
        valid_moves
    }
    pub fn get_piece_game_moves(&self, start_x: usize, start_y: usize) -> Result<Vec<GameMove>, ()> {
        let piece = self.tiles[start_y][start_x];
        if piece == Piece::Blank {
            return Err(()) 
        }
        Ok(piece.get_valid_moves(self, start_x as i64, start_y as i64))
    }
    //trusts that game_move is a valid move and already been checked
    pub fn make_move(&mut self, game_move: GameMove) -> bool {
        self.tiles[game_move.end_y][game_move.end_x] = self.tiles[game_move.start_y][game_move.start_x];
        self.tiles[game_move.start_y][game_move.start_x] = Piece::Blank;
        true
    }
    pub fn to_string(&self) -> String{
        let mut board_string: String = String::from("-----------------\n");
        for row in &self.tiles {
            board_string += "|";
            for column in row {
                let emoji = match column {
                    Piece::King(Team::White) => "♚",
                    Piece::Queen(Team::White) => "♛",
                    Piece::Rook(Team::White) => "♜",
                    Piece::Bishop(Team::White) => "♝",
                    Piece::Knight(Team::White) => "♞",
                    Piece::Pawn(Team::White) => "♟",
                    Piece::King(Team::Black) => "♔",
                    Piece::Queen(Team::Black) => "♕",
                    Piece::Rook(Team::Black) => "♖",
                    Piece::Bishop(Team::Black) => "♗",
                    Piece::Knight(Team::Black) => "♘",
                    Piece::Pawn(Team::Black) => "♙",
                    _ => " ",
                };
                board_string += emoji;
                board_string += "|";
            }
            board_string += "\n";
        }
        board_string += "-----------------";
        board_string
    }
}
