#[derive(Copy,Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Wpawn,
    Wrook,
    Wknight,
    Wbishop,
    Wqueen,
    Wking,
    Bpawn,
    Brook,
    Bknight,
    Bbishop,
    Bqueen,
    Bking,
    None,
}

#[derive(Copy,Clone, Debug)]
pub struct Piece {
    pub cords: [usize; 2],
    pub type_p: Type,
    pub moved: bool,
}
