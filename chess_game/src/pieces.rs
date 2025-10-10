
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

pub fn in_board(x: i64, y: i64) -> bool { 
    if x < 0 || x > 7 || y < 0 || y > 7{
        return false;
    }
    true
}
pub fn is_opposing_piece(team: &Team, board: &Board, x: i64, y: i64) -> bool {
    if !in_board(x, y) {
        return false
    }
    return !board.get_piece(x as usize, y as usize).on_team(team);
}

pub fn is_empty_or_opposing_tile(team: &Team, board: &Board, x: i64, y: i64) -> bool {
    return is_empty_tile(board, x, y) || is_opposing_piece(team, board, x, y);
}

pub fn tile_is_under_attack(current_team: &Team, board: &Board, x: i64, y: i64) -> bool {
    //check pawns
    println!("called");
    if *current_team == Team::White {
        if in_board(x + 1, y - 1) {
            let piece = board.get_piece(x as usize + 1, y as usize - 1);
            if piece == Piece::Pawn(Team::Black) {
                return true;
            }
        }
        if in_board(x - 1, y - 1) {
            let piece = board.get_piece(x as usize - 1, y as usize - 1);
            if piece == Piece::Pawn(Team::Black) {
                return true;
            }
        }
    }
    else {
        if in_board(x + 1, y + 1) {
            let piece = board.get_piece(x as usize + 1, y as usize + 1);
            if piece == Piece::Pawn(Team::White) {
                return true;
            }
        }
        if in_board(x - 1, y + 1) {
            let piece = board.get_piece(x as usize - 1, y as usize + 1);
            if piece == Piece::Pawn(Team::White) {
                return true;
            }
        }
    }

    //knight check
    for y_iter in -2i64..=2 {
        for x_iter in -2i64..=2 {
            if x_iter == 0 || y_iter == 0 {
                continue;
            }

            let abs_x = x_iter.abs();
            let abs_y = y_iter.abs();

            if (abs_x - abs_y).abs() == 1 {
                if in_board(x + x_iter, y + y_iter) {
                    let piece = board.get_piece((x + x_iter) as usize, (y + y_iter) as usize);
                    if piece == Piece::Knight(Team::Black) && *current_team == Team::White {
                        return true
                    }
                    if piece == Piece::Knight(Team::White) && *current_team == Team::Black {
                        return true
                    }
                }

            }
        }
    }

    //horizonal check (queen and rook)
    for y_iter in -1i64..=1 { //hard declaring types for abs
        for x_iter in -1i64..=1 {
            if y_iter == 0 && x_iter == 0 || x_iter.abs()-y_iter.abs() == 0 { 
                continue;
            }
            for mult in 1..8 {
                if in_board(x+x_iter*mult, y+y_iter*mult){
                    let piece = board.get_piece((x+x_iter*mult) as usize, (y+y_iter*mult) as usize);
                    
                    if piece == Piece::Queen(Team::Black) && *current_team == Team::White {
                        return true;
                    }
                    if piece == Piece::Rook(Team::Black) && *current_team == Team::White {
                        return true;
                    }

                    if piece == Piece::Queen(Team::White) && *current_team == Team::Black {
                        return true;
                    }
                    if piece == Piece::Rook(Team::White) && *current_team == Team::Black {
                        return true;
                    }

                    if piece != Piece::Blank && piece != Piece::King(*current_team) {
                        break;
                    }
                }
            }
        }
    }

    //diagonal check (queen and bishop)
    for y_iter in -1i64..=1 { //hard declaring types for abs
        for x_iter in -1i64..=1 {
            if y_iter == 0 || x_iter == 0 { //dont allow move to same tile or horizonal
                continue;
            }
            for mult in 1..8 {
                if in_board(x+x_iter*mult, y+y_iter*mult){
                    let piece = board.get_piece((x+x_iter*mult) as usize, (y+y_iter*mult) as usize);
                    
                    if piece == Piece::Queen(Team::Black) && *current_team == Team::White {
                        return true;
                    }
                    if piece == Piece::Bishop(Team::Black) && *current_team == Team::White {
                        return true;
                    }

                    if piece == Piece::Queen(Team::White) && *current_team == Team::Black {
                        return true;
                    }
                    if piece == Piece::Bishop(Team::White) && *current_team == Team::Black {
                        return true;
                    }

                    if piece != Piece::Blank && piece != Piece::King(*current_team) {
                        break;
                    }
                }
            }
        }
    }

    //king check
    for y_iter in -1..=1 {
        for x_iter in -1..=1 {
            if y_iter == 0 && x_iter == 0 { //dont allow move to same tile
                continue;
            }

            if in_board(x+x_iter, y+y_iter) {
                let piece = board.get_piece((x+x_iter) as usize, (y+y_iter) as usize);

                if piece == Piece::King(Team::White) && *current_team == Team::Black {
                    return true;
                }
                if piece == Piece::King(Team::Black) && *current_team == Team::White {
                    return true;
                }
            }
        }
    }
    false 
}

pub fn in_check(team: &Team, board: &Board) -> bool {
    if *team == Team::Black && tile_is_under_attack(team, board, board.king_black_x as i64, board.king_black_y as i64) {
        return true;
    }
    if *team == Team::White && tile_is_under_attack(team, board, board.king_white_x as i64, board.king_white_y as i64) {
        return true;
    }
    return false;
}

//function used to check for things such as will king be in check if this move happens
pub fn try_move(board: &Board, game_move: &GameMove) -> bool {
    let team = board.current_turn;
    let mut explorer_board: Board = board.clone();
    explorer_board.make_move(game_move.clone());
    return !in_check(&team, &explorer_board);
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

        if !self.on_team(&board.current_turn) {
            println!("exiting");
            return valid_moves;
        }

        let in_check: bool = in_check(&board.current_turn, board);

        println!("{}", in_check);

        //NEED TO ENSURE TO RETURN NO MOVES IF CURRENT PIECE ISNT KING AND IN CHECK
        'outer: {
            match self {
                Piece::King(team) => {
                    if board.castle_king_white && *team == Team::White {
                        if is_empty_tile(board, 5, 7) && is_empty_tile(board, 6, 7)  {
                            if !tile_is_under_attack(&Team::White, board, 4, 7) &&
                            !tile_is_under_attack(&Team::White, board, 5, 7) &&
                            !tile_is_under_attack(&Team::White, board, 6, 7) {
                                let game_move = GameMove{
                                    start_x: start_x as usize,
                                    start_y: start_y as usize,
                                    end_x: (start_x + 2) as usize,
                                    end_y: (start_y) as usize,
                                };
                                valid_moves.push(game_move);
                            }
                        }
                    }
                    if board.castle_king_black && *team == Team::Black{
                        if is_empty_tile(board, 5, 0) && is_empty_tile(board, 6, 0)  {
                            if !tile_is_under_attack(&Team::Black, board, 4, 0) &&
                            !tile_is_under_attack(&Team::Black, board, 5, 0) &&
                            !tile_is_under_attack(&Team::Black, board, 6, 0) {
                                let game_move = GameMove{
                                    start_x: start_x as usize,
                                    start_y: start_y as usize,
                                    end_x: (start_x + 2) as usize,
                                    end_y: (start_y) as usize,
                                };
                                valid_moves.push(game_move);
                            }
                        }
                    }
                    if board.castle_queen_white && *team == Team::White {
                        if is_empty_tile(board, 3, 7) && is_empty_tile(board, 2, 7) && is_empty_tile(board, 1, 7)  {
                            if !tile_is_under_attack(&Team::White, board, 4, 7) &&
                            !tile_is_under_attack(&Team::White, board, 3, 7) &&
                            !tile_is_under_attack(&Team::White, board, 2, 7) {
                                let game_move = GameMove{
                                    start_x: start_x as usize,
                                    start_y: start_y as usize,
                                    end_x: (start_x - 2) as usize,
                                    end_y: (start_y) as usize,
                                };
                                valid_moves.push(game_move);
                            }
                        }
                    }
                    if board.castle_queen_black && *team == Team::Black{
                        if is_empty_tile(board, 3, 0) && is_empty_tile(board, 2, 0) && is_empty_tile(board, 1, 0)  {
                            if !tile_is_under_attack(&Team::Black, board, 4, 0) &&
                            !tile_is_under_attack(&Team::Black, board, 3, 0) &&
                            !tile_is_under_attack(&Team::Black, board, 2, 0) {
                                let game_move = GameMove{
                                    start_x: start_x as usize,
                                    start_y: start_y as usize,
                                    end_x: (start_x - 2) as usize,
                                    end_y: (start_y) as usize,
                                };
                                valid_moves.push(game_move);
                            }
                        }
                    }

                    //also need to account for not allowing to move into check
                    for y in -1..=1 {
                        for x in -1..=1 {
                            if y == 0 && x == 0 { //dont allow move to same tile
                                continue;
                            }
                            if !is_empty_or_opposing_tile(&team, &board, start_x+x, start_y+y) {
                                continue;
                            }
                            if tile_is_under_attack(team, board, start_x+x, start_y+y) {
                                continue;
                            }
                            let game_move = GameMove{
                                start_x: start_x as usize,
                                start_y: start_y as usize,
                                end_x: (start_x + x) as usize,
                                end_y: (start_y + y) as usize,
                            };
                            let safe = try_move(board, &game_move); 
                            if safe {
                                valid_moves.push(game_move);
                            }
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
                                let game_move = GameMove{
                                    start_x: start_x as usize,
                                    start_y: start_y as usize,
                                    end_x: (start_x + x*mult) as usize,
                                    end_y: (start_y + y*mult) as usize,
                                };
                                let safe = try_move(board, &game_move); 
                                if safe {
                                    valid_moves.push(game_move);
                                }
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
                            if y == 0 && x == 0 || x.abs()-y.abs() == 0 { 
                                continue;
                            }
                            for mult in 1..8 {
                                if !is_empty_or_opposing_tile(&team, &board, start_x+x*mult, start_y+y*mult){
                                    break;
                                }
                                let game_move = GameMove{
                                    start_x: start_x as usize,
                                    start_y: start_y as usize,
                                    end_x: (start_x + x*mult) as usize,
                                    end_y: (start_y + y*mult) as usize,
                                };
                                let safe = try_move(board, &game_move); 
                                if safe {
                                    valid_moves.push(game_move);
                                }
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
                                    let game_move = GameMove{
                                        start_x: start_x as usize,
                                        start_y: start_y as usize,
                                        end_x: (start_x + x) as usize,
                                        end_y: (start_y + y) as usize,
                                    };
                                    let safe = try_move(board, &game_move); 
                                    if safe {
                                        valid_moves.push(game_move);
                                    }
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
                                let game_move = GameMove{
                                    start_x: start_x as usize,
                                    start_y: start_y as usize,
                                    end_x: (start_x + x*mult) as usize,
                                    end_y: (start_y + y*mult) as usize,
                                };
                                let safe = try_move(board, &game_move); 
                                if safe {
                                    valid_moves.push(game_move);
                                }
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
                            let game_move = GameMove {
                                start_x: start_x as usize,
                                start_y: start_y as usize,
                                end_x: (start_x - 1) as usize,
                                end_y: (start_y - 1) as usize,
                            };
                            let safe = try_move(board, &game_move); 
                            if safe {
                                valid_moves.push(game_move);
                            }
                        }
                        if is_opposing_piece(team, &board, start_x+1, start_y-1) {
                            let game_move = GameMove {
                                start_x: start_x as usize,
                                start_y: start_y as usize,
                                end_x: (start_x + 1) as usize,
                                end_y: (start_y - 1) as usize,
                            };
                            let safe = try_move(board, &game_move); 
                            if safe {
                                valid_moves.push(game_move);
                            }
                        }
                        //must do en_passant and somehow check if turn was just done as thats only
                        
                        //time enpassant allowed
                        if !is_empty_tile(&board, start_x, start_y-1) {
                            break 'outer; 
                        }
                        let game_move = GameMove{ 
                            start_x: start_x as usize,
                            start_y: start_y as usize,
                            end_x: start_x as usize,
                            end_y: (start_y - 1) as usize,
                        };
                        let safe = try_move(board, &game_move); 
                        if safe {
                            valid_moves.push(game_move);
                        }
                        if start_y != 6 { //second bottom rank
                            break 'outer; 
                        }
                        if !is_empty_tile(&board, start_x, start_y-2) {
                            break 'outer; 
                        }
                        let game_move = GameMove{ 
                            start_x: start_x as usize,
                            start_y: start_y as usize,
                            end_x: start_x as usize,
                            end_y: (start_y - 2) as usize,
                        };
                        let safe = try_move(board, &game_move); 
                        if safe {
                            valid_moves.push(game_move);
                        }
                    }
                    else {
                        //must do check diagonal takes
                        if is_opposing_piece(team, &board, start_x-1, start_y+1) {
                            let game_move = GameMove {
                                start_x: start_x as usize,
                                start_y: start_y as usize,
                                end_x: (start_x - 1) as usize,
                                end_y: (start_y + 1) as usize,
                            };
                            let safe = try_move(board, &game_move); 
                            if safe {
                                valid_moves.push(game_move);
                            }
                        }
                        if is_opposing_piece(team, &board, start_x+1, start_y+1) {
                            let game_move = GameMove {
                                start_x: start_x as usize,
                                start_y: start_y as usize,
                                end_x: (start_x + 1) as usize,
                                end_y: (start_y + 1) as usize,
                            };
                            let safe = try_move(board, &game_move); 
                            if safe {
                                valid_moves.push(game_move);
                            }
                        }
                        //must do en_passant

                        if !is_empty_tile(&board, start_x, start_y+1) {
                            break 'outer; 
                        }
                        let game_move = GameMove{ 
                            start_x: start_x as usize,
                            start_y: start_y as usize,
                            end_x: start_x as usize,
                            end_y: (start_y + 1) as usize,
                        };
                        let safe = try_move(board, &game_move); 
                        if safe {
                            valid_moves.push(game_move);
                        }
                        if start_y != 1 { //second bottom rank
                            break 'outer; 
                        }
                        if !is_empty_tile(&board, start_x, start_y+2) {
                            break 'outer; 
                        }
                        let game_move = GameMove{ 
                            start_x: start_x as usize,
                            start_y: start_y as usize,
                            end_x: start_x as usize,
                            end_y: (start_y + 2) as usize,
                        };
                        let safe = try_move(board, &game_move); 
                        if safe {
                            valid_moves.push(game_move);
                        }
                    }
                }
                _ => {

                }
            }
        }
        valid_moves
    }
}
