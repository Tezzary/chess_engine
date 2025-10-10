pub mod types;
pub use types::{Board, Piece, Team, GameMove};
mod pieces;

impl Board {

    pub fn new() -> Board {
        Board {
            tiles: [[Piece::Blank; 8]; 8], //fills tiles with 0's
            current_turn: Team::White,

            last_moved_x: 0,
            last_moved_y: 0,

            castle_queen_white: true,
            castle_king_white: true,
            castle_queen_black: true,
            castle_king_black: true,

            king_white_x: 0,
            king_white_y: 0,
            king_black_x: 0,
            king_black_y: 0,

            can_en_passant: false,
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

        self.king_black_x = 4;
        self.king_black_y = 0;

        self.king_white_x = 4;
        self.king_white_y = 7;
    }

    fn switch_turn(&mut self) {
        if self.current_turn == Team::White {
            self.current_turn = Team::Black;
        }
        else {
            self.current_turn = Team::White;
        }
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Piece {
        self.tiles[y][x]
    }

    //not implemented yet
    pub fn get_all_game_moves(&self) -> Vec<GameMove> {
        let mut valid_moves = Vec::new();
        for y in 0..7 {
            for x in 0..7 {
                let piece = self.get_piece(x, y);
                if piece.on_team(&self.current_turn) {
                    let piece_valid_moves = piece.get_valid_moves(&self, x as i64, y as i64);
                    valid_moves.extend_from_slice(&piece_valid_moves);
                }
            }
        }
        valid_moves
    }

    //trusts that game_move is a valid move and already been checked
    pub fn make_move(&mut self, game_move: GameMove) -> bool {
        
        self.can_en_passant = false;

        let piece = self.tiles[game_move.start_y][game_move.start_x];

        if piece == Piece::King(Team::White) {
            //castling king side
            if game_move.end_x as i64 - game_move.start_x as i64 == 2 {
                self.tiles[game_move.end_y][game_move.end_x - 1] = Piece::Rook(Team::White);
                self.tiles[game_move.end_y][7] = Piece::Blank;
            }
            if game_move.end_x as i64 - game_move.start_x as i64 == -2 {
                self.tiles[game_move.end_y][game_move.end_x + 1] = Piece::Rook(Team::White);
                self.tiles[game_move.end_y][0] = Piece::Blank;
            }
            self.castle_queen_white = false;
            self.castle_king_white = false;
            self.king_white_x = game_move.end_x;
            self.king_white_y = game_move.end_y;
        }
        if piece == Piece::King(Team::Black) {
            if game_move.end_x as i64 - game_move.start_x as i64 == 2 {
                self.tiles[game_move.end_y][game_move.end_x - 1] = Piece::Rook(Team::Black);
                self.tiles[game_move.end_y][7] = Piece::Blank;
            }
            if game_move.end_x as i64 - game_move.start_x as i64 == -2 {
                self.tiles[game_move.end_y][game_move.end_x + 1] = Piece::Rook(Team::Black);
                self.tiles[game_move.end_y][0] = Piece::Blank;
            }
            self.castle_queen_black = false;
            self.castle_king_black = false;
            self.king_black_x = game_move.end_x;
            self.king_black_y = game_move.end_y;
        }

        if piece == Piece::Rook(Team::White) && game_move.start_x == 0 && game_move.start_y == 7 {
            self.castle_queen_white = false;
        }
        if piece == Piece::Rook(Team::White) && game_move.start_x == 7 && game_move.start_y == 7 {
            self.castle_king_white = false;
        }

        if piece == Piece::Rook(Team::Black) && game_move.start_x == 0 && game_move.start_y == 0 {
            self.castle_queen_black = false;
        }
        if piece == Piece::Rook(Team::Black) && game_move.start_x == 7 && game_move.start_y == 0 {
            self.castle_king_black = false;
        }

        if piece == Piece::Pawn(Team::White) {
            if game_move.start_x != game_move.end_x && self.get_piece(game_move.end_x, game_move.end_y) == Piece::Blank {
                self.tiles[game_move.end_y+1][game_move.end_x] = Piece::Blank;
            }
        }
        if piece == Piece::Pawn(Team::Black) {
            if game_move.start_x != game_move.end_x && self.get_piece(game_move.end_x, game_move.end_y) == Piece::Blank {
                self.tiles[game_move.end_y-1][game_move.end_x] = Piece::Blank;
            }
        }

        if matches!(piece, Piece::Pawn(_)) {

            if (game_move.end_y as i64 - game_move.start_y as i64).abs() == 2 {
                self.can_en_passant = true;
            }
        }

        self.tiles[game_move.end_y][game_move.end_x] = piece;
        self.tiles[game_move.start_y][game_move.start_x] = Piece::Blank;
        
        if game_move.promote_to != Piece::Blank {
            self.tiles[game_move.end_y][game_move.end_x] = game_move.promote_to;
        }

        self.last_moved_x = game_move.end_x;
        self.last_moved_y = game_move.end_y;

        self.switch_turn();

        false
    }

    pub fn is_checkmate(&self) -> bool{
        if self.current_turn == Team::White {
            let king_piece = self.get_piece(self.king_white_x, self.king_white_y);
            let in_check = pieces::in_check(&self.current_turn, &self); 
            if in_check { //important to check this first otherwise will lead to infinite
                          //recursion, king dancing around board
                let king_has_no_valid_moves = king_piece.get_valid_moves(&self, self.king_white_x as i64, self.king_white_y as i64).len() == 0; 
                if  in_check && king_has_no_valid_moves && self.get_all_game_moves().len() == 0 {
                    return true;
                }
            }
        }
        else {
            let king_piece = self.get_piece(self.king_black_x, self.king_black_y);
            let in_check = pieces::in_check(&self.current_turn, &self);
            if in_check {
                let king_has_no_valid_moves = king_piece.get_valid_moves(&self, self.king_black_x as i64, self.king_black_y as i64).len() == 0;
                if  in_check && king_has_no_valid_moves && self.get_all_game_moves().len() == 0 {
                    return true;
                }
            }
        }
        false
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
