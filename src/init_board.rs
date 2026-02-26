use crate::structs::Piece;
use crate::structs::Type;

pub fn init_board() -> [[Piece; 8]; 8] {
    let mut board: [[Piece; 8]; 8] = [[Piece{
        cords: [0, 0], type_p: Type::None, moved: false}; 8]; 8];

    for i in 0..8 {
        for j in 0..8 {
            board[i][j].cords = [i, j];
        }
    }
    board[0][0].type_p = Type::Wrook;
    board[0][7].type_p = Type::Wrook;
    board[0][1].type_p = Type::Wknight;
    board[0][6].type_p = Type::Wknight;
    board[0][2].type_p = Type::Wbishop;
    board[0][5].type_p = Type::Wbishop;
    board[0][4].type_p = Type::Wqueen;
    board[0][3].type_p = Type::Wking;

    board[7][0].type_p = Type::Brook;
    board[7][7].type_p = Type::Brook;
    board[7][6].type_p = Type::Bknight;
    board[7][1].type_p = Type::Bknight;
    board[7][2].type_p = Type::Bbishop;
    board[7][5].type_p = Type::Bbishop;
    board[7][4].type_p = Type::Bqueen;
    board[7][3].type_p = Type::Bking;

    for i in 0..8 {
        board[1][i].type_p = Type::Wpawn;
        board[6][i].type_p = Type::Bpawn;
    }

    return board;
}
