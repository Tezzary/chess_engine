

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


pub struct Board {
    pub tiles: [[Piece; 8]; 8],
}


pub struct GameMove {
    //piece: Piece,
    pub start_x: usize,
    pub start_y: usize,
    pub end_x: usize,
    pub end_y: usize,
}
