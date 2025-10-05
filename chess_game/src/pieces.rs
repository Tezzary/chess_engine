
use crate::types;
pub use types::Piece;


pub fn is_valid_tile(team: &types::Team, board: &types::Board, x: usize, y: usize) -> bool {
    match &board.tiles[y][x] {
        Piece::Blank => true,
        Piece::King(piece_team)
        | Piece::Queen(piece_team)
        | Piece::Rook(piece_team)
        | Piece::Bishop(piece_team)
        | Piece::Knight(piece_team)
        | Piece::Pawn(piece_team) => team != piece_team,
    }
}

impl Piece {
    pub fn get_valid_moves(&self, board: &types::Board, start_x: i64, start_y: i64) -> Vec<types::GameMove> {
        let mut valid_moves = Vec::<types::GameMove>::new();
        match self {
            Piece::King(team) => {
               //also need to account for not allowing to move into check
               for y in -1..=1 {
                    for x in -1..=1 {
                        if y == 0 && x == 0 { //dont allow move to same tile
                            continue;
                        }
                        is_valid_tile(team, &board, x as usize, y as usize);
                        valid_moves.push(types::GameMove{
                            start_x: start_x as usize,
                            start_y: start_y as usize,
                            end_x: (start_x + x) as usize,
                            end_y: (start_y + y) as usize,
                        })
                    }
               }
            }
            _ => {

            }
        }
        valid_moves
    }
}
