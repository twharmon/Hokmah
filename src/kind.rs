#[derive(Copy, Clone, PartialEq)]
pub enum Kind {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}


impl Kind {
    pub fn hash(&self) -> u64 {
        match self {
            Kind::Pawn => 7535471037945741149,
            Kind::Knight => 15571965623176004676,
            Kind::Bishop => 14147008775600472391,
            Kind::Rook => 41436723485343745,
            Kind::Queen => 9049761951147622386,
            Kind::King => 16718855679439274854,
        }
    }
}