pub struct Board {
    tiles: [[Piece; 8]; 8],
}

#[derive(Copy, Clone)]
pub enum Team {
    White,
    Black,
}

#[derive(Copy, Clone)]
pub enum Piece {
    King(Team),
    Queen(Team),
    Rook(Team),
    Bishop(Team),
    Knight(Team),
    Pawn(Team),
    Blank,
}

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
        self.tiles[7][3] = Piece::King(Team::White);
        self.tiles[7][4] = Piece::Queen(Team::White);
        self.tiles[7][5] = Piece::Bishop(Team::White);
        self.tiles[7][6] = Piece::Knight(Team::White);
        self.tiles[7][7] = Piece::Rook(Team::White);
    }
}
