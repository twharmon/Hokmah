use crate::cache::Cache;
use crate::color::Color;
use crate::game::Game;
use crate::kind::Kind;
use crate::piece::Piece;
use crate::position::Position;
use crate::traits::BestFirstSort;
use std::ops::BitXor;
use std::sync::{Arc, Mutex};

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
            "{}{}{}",
            self.origin,
            self.destination,
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
        if let Some(t) = self.target {
            match g.board[self.destination.1][self.destination.0] {
                None => {
                    let position = Position(self.origin.1, self.destination.0);
                    hash = hash.bitxor(position.hash(&t));
                }
                Some(p) => hash = hash.bitxor(self.destination.hash(&p)),
            };
        }
        let new_piece = match self.promotion {
            Some(p) => p,
            None => self.piece,
        };
        hash = hash.bitxor(self.destination.hash(&new_piece));

        if self.piece.kind == Kind::King && self.origin.0 == 4 {
            let rook = Piece {
                color: g.turn,
                kind: Kind::Rook,
            };
            if self.destination.0 == 2 {
                let position = Position(0, self.origin.1);
                hash = hash.bitxor(position.hash(&rook));
                let position = Position(3, self.origin.1);
                hash = hash.bitxor(position.hash(&rook));
            } else if self.destination.0 == 6 {
                let position = Position(7, self.origin.1);
                hash = hash.bitxor(position.hash(&rook));
                let position = Position(5, self.origin.1);
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
                        "{}x{}{}",
                        match self.origin.0 {
                            0 => "a",
                            1 => "b",
                            2 => "c",
                            3 => "d",
                            4 => "e",
                            5 => "f",
                            6 => "g",
                            _ => "h",
                        },
                        self.destination,
                        promotion_str
                    ),
                    None => format!(
                        "{}{}",
                        self.destination, promotion_str
                    ),
                }
            }
            _ => {
                if self.piece.kind == Kind::King
                    && (self.origin.0 as i8
                        - self.destination.0 as i8)
                        .abs()
                        == 2
                {
                    match self.destination.0 {
                        2 => return format!("O-O-O"),
                        _ => return format!("O-O"),
                    };
                }
                match self.target {
                    Some(_) => format!(
                        "{}x{}",
                        self.piece.to_letter(),
                        self.destination,
                    ),
                    None => format!(
                        "{}{}",
                        self.piece.to_letter(),
                        self.destination,
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
                if ply.origin.0 != self.origin.0 {
                    let (san_a, san_b) = naive_san.split_at(1);
                    return format!("{}{}{}", san_a, match self.origin.0 {
                        0 => "a",
                        1 => "b",
                        2 => "c",
                        3 => "d",
                        4 => "e",
                        5 => "f",
                        6 => "g",
                        _ => "h",
                    }, san_b);
                } else if ply.origin.1 != self.origin.1 {
                    let (san_a, san_b) = naive_san.split_at(1);
                    return format!("{}{}{}", san_a, self.origin.1 + 1, san_b);
                } else {
                    let (san_a, san_b) = naive_san.split_at(1);
                    return format!("{}{}{}", san_a, self.origin, san_b);
                }
            }
        }

        naive_san
    }
}

impl BestFirstSort for Vec<Ply> {
    fn best_first_sort(&mut self, depth: u8, g: &Game, color: Color, cache: &Arc<Mutex<Cache>>) {
        if depth > 2 {
            let c = cache.lock().unwrap();
            self.sort_by_cached_key(|a| {
                let key = a.hash(g).bitxor(color.hash());
                match c.get(key) {
                    Some(e) => e,
                    None => match a.promotion {
                        Some(p) => 0 - p.naive_value(),
                        None => match a.target {
                            Some(t) => 0 - t.naive_value(),
                            None => 0,
                        },
                    },
                }
            });
        } else {
            self.sort_unstable_by_key(|a| match a.promotion {
                Some(p) => 0 - p.naive_value(),
                None => match a.target {
                    Some(t) => 0 - t.naive_value(),
                    None => 0,
                },
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BestFirstSort;
    use crate::cache::Cache;
    use crate::color::Color;
    use crate::engine::search;
    use crate::kind::Kind;
    use crate::params::Params;
    use crate::piece::Piece;
    use crate::ply::Ply;
    use crate::position::Position;
    use crate::tests::make_game;
    use test::Bencher;

    const MID_GAME_PGN: &'static str = "1. e4 e5 2. Nf3 Nc6 3. Nc3 Nf6 4. Bb5 a6 5. Bxc6 dxc6 6. d4 exd4 7. Nxd4 Bc5 8. Be3 O-O 9. Qd3 Bg4 10. f3 Bh5 11. g4 Bg6 12. O-O-O b5 13. Nf5 Bxe3+ 14. Qxe3 Qc8";

    #[test]
    fn it_hashes_correctly() {
        let mut g = make_game("e4 e5 Nf3 Nc6 Nc3 Nf6 Bb5 a6 Bxc6 dxc6 d4 exd4");

        let ply = Ply {
            origin: Position(5, 2),
            destination: Position(3, 3),
            piece: Piece {
                kind: Kind::Knight,
                color: Color::White,
            },
            target: Some(Piece {
                kind: Kind::Pawn,
                color: Color::Black,
            }),
            promotion: None,
        };

        let test_hash = ply.hash(&g);
        g.do_ply(ply, false);
        assert_eq!(g.hash, test_hash);
    }

    #[bench]
    fn bench_best_first_sort_simple(b: &mut Bencher) {
        let params = Params::get();
        let mut g = make_game(MID_GAME_PGN);
        let cache = Cache::new();
        let future_cache = Cache::new();
        search(g.clone(), params, 4, 0, &cache, &future_cache);
        let mut plies = g.get_valid_plies();
        b.iter(|| {
            plies.best_first_sort(1, &g, Color::White, &future_cache);
        });
    }

    #[bench]
    fn bench_best_first_sort_cache(b: &mut Bencher) {
        let params = Params::get();
        let mut g = make_game(MID_GAME_PGN);
        let cache = Cache::new();
        let future_cache = Cache::new();
        search(g.clone(), params, 4, 0, &cache, &future_cache);
        let mut plies = g.get_valid_plies();
        b.iter(|| {
            plies.best_first_sort(4, &g, Color::White, &future_cache);
        });
    }

    #[bench]
    fn bench_best_first_sort_empty_cache(b: &mut Bencher) {
        let mut g = make_game(MID_GAME_PGN);
        let future_cache = Cache::new();
        let mut plies = g.get_valid_plies();
        b.iter(|| {
            plies.best_first_sort(4, &g, Color::White, &future_cache);
        });
    }
}
