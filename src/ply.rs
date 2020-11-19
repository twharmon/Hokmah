use crate::file::File;
use crate::kind::Kind;
use crate::piece::Piece;
use crate::position::Position;
use crate::traits::BestFirstSort;

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
    fn best_first_sort(&mut self) {
         self.sort_unstable_by_key(|a| {
            match a.promotion {
                Some(p) => 0 - p.naive_value(),
                None => match a.target {
                    Some(t) => 0 - t.naive_value(),
                    None => 0,
                },
            }
        });
    }
}


#[cfg(test)]
mod tests {
    use crate::tests::make_game;
    use crate::params::Params;
    use crate::engine::search;
    use super::BestFirstSort;
    use test::Bencher;

    const MID_GAME_PGN: &'static str = "1. e4 e5 2. Nf3 Nc6 3. Nc3 Nf6 4. Bb5 a6 5. Bxc6 dxc6 6. d4 exd4 7. Nxd4 Bc5 8. Be3 O-O 9. Qd3 Bg4 10. f3 Bh5 11. g4 Bg6 12. O-O-O b5 13. Nf5 Bxe3+ 14. Qxe3 Qc8";

    #[bench]
    fn bench_best_first_sort_simple(b: &mut Bencher) {
        let params = Params::get();
        let mut g = make_game(MID_GAME_PGN);
        search(g.clone(), params, 4, 0);
        let mut plies = g.get_valid_plies();
        b.iter(|| {
            plies.best_first_sort();
        });
    }
}