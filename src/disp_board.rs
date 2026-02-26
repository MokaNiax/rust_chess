use crate::structs::Piece;
use crate::structs::Type;

pub fn disp_board(board: [[Piece; 8]; 8]) {
    for i in 0..8 {
        for j in 0..8 {
            if board[i][j].type_p == Type::Bpawn || board[i][j].type_p == Type::Wpawn {
                print!("P");
            }
            if board[i][j].type_p == Type::Brook || board[i][j].type_p == Type::Wrook {
                print!("R");
            }
            if board[i][j].type_p == Type::Bknight || board[i][j].type_p == Type::Wknight {
                print!("H");
            }
            if board[i][j].type_p == Type::Bbishop || board[i][j].type_p == Type::Wbishop {
                print!("B");
            }
            if board[i][j].type_p == Type::Bqueen || board[i][j].type_p == Type::Wqueen {
                print!("Q");
            }
            if board[i][j].type_p == Type::Bking || board[i][j].type_p == Type::Wking {
                print!("K");
            }
            if board[i][j].type_p == Type::None {
                print!(" ");
            }
            print!(" ");
        }
        print!("\n");
    }
}
