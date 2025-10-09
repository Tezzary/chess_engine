
use crate::types;
pub use types::{Piece, Team, GameMove, Board};

pub fn is_empty_tile(board: &Board, x: i64, y: i64) -> bool {
    if x < 0 || x > 7 || y < 0 || y > 7{
        return false;
    }
    let piece = board.get_piece(x as usize, y as usize);
    if piece == Piece::Blank {
        return true;
    }
    
    return false;

}

pub fn is_opposing_piece(team: &Team, board: &Board, x: i64, y: i64) -> bool {
    if x < 0 || x > 7 || y < 0 || y > 7{
        return false;
    }
    return !board.get_piece(x as usize, y as usize).on_team(team);
}

pub fn is_empty_or_opposing_tile(team: &Team, board: &Board, x: i64, y: i64) -> bool {
    return is_empty_tile(board, x, y) || is_opposing_piece(team, board, x, y);
}
impl Piece {
    pub fn on_team(&self, team: &Team) -> bool {
        match self{
            Piece::Blank => true,
            Piece::King(piece_team)
            | Piece::Queen(piece_team)
            | Piece::Rook(piece_team)
            | Piece::Bishop(piece_team)
            | Piece::Knight(piece_team)
            | Piece::Pawn(piece_team) => team == piece_team,
        }
    }
    pub fn get_valid_moves(&self, board: &Board, start_x: i64, start_y: i64) -> Vec<GameMove> {
        let mut valid_moves = Vec::<GameMove>::new();

        //NEED TO ENSURE TO RETURN NO MOVES IF CURRENT PIECE ISNT KING AND IN CHECK
        'outer: {
            match self {
                Piece::King(team) => {
                    //also need to account for not allowing to move into check
                    for y in -1..=1 {
                        for x in -1..=1 {
                            if y == 0 && x == 0 { //dont allow move to same tile
                                continue;
                            }
                            if !is_empty_or_opposing_tile(&team, &board, start_x+x, start_y+y) {
                                continue;
                            }
                            valid_moves.push(GameMove{
                                start_x: start_x as usize,
                                start_y: start_y as usize,
                                end_x: (start_x + x) as usize,
                                end_y: (start_y + y) as usize,
                            })
                        }
                    }
                }

                Piece::Queen(team) => {
                    for y in -1..=1 {
                        for x in -1..=1 {
                            if y == 0 && x == 0 { //dont allow move to same tile
                                continue;
                            }
                            for mult in 1..8 {
                                if !is_empty_or_opposing_tile(&team, &board, start_x+x*mult, start_y+y*mult){
                                    break;
                                }
                                valid_moves.push(GameMove{
                                    start_x: start_x as usize,
                                    start_y: start_y as usize,
                                    end_x: (start_x + x*mult) as usize,
                                    end_y: (start_y + y*mult) as usize,
                                });
                                if !is_empty_tile(&board, start_x+x*mult, start_y+y*mult) {
                                    break;
                                }
                            }
                        }
                    }
                }

                Piece::Rook(team) => {
                    for y in -1i64..=1 { //hard declaring types for abs
                        for x in -1i64..=1 {
                            if y == 0 && x == 0 || x.abs()-y.abs() == 0 { //dont allow move to same tile or horizonal
                                continue;
                            }
                            for mult in 1..8 {
                                if !is_empty_or_opposing_tile(&team, &board, start_x+x*mult, start_y+y*mult){
                                    break;
                                }
                                valid_moves.push(GameMove{
                                    start_x: start_x as usize,
                                    start_y: start_y as usize,
                                    end_x: (start_x + x*mult) as usize,
                                    end_y: (start_y + y*mult) as usize,
                                });
                                if !is_empty_tile(&board, start_x+x*mult, start_y+y*mult) {
                                    break;
                                }
                            }
                        }
                    }
                }

                Piece::Knight(team) => {
                    for y in -2i64..=2 {
                        for x in -2i64..=2 {
                            if x == 0 || y == 0 {
                                continue;
                            }

                            let abs_x = x.abs();
                            let abs_y = y.abs();

                            if (abs_x - abs_y).abs() == 1 {
                                if is_empty_or_opposing_tile(&team, &board, start_x+x, start_y+y){
                                    valid_moves.push(GameMove{
                                        start_x: start_x as usize,
                                        start_y: start_y as usize,
                                        end_x: (start_x + x) as usize,
                                        end_y: (start_y + y) as usize,
                                    });
                                }
                            }
                        }
                    }
                }



                Piece::Bishop(team) => {
                    for y in -1..=1 {
                        for x in -1..=1 {
                            if y == 0 || x == 0 { //dont allow move to same tile or horizonal
                                continue;
                            }
                            for mult in 1..8 {
                                if !is_empty_or_opposing_tile(&team, &board, start_x+x*mult, start_y+y*mult){
                                    break;
                                }
                                valid_moves.push(GameMove{
                                    start_x: start_x as usize,
                                    start_y: start_y as usize,
                                    end_x: (start_x + x*mult) as usize,
                                    end_y: (start_y + y*mult) as usize,
                                });
                                if !is_empty_tile(&board, start_x+x*mult, start_y+y*mult) {
                                    break;
                                }
                            }
                        }
                    }

                }

                Piece::Pawn(team) => {
                    if *team == Team::White {
                        //must do check diagonal takes
                        
                        if is_opposing_piece(team, &board, start_x-1, start_y-1) {
                            valid_moves.push(GameMove {
                                start_x: start_x as usize,
                                start_y: start_y as usize,
                                end_x: (start_x - 1) as usize,
                                end_y: (start_y - 1) as usize,
                            });
                        }
                        if is_opposing_piece(team, &board, start_x+1, start_y-1) {
                            valid_moves.push(GameMove {
                                start_x: start_x as usize,
                                start_y: start_y as usize,
                                end_x: (start_x + 1) as usize,
                                end_y: (start_y - 1) as usize,
                            });
                        }
                        //must do en_passant and somehow check if turn was just done as thats only
                        
                        //time enpassant allowed
                        if !is_empty_tile(&board, start_x, start_y-1) {
                            break 'outer; 
                        }
                        valid_moves.push(GameMove{ 
                            start_x: start_x as usize,
                            start_y: start_y as usize,
                            end_x: start_x as usize,
                            end_y: (start_y - 1) as usize,
                        });
                        if start_y != 6 { //second bottom rank
                            break 'outer; 
                        }
                        if !is_empty_tile(&board, start_x, start_y-2) {
                            break 'outer; 
                        }
                        valid_moves.push(GameMove{ 
                            start_x: start_x as usize,
                            start_y: start_y as usize,
                            end_x: start_x as usize,
                            end_y: (start_y - 2) as usize,
                        });
                    }
                    else {
                        //must do check diagonal takes
                        if is_opposing_piece(team, &board, start_x-1, start_y+1) {
                            valid_moves.push(GameMove {
                                start_x: start_x as usize,
                                start_y: start_y as usize,
                                end_x: (start_x - 1) as usize,
                                end_y: (start_y + 1) as usize,
                            });
                        }
                        if is_opposing_piece(team, &board, start_x+1, start_y+1) {
                            valid_moves.push(GameMove {
                                start_x: start_x as usize,
                                start_y: start_y as usize,
                                end_x: (start_x + 1) as usize,
                                end_y: (start_y + 1) as usize,
                            });
                        }
                        //must do en_passant

                        if !is_empty_tile(&board, start_x, start_y+1) {
                            break 'outer; 
                        }
                        valid_moves.push(GameMove{ 
                            start_x: start_x as usize,
                            start_y: start_y as usize,
                            end_x: start_x as usize,
                            end_y: (start_y + 1) as usize,
                        });
                        if start_y != 1 { //second bottom rank
                            break 'outer; 
                        }
                        if !is_empty_tile(&board, start_x, start_y+2) {
                            break 'outer; 
                        }
                        valid_moves.push(GameMove{ 
                            start_x: start_x as usize,
                            start_y: start_y as usize,
                            end_x: start_x as usize,
                            end_y: (start_y + 2) as usize,
                        });
                    }
                }
                _ => {

                }
            }
        }
        valid_moves
    }
}
