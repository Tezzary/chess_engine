

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Team {
    White,
    Black,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Piece {
    King(Team),
    Queen(Team),
    Rook(Team),
    Bishop(Team),
    Knight(Team),
    Pawn(Team),
    Blank,
}

#[derive(Clone)]
pub struct Board {
    pub tiles: [[Piece; 8]; 8],
    pub current_turn: Team,

    pub last_moved_x: usize,
    pub last_moved_y: usize,

    pub castle_queen_white: bool,
    pub castle_king_white: bool,
    pub castle_queen_black: bool,
    pub castle_king_black: bool,

    pub king_white_x: usize,
    pub king_white_y: usize,
    pub king_black_x: usize,
    pub king_black_y: usize,
}

#[derive(Clone)]
pub struct GameMove {
    //piece: Piece,
    pub start_x: usize,
    pub start_y: usize,
    pub end_x: usize,
    pub end_y: usize,
}
