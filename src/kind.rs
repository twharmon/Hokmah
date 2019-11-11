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
            Kind::Pawn => 15563246598181575127,
            Kind::Knight => 2562286319949714761,
            Kind::Bishop => 6486798529299960966,
            Kind::Rook => 3036314411627625419,
            Kind::Queen => 4748185868227808731,
            Kind::King => 5369441690474522948,
        }
    }
}