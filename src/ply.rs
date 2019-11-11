use crate::file::File;
use crate::kind::Kind;
use crate::piece::Piece;
use crate::game::Game;
use crate::position::Position;
use crate::traits::BestFirstSort;
use std::ops::BitXor;
use std::sync::{Arc, Mutex};
use crate::cache::Cache;

#[derive(Copy, Clone, PartialEq)]
pub struct Ply {
    pub piece: Piece,
    pub origin: Position,
    pub destination: Position,
    pub target: Option<Piece>,
    pub promotion: Option<Piece>,
}

impl Ply {
    pub fn uci(&self) -> String {
        format!(
            "{}{}{}{}{}",
            self.origin.file,
            self.origin.rank,
            self.destination.file,
            self.destination.rank,
            {
                match self.promotion {
                    Some(p) => p.to_letter().to_lowercase(),
                    None => "".to_string(),
                }
            }
        )
    }

    pub fn hash(&self, g: &Game) -> u64 {
        let mut hash = g.hash;
        let destination_rank_usize: usize = self.destination.rank.into();
        let destination_file_usize: usize = self.destination.file.into();
        if let Some(t) = self.target {
            match g.board[destination_rank_usize][destination_file_usize] {
                None => {
                    let position = Position { rank: self.origin.rank, file: self.destination.file };
                    hash = hash.bitxor(position.hash(&t));
                },
                Some(p) => hash = hash.bitxor(self.destination.hash(&p)),
            };
        }
        let new_piece = match self.promotion {
            Some(p) => p,
            None => self.piece,
        };
        hash = hash.bitxor(self.destination.hash(&new_piece));

        if self.piece.kind == Kind::King && self.origin.file == File::E {
            let rook = Piece {
                color: g.turn,
                kind: Kind::Rook,
            };
            if self.destination.file == File::C {
                let position = Position { file: File::A, rank: self.origin.rank };
                hash = hash.bitxor(position.hash(&rook));
                let position = Position { file: File::D, rank: self.origin.rank };
                hash = hash.bitxor(position.hash(&rook));
            } else if self.destination.file == File::G {
                let position = Position { file: File::H, rank: self.origin.rank };
                hash = hash.bitxor(position.hash(&rook));
                let position = Position { file: File::F, rank: self.origin.rank };
                hash = hash.bitxor(position.hash(&rook));
            }
        }
        hash = hash.bitxor(self.origin.hash(&self.piece));
        hash = hash.bitxor(1);
        hash
    }

    fn naive_san(&self) -> String {
        match self.piece.kind {
            Kind::Pawn => {
                let promotion_str = match self.promotion {
                    Some(p) => format!("={}", p.to_letter()),
                    _ => "".to_string(),
                };
                match self.target {
                    Some(_) => format!(
                        "{}x{}{}{}",
                        self.origin.file,
                        self.destination.file,
                        self.destination.rank,
                        promotion_str
                    ),
                    None => format!(
                        "{}{}{}",
                        self.destination.file, self.destination.rank, promotion_str
                    ),
                }
            }
            _ => {
                if self.piece.kind == Kind::King
                    && (usize::from(self.origin.file) as i8
                        - usize::from(self.destination.file) as i8)
                        .abs()
                        == 2
                {
                    match self.destination.file {
                        File::C => return format!("O-O-O"),
                        _ => return format!("O-O"),
                    };
                }
                match self.target {
                    Some(_) => format!(
                        "{}x{}{}",
                        self.piece.to_letter(),
                        self.destination.file,
                        self.destination.rank
                    ),
                    None => format!(
                        "{}{}{}",
                        self.piece.to_letter(),
                        self.destination.file,
                        self.destination.rank
                    ),
                }
            }
        }
    }

    pub fn san(&self, others: &Vec<Ply>) -> String {
        let naive_san = self.naive_san();

        for ply in others {
            if ply.origin == self.origin && ply.destination == self.destination {
                continue;
            }
            if ply.naive_san() == naive_san {
                if ply.origin.file != self.origin.file {
                    let (san_a, san_b) = naive_san.split_at(1);
                    return format!("{}{}{}", san_a, self.origin.file, san_b);
                } else if ply.origin.rank != self.origin.rank {
                    let (san_a, san_b) = naive_san.split_at(1);
                    return format!("{}{}{}", san_a, self.origin.rank, san_b);
                } else {
                    let (san_a, san_b) = naive_san.split_at(1);
                    return format!("{}{}{}{}", san_a, self.origin.file, self.origin.rank, san_b);
                }
            }
        }

        naive_san
    }
}

impl BestFirstSort for Vec<Ply> {
    fn best_first_sort(&mut self, depth: u8, g: &Game, cache: &Arc<Mutex<Cache>>) {
        if depth > 2 {
            let c = cache.lock().unwrap();
            self.sort_unstable_by(|a, b| {
                let a_eval = match c.get(a.hash(g)) {
                    Some(e) => e,
                    None => match a.promotion {
                        Some(p) => p.naive_value(),
                        None => match a.target {
                            Some(t) => t.naive_value(),
                            None => 0,
                        },
                    },
                };

                let b_eval = match c.get(b.hash(g)) {
                    Some(e) => e,
                    None => match b.promotion {
                        Some(p) => p.naive_value(),
                        None => match b.target {
                            Some(t) => t.naive_value(),
                            None => 0,
                        },
                    },
                };

                b_eval.cmp(&a_eval)
            });
        } else {
            self.sort_unstable_by(|a, b| {
                let a_eval = match a.promotion {
                    Some(p) => p.naive_value(),
                    None => match a.target {
                        Some(t) => t.naive_value(),
                        None => 0,
                    },
                };

                let b_eval = match b.promotion {
                    Some(p) => p.naive_value(),
                    None => match b.target {
                        Some(t) => t.naive_value(),
                        None => 0,
                    },
                };

                b_eval.cmp(&a_eval)
            });
        }
    }
}
