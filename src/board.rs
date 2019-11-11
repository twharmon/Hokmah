use crate::color::Color;
use crate::kind::Kind;
use crate::piece::Piece;

pub type Board = [[Option<Piece>; 8]; 8];

pub fn eq(a: &Board, b: &Board) -> bool {
    for i in 0..8 {
        for j in 0..8 {
            if a[i][j] != b[i][j] {
                return false;
            }
        }
    }
    true
}

pub fn new() -> Board {
    [
        [
            Some(Piece {
                color: Color::White,
                kind: Kind::Rook,
            }),
            Some(Piece {
                color: Color::White,
                kind: Kind::Knight,
            }),
            Some(Piece {
                color: Color::White,
                kind: Kind::Bishop,
            }),
            Some(Piece {
                color: Color::White,
                kind: Kind::Queen,
            }),
            Some(Piece {
                color: Color::White,
                kind: Kind::King,
            }),
            Some(Piece {
                color: Color::White,
                kind: Kind::Bishop,
            }),
            Some(Piece {
                color: Color::White,
                kind: Kind::Knight,
            }),
            Some(Piece {
                color: Color::White,
                kind: Kind::Rook,
            }),
        ],
        [
            Some(Piece {
                color: Color::White,
                kind: Kind::Pawn,
            }),
            Some(Piece {
                color: Color::White,
                kind: Kind::Pawn,
            }),
            Some(Piece {
                color: Color::White,
                kind: Kind::Pawn,
            }),
            Some(Piece {
                color: Color::White,
                kind: Kind::Pawn,
            }),
            Some(Piece {
                color: Color::White,
                kind: Kind::Pawn,
            }),
            Some(Piece {
                color: Color::White,
                kind: Kind::Pawn,
            }),
            Some(Piece {
                color: Color::White,
                kind: Kind::Pawn,
            }),
            Some(Piece {
                color: Color::White,
                kind: Kind::Pawn,
            }),
        ],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [
            Some(Piece {
                color: Color::Black,
                kind: Kind::Pawn,
            }),
            Some(Piece {
                color: Color::Black,
                kind: Kind::Pawn,
            }),
            Some(Piece {
                color: Color::Black,
                kind: Kind::Pawn,
            }),
            Some(Piece {
                color: Color::Black,
                kind: Kind::Pawn,
            }),
            Some(Piece {
                color: Color::Black,
                kind: Kind::Pawn,
            }),
            Some(Piece {
                color: Color::Black,
                kind: Kind::Pawn,
            }),
            Some(Piece {
                color: Color::Black,
                kind: Kind::Pawn,
            }),
            Some(Piece {
                color: Color::Black,
                kind: Kind::Pawn,
            }),
        ],
        [
            Some(Piece {
                color: Color::Black,
                kind: Kind::Rook,
            }),
            Some(Piece {
                color: Color::Black,
                kind: Kind::Knight,
            }),
            Some(Piece {
                color: Color::Black,
                kind: Kind::Bishop,
            }),
            Some(Piece {
                color: Color::Black,
                kind: Kind::Queen,
            }),
            Some(Piece {
                color: Color::Black,
                kind: Kind::King,
            }),
            Some(Piece {
                color: Color::Black,
                kind: Kind::Bishop,
            }),
            Some(Piece {
                color: Color::Black,
                kind: Kind::Knight,
            }),
            Some(Piece {
                color: Color::Black,
                kind: Kind::Rook,
            }),
        ],
    ]
}
