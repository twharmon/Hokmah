use crate::direction::Direction;
use crate::file::File;
use crate::rank::Rank;

pub static ALL_POSITIONS: [Position; 64] = [
    Position {
        file: File::E,
        rank: Rank::Eight,
    },
    Position {
        file: File::E,
        rank: Rank::One,
    },
    Position {
        file: File::A,
        rank: Rank::Eight,
    },
    Position {
        file: File::B,
        rank: Rank::Eight,
    },
    Position {
        file: File::C,
        rank: Rank::Eight,
    },
    Position {
        file: File::D,
        rank: Rank::Eight,
    },
    Position {
        file: File::F,
        rank: Rank::Eight,
    },
    Position {
        file: File::G,
        rank: Rank::Eight,
    },
    Position {
        file: File::H,
        rank: Rank::Eight,
    },
    Position {
        file: File::A,
        rank: Rank::One,
    },
    Position {
        file: File::B,
        rank: Rank::One,
    },
    Position {
        file: File::C,
        rank: Rank::One,
    },
    Position {
        file: File::D,
        rank: Rank::One,
    },
    Position {
        file: File::F,
        rank: Rank::One,
    },
    Position {
        file: File::G,
        rank: Rank::One,
    },
    Position {
        file: File::H,
        rank: Rank::One,
    },
    Position {
        file: File::A,
        rank: Rank::Two,
    },
    Position {
        file: File::B,
        rank: Rank::Two,
    },
    Position {
        file: File::C,
        rank: Rank::Two,
    },
    Position {
        file: File::D,
        rank: Rank::Two,
    },
    Position {
        file: File::E,
        rank: Rank::Two,
    },
    Position {
        file: File::F,
        rank: Rank::Two,
    },
    Position {
        file: File::G,
        rank: Rank::Two,
    },
    Position {
        file: File::H,
        rank: Rank::Two,
    },
    Position {
        file: File::A,
        rank: Rank::Seven,
    },
    Position {
        file: File::B,
        rank: Rank::Seven,
    },
    Position {
        file: File::C,
        rank: Rank::Seven,
    },
    Position {
        file: File::D,
        rank: Rank::Seven,
    },
    Position {
        file: File::E,
        rank: Rank::Seven,
    },
    Position {
        file: File::F,
        rank: Rank::Seven,
    },
    Position {
        file: File::G,
        rank: Rank::Seven,
    },
    Position {
        file: File::H,
        rank: Rank::Seven,
    },
    Position {
        file: File::A,
        rank: Rank::Three,
    },
    Position {
        file: File::B,
        rank: Rank::Three,
    },
    Position {
        file: File::C,
        rank: Rank::Three,
    },
    Position {
        file: File::D,
        rank: Rank::Three,
    },
    Position {
        file: File::E,
        rank: Rank::Three,
    },
    Position {
        file: File::F,
        rank: Rank::Three,
    },
    Position {
        file: File::G,
        rank: Rank::Three,
    },
    Position {
        file: File::H,
        rank: Rank::Three,
    },
    Position {
        file: File::A,
        rank: Rank::Four,
    },
    Position {
        file: File::B,
        rank: Rank::Four,
    },
    Position {
        file: File::C,
        rank: Rank::Four,
    },
    Position {
        file: File::D,
        rank: Rank::Four,
    },
    Position {
        file: File::E,
        rank: Rank::Four,
    },
    Position {
        file: File::F,
        rank: Rank::Four,
    },
    Position {
        file: File::G,
        rank: Rank::Four,
    },
    Position {
        file: File::H,
        rank: Rank::Four,
    },
    Position {
        file: File::A,
        rank: Rank::Five,
    },
    Position {
        file: File::B,
        rank: Rank::Five,
    },
    Position {
        file: File::C,
        rank: Rank::Five,
    },
    Position {
        file: File::D,
        rank: Rank::Five,
    },
    Position {
        file: File::E,
        rank: Rank::Five,
    },
    Position {
        file: File::F,
        rank: Rank::Five,
    },
    Position {
        file: File::G,
        rank: Rank::Five,
    },
    Position {
        file: File::H,
        rank: Rank::Five,
    },
    Position {
        file: File::A,
        rank: Rank::Six,
    },
    Position {
        file: File::B,
        rank: Rank::Six,
    },
    Position {
        file: File::C,
        rank: Rank::Six,
    },
    Position {
        file: File::D,
        rank: Rank::Six,
    },
    Position {
        file: File::E,
        rank: Rank::Six,
    },
    Position {
        file: File::F,
        rank: Rank::Six,
    },
    Position {
        file: File::G,
        rank: Rank::Six,
    },
    Position {
        file: File::H,
        rank: Rank::Six,
    },
];

pub static CENTER_POSITIONS: [Position; 4] = [
    Position {
        file: File::D,
        rank: Rank::Four,
    },
    Position {
        file: File::D,
        rank: Rank::Five,
    },
    Position {
        file: File::E,
        rank: Rank::Four,
    },
    Position {
        file: File::E,
        rank: Rank::Five,
    },
];

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
    pub file: File,
    pub rank: Rank,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
    }
}

impl Position {
    pub fn step(&self, file_dir: Direction, rank_dir: Direction) -> Option<Self> {
        let mut file = self.file;
        let mut rank = self.rank;
        file = match file.step(file_dir) {
            Some(f) => f,
            None => return None,
        };

        rank = match rank.step(rank_dir) {
            Some(r) => r,
            None => return None,
        };

        Some(Position { file, rank })
    }

    pub fn step_n(&self) -> Option<Self> {
        self.step(Direction::None, Direction::Inc)
    }

    pub fn step_nn(&self) -> Option<Self> {
        match self.step_n() {
            Some(p) => p.step_n(),
            None => None,
        }
    }

    pub fn step_nne(&self) -> Option<Self> {
        match self.step_ne() {
            Some(p) => p.step_n(),
            None => None,
        }
    }

    pub fn step_ne(&self) -> Option<Self> {
        self.step(Direction::Inc, Direction::Inc)
    }

    pub fn step_nene(&self) -> Option<Self> {
        match self.step_ne() {
            Some(p) => p.step_ne(),
            None => None,
        }
    }

    pub fn step_ene(&self) -> Option<Self> {
        match self.step_ne() {
            Some(p) => p.step_e(),
            None => None,
        }
    }

    pub fn step_e(&self) -> Option<Self> {
        self.step(Direction::Inc, Direction::None)
    }

    pub fn step_ee(&self) -> Option<Self> {
        match self.step_e() {
            Some(p) => p.step_e(),
            None => None,
        }
    }

    pub fn step_ese(&self) -> Option<Self> {
        match self.step_se() {
            Some(p) => p.step_e(),
            None => None,
        }
    }

    pub fn step_se(&self) -> Option<Self> {
        self.step(Direction::Inc, Direction::Dec)
    }

    pub fn step_sese(&self) -> Option<Self> {
        match self.step_se() {
            Some(p) => p.step_se(),
            None => None,
        }
    }

    pub fn step_sse(&self) -> Option<Self> {
        match self.step_se() {
            Some(p) => p.step_s(),
            None => None,
        }
    }

    pub fn step_s(&self) -> Option<Self> {
        self.step(Direction::None, Direction::Dec)
    }

    pub fn step_ss(&self) -> Option<Self> {
        match self.step_s() {
            Some(p) => p.step_s(),
            None => None,
        }
    }

    pub fn step_ssw(&self) -> Option<Self> {
        match self.step_sw() {
            Some(p) => p.step_s(),
            None => None,
        }
    }

    pub fn step_sw(&self) -> Option<Self> {
        self.step(Direction::Dec, Direction::Dec)
    }

    pub fn step_swsw(&self) -> Option<Self> {
        match self.step_sw() {
            Some(p) => p.step_sw(),
            None => None,
        }
    }

    pub fn step_wsw(&self) -> Option<Self> {
        match self.step_sw() {
            Some(p) => p.step_w(),
            None => None,
        }
    }

    pub fn step_w(&self) -> Option<Self> {
        self.step(Direction::Dec, Direction::None)
    }

    pub fn step_ww(&self) -> Option<Self> {
        match self.step_w() {
            Some(p) => p.step_w(),
            None => None,
        }
    }

    pub fn step_wnw(&self) -> Option<Self> {
        match self.step_nw() {
            Some(p) => p.step_w(),
            None => None,
        }
    }

    pub fn step_nw(&self) -> Option<Self> {
        self.step(Direction::Dec, Direction::Inc)
    }

    pub fn step_nwnw(&self) -> Option<Self> {
        match self.step_nw() {
            Some(p) => p.step_nw(),
            None => None,
        }
    }

    pub fn step_nnw(&self) -> Option<Self> {
        match self.step_nw() {
            Some(p) => p.step_n(),
            None => None,
        }
    }

    pub fn is_black(&self) -> bool {
        (usize::from(self.file) + usize::from(self.rank)) % 2 == 0
    }

    // pub fn hash(&self, piece: &Piece) -> u64 {
    //     match (self, piece) {
    //         (Position { file: File::A, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Rook }) => 7533913771606016,
    //         (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Rook }) => 2334725846859776,
    //         (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Rook }) => 4456973242204160,
    //         (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Rook }) => 2080970400333824,
    //         (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Rook }) => 1428174361591808,
    //         (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Rook }) => 476289295187968,
    //         (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Rook }) => 7153013854568448,
    //         (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Rook }) => 3834333009805312,
    //         (Position { file: File::B, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Rook }) => 8953003774050304,
    //         (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Rook }) => 7439954825183232,
    //         (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Rook }) => 8258101018886144,
    //         (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Rook }) => 7776187304640512,
    //         (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Rook }) => 2512153284182016,
    //         (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Rook }) => 6092488297152512,
    //         (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Rook }) => 7074548004421632,
    //         (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Rook }) => 5291501443612672,
    //         (Position { file: File::C, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Rook }) => 1805342390353920,
    //         (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Rook }) => 5930320910090240,
    //         (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Rook }) => 2689048814551040,
    //         (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Rook }) => 2344140289343488,
    //         (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Rook }) => 2539910451953664,
    //         (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Rook }) => 3028060603416576,
    //         (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Rook }) => 2758009384075264,
    //         (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Rook }) => 5961781713305600,
    //         (Position { file: File::D, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Rook }) => 465320955346944,
    //         (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Rook }) => 3150683301740544,
    //         (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Rook }) => 309790366498816,
    //         (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Rook }) => 1116802788622336,
    //         (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Rook }) => 6980434544033792,
    //         (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Rook }) => 6323440455778304,
    //         (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Rook }) => 5825437569646592,
    //         (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Rook }) => 5885848622989312,
    //         (Position { file: File::E, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Rook }) => 2575392500088832,
    //         (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Rook }) => 7264452598562816,
    //         (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Rook }) => 8027219962101760,
    //         (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Rook }) => 5912217927024640,
    //         (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Rook }) => 1712545763164160,
    //         (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Rook }) => 1692139329159168,
    //         (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Rook }) => 8343736895406080,
    //         (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Rook }) => 4238225635803136,
    //         (Position { file: File::F, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Rook }) => 3378463713001472,
    //         (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Rook }) => 8315233131036672,
    //         (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Rook }) => 7939415441670144,
    //         (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Rook }) => 4171939268526080,
    //         (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Rook }) => 6591693390348288,
    //         (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Rook }) => 1081013826486272,
    //         (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Rook }) => 154733392166912,
    //         (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Rook }) => 5566166609166336,
    //         (Position { file: File::G, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Rook }) => 4176408244912128,
    //         (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Rook }) => 8673952098418688,
    //         (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Rook }) => 7445456898490368,
    //         (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Rook }) => 1107864389156864,
    //         (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Rook }) => 5640579455123456,
    //         (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Rook }) => 4664253491445760,
    //         (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Rook }) => 5378098799312896,
    //         (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Rook }) => 3560295261798400,
    //         (Position { file: File::H, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Rook }) => 3980159866634240,
    //         (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Rook }) => 2333330016043008,
    //         (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Rook }) => 6077176302534656,
    //         (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Rook }) => 4926099016384512,
    //         (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Rook }) => 4774830117224448,
    //         (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Rook }) => 6753953976942592,
    //         (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Rook }) => 6468908976439296,
    //         (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Rook }) => 5751585634254848,
    //         (Position { file: File::A, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Bishop }) => 3807509670264832,
    //         (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Bishop }) => 3902392384880640,
    //         (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Bishop }) => 1890139905196032,
    //         (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Bishop }) => 6001094022922240,
    //         (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Bishop }) => 7730225154621440,
    //         (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Bishop }) => 2995716897112064,
    //         (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Bishop }) => 7976288312623104,
    //         (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Bishop }) => 2091828583071744,
    //         (Position { file: File::B, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Bishop }) => 8658417040228352,
    //         (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Bishop }) => 7836545394409472,
    //         (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Bishop }) => 4361446630621184,
    //         (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Bishop }) => 8711456033865728,
    //         (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Bishop }) => 572699608875008,
    //         (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Bishop }) => 645188758274048,
    //         (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Bishop }) => 8276823330783232,
    //         (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Bishop }) => 3128347208974336,
    //         (Position { file: File::C, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Bishop }) => 7596688889872384,
    //         (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Bishop }) => 5981420883279872,
    //         (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Bishop }) => 1184587719901184,
    //         (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Bishop }) => 5754869270970368,
    //         (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Bishop }) => 347189167521792,
    //         (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Bishop }) => 8221722371162112,
    //         (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Bishop }) => 855656487714816,
    //         (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Bishop }) => 5288851654311936,
    //         (Position { file: File::D, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Bishop }) => 3771353333235712,
    //         (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Bishop }) => 6439349793062912,
    //         (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Bishop }) => 4518074417938432,
    //         (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Bishop }) => 2828376427462656,
    //         (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Bishop }) => 167171290300416,
    //         (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Bishop }) => 2630493384212480,
    //         (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Bishop }) => 1019453659676672,
    //         (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Bishop }) => 2613884976365568,
    //         (Position { file: File::E, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Bishop }) => 3035001895518208,
    //         (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Bishop }) => 2370738258444288,
    //         (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Bishop }) => 4734139540439040,
    //         (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Bishop }) => 2937876870332416,
    //         (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Bishop }) => 1330312833400832,
    //         (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Bishop }) => 1284689964826624,
    //         (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Bishop }) => 6406357425061888,
    //         (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Bishop }) => 3713968430383104,
    //         (Position { file: File::F, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Bishop }) => 2772889092751360,
    //         (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Bishop }) => 7879889573117952,
    //         (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Bishop }) => 797400713134080,
    //         (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Bishop }) => 3869614056407040,
    //         (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Bishop }) => 8553858410217472,
    //         (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Bishop }) => 7250046084972544,
    //         (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Bishop }) => 3201531186774016,
    //         (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Bishop }) => 5566703926771712,
    //         (Position { file: File::G, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Bishop }) => 3733164004474880,
    //         (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Bishop }) => 4293792156876800,
    //         (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Bishop }) => 3637029671796736,
    //         (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Bishop }) => 6917839476752384,
    //         (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Bishop }) => 5264673987362816,
    //         (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Bishop }) => 4117920365936640,
    //         (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Bishop }) => 13196670795776,
    //         (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Bishop }) => 359498933862400,
    //         (Position { file: File::H, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Bishop }) => 1224833197146112,
    //         (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Bishop }) => 4025661616291840,
    //         (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Bishop }) => 7945672932196352,
    //         (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Bishop }) => 1070361288376320,
    //         (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Bishop }) => 3967024665460736,
    //         (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Bishop }) => 4843913386917888,
    //         (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Bishop }) => 7156347709685760,
    //         (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Bishop }) => 617007485550592,
    //         (Position { file: File::A, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Knight }) => 7902381677215744,
    //         (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Knight }) => 6591966087217152,
    //         (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Knight }) => 7850516293877760,
    //         (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Knight }) => 3284920801689600,
    //         (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Knight }) => 6519367325974528,
    //         (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Knight }) => 7001517965967360,
    //         (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Knight }) => 2164740746903552,
    //         (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Knight }) => 5828230437142528,
    //         (Position { file: File::B, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Knight }) => 6682548939784192,
    //         (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Knight }) => 5949505845329920,
    //         (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Knight }) => 4236361252995072,
    //         (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Knight }) => 432706961801216,
    //         (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Knight }) => 3598306265530368,
    //         (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Knight }) => 3865595049148416,
    //         (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Knight }) => 4745102176026624,
    //         (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Knight }) => 5723283049676800,
    //         (Position { file: File::C, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Knight }) => 6379659050614784,
    //         (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Knight }) => 5760353549942784,
    //         (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Knight }) => 6054220144836608,
    //         (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Knight }) => 7030316933840896,
    //         (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Knight }) => 1947758678245376,
    //         (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Knight }) => 6980965136072704,
    //         (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Knight }) => 3545799145291776,
    //         (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Knight }) => 2762850690924544,
    //         (Position { file: File::D, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Knight }) => 8281970356256768,
    //         (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Knight }) => 4167549954555904,
    //         (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Knight }) => 8780449971175424,
    //         (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Knight }) => 1241370412974080,
    //         (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Knight }) => 5434404228825088,
    //         (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Knight }) => 2671536609689600,
    //         (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Knight }) => 4739029299888128,
    //         (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Knight }) => 6582154364452864,
    //         (Position { file: File::E, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Knight }) => 2099170546548736,
    //         (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Knight }) => 5850727670874112,
    //         (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Knight }) => 5368740164141056,
    //         (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Knight }) => 6459505099407360,
    //         (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Knight }) => 5770921180659712,
    //         (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Knight }) => 930405251284992,
    //         (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Knight }) => 236012422823936,
    //         (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Knight }) => 3870481216176128,
    //         (Position { file: File::F, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Knight }) => 455455864258560,
    //         (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Knight }) => 2131464594391040,
    //         (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Knight }) => 7061835079483392,
    //         (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Knight }) => 3085138187517952,
    //         (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Knight }) => 624734758764544,
    //         (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Knight }) => 627531881381888,
    //         (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Knight }) => 1145113659047936,
    //         (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Knight }) => 2800889897156608,
    //         (Position { file: File::G, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Knight }) => 212298601857024,
    //         (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Knight }) => 1751186579390464,
    //         (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Knight }) => 7256044956811264,
    //         (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Knight }) => 8333109472788480,
    //         (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Knight }) => 7789189537988608,
    //         (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Knight }) => 389543408697344,
    //         (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Knight }) => 8722119783350272,
    //         (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Knight }) => 2690609445863424,
    //         (Position { file: File::H, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Knight }) => 1340284214444032,
    //         (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Knight }) => 1485152226115584,
    //         (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Knight }) => 2811199559303168,
    //         (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Knight }) => 330563690954752,
    //         (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Knight }) => 1196787993935872,
    //         (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Knight }) => 8117729353531392,
    //         (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Knight }) => 8113990517391360,
    //         (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Knight }) => 5562363528871936,
    //         (Position { file: File::A, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Pawn }) => 1622537811263488,
    //         (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Pawn }) => 6962996045676544,
    //         (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Pawn }) => 2749001002647552,
    //         (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Pawn }) => 7396917044576256,
    //         (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Pawn }) => 4177575108673536,
    //         (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Pawn }) => 295921346674688,
    //         (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Pawn }) => 857609762504704,
    //         (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Pawn }) => 2165736552267776,
    //         (Position { file: File::B, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Pawn }) => 5605669401001984,
    //         (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Pawn }) => 3075899117273088,
    //         (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Pawn }) => 2464583946076160,
    //         (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Pawn }) => 8253112261279744,
    //         (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Pawn }) => 6021776614096896,
    //         (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Pawn }) => 6202039015047168,
    //         (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Pawn }) => 8515621906022400,
    //         (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Pawn }) => 4569788397387776,
    //         (Position { file: File::C, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Pawn }) => 1895499101634560,
    //         (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Pawn }) => 3920809062039552,
    //         (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Pawn }) => 6352913540579328,
    //         (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Pawn }) => 2634408376926208,
    //         (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Pawn }) => 4668675290300416,
    //         (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Pawn }) => 2607545990512640,
    //         (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Pawn }) => 8065781338210304,
    //         (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Pawn }) => 8727192362024960,
    //         (Position { file: File::D, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Pawn }) => 4818779362033664,
    //         (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Pawn }) => 5890579034537984,
    //         (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Pawn }) => 7825433869418496,
    //         (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Pawn }) => 6600749232422912,
    //         (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Pawn }) => 6173922653372416,
    //         (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Pawn }) => 5487051868733440,
    //         (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Pawn }) => 6279204387684352,
    //         (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Pawn }) => 8768085760671744,
    //         (Position { file: File::E, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Pawn }) => 1620018143952896,
    //         (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Pawn }) => 7626928534061056,
    //         (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Pawn }) => 6677986046640128,
    //         (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Pawn }) => 8006553594494976,
    //         (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Pawn }) => 7102742071869440,
    //         (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Pawn }) => 8634023721041920,
    //         (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Pawn }) => 7828758845194240,
    //         (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Pawn }) => 6557996804997120,
    //         (Position { file: File::F, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Pawn }) => 1460804897472512,
    //         (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Pawn }) => 6697189107564544,
    //         (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Pawn }) => 4546517236973568,
    //         (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Pawn }) => 2467690612523008,
    //         (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Pawn }) => 8687175132512256,
    //         (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Pawn }) => 215013237194752,
    //         (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Pawn }) => 1969351747960832,
    //         (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Pawn }) => 7534565048451072,
    //         (Position { file: File::G, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Pawn }) => 779857946476544,
    //         (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Pawn }) => 7389014812262400,
    //         (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Pawn }) => 7291753876946944,
    //         (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Pawn }) => 7286018751332352,
    //         (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Pawn }) => 7627020062162944,
    //         (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Pawn }) => 8372021991833600,
    //         (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Pawn }) => 1945758351753216,
    //         (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Pawn }) => 1490595006644224,
    //         (Position { file: File::H, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Pawn }) => 4417565558308864,
    //         (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Pawn }) => 3950975255576576,
    //         (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Pawn }) => 8875629357301760,
    //         (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Pawn }) => 8027211648991232,
    //         (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Pawn }) => 1862470834585600,
    //         (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Pawn }) => 2808834332557312,
    //         (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Pawn }) => 4681164514131968,
    //         (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Pawn }) => 4936453328994304,
    //         (Position { file: File::A, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Queen }) => 1076914624135168,
    //         (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Queen }) => 8284239141273600,
    //         (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Queen }) => 682566770229248,
    //         (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Queen }) => 3425369482854400,
    //         (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Queen }) => 8047132913172480,
    //         (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Queen }) => 938292128251904,
    //         (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Queen }) => 8199045921112064,
    //         (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Queen }) => 6240162065416192,
    //         (Position { file: File::B, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Queen }) => 3321051075313664,
    //         (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Queen }) => 5791273321496576,
    //         (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Queen }) => 1284806056869888,
    //         (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Queen }) => 3616822465134592,
    //         (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Queen }) => 3265368166498304,
    //         (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Queen }) => 1318075670462464,
    //         (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Queen }) => 1874240341016576,
    //         (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Queen }) => 6144163808542720,
    //         (Position { file: File::C, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Queen }) => 2579311261384704,
    //         (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Queen }) => 1943863453286400,
    //         (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Queen }) => 2884836018618368,
    //         (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Queen }) => 6623044478959616,
    //         (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Queen }) => 5490856503017472,
    //         (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Queen }) => 5764592913350656,
    //         (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Queen }) => 8438485249687552,
    //         (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Queen }) => 8893532592930816,
    //         (Position { file: File::D, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Queen }) => 3596491616681984,
    //         (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Queen }) => 5400691682050048,
    //         (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Queen }) => 4622727849705472,
    //         (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Queen }) => 7704081428643840,
    //         (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Queen }) => 7616456070004736,
    //         (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Queen }) => 8602839465066496,
    //         (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Queen }) => 2059032430903296,
    //         (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Queen }) => 7693456526802944,
    //         (Position { file: File::E, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Queen }) => 7679391419072512,
    //         (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Queen }) => 8273589339947008,
    //         (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Queen }) => 7315564917686272,
    //         (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Queen }) => 6561767412989952,
    //         (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Queen }) => 4826876985999360,
    //         (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Queen }) => 5183761167679488,
    //         (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Queen }) => 3523368229797888,
    //         (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Queen }) => 1769403909668864,
    //         (Position { file: File::F, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Queen }) => 2658434660958208,
    //         (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Queen }) => 3231329292386304,
    //         (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Queen }) => 1221006018478080,
    //         (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Queen }) => 4010156205539328,
    //         (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Queen }) => 4316796999958528,
    //         (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Queen }) => 6198690230304768,
    //         (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Queen }) => 1031611799830528,
    //         (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Queen }) => 5370088802746368,
    //         (Position { file: File::G, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Queen }) => 1522121784164352,
    //         (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Queen }) => 5042563104899072,
    //         (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Queen }) => 4604813339787264,
    //         (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Queen }) => 148981793423360,
    //         (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Queen }) => 8226428959064064,
    //         (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Queen }) => 5127527158775808,
    //         (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Queen }) => 5366926683406336,
    //         (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Queen }) => 7235904542343168,
    //         (Position { file: File::H, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Queen }) => 7129126917898240,
    //         (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Queen }) => 4711528120975360,
    //         (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Queen }) => 1310980067819520,
    //         (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Queen }) => 7838093528793088,
    //         (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Queen }) => 4062926361067520,
    //         (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Queen }) => 6037314887221248,
    //         (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Queen }) => 6607057807474688,
    //         (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Queen }) => 7480355722362880,
    //         (Position { file: File::A, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::King }) => 8450447809445888,
    //         (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::King }) => 7872702364778496,
    //         (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::King }) => 645779876216832,
    //         (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::King }) => 2145762817343488,
    //         (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::King }) => 5837013271969792,
    //         (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::King }) => 1756518030508032,
    //         (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::King }) => 7155587794075648,
    //         (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::King }) => 2974168375099392,
    //         (Position { file: File::B, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::King }) => 2662019176071168,
    //         (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::King }) => 3083780975755264,
    //         (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::King }) => 5126269769351168,
    //         (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::King }) => 5685034423943168,
    //         (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::King }) => 6447988092174336,
    //         (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::King }) => 6484807412350976,
    //         (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::King }) => 4876219413692416,
    //         (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::King }) => 904005343510528,
    //         (Position { file: File::C, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::King }) => 6887931914485760,
    //         (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::King }) => 4360487764492288,
    //         (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::King }) => 5304304372023296,
    //         (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::King }) => 7921402803912704,
    //         (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::King }) => 4357910960275456,
    //         (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::King }) => 6975147233771520,
    //         (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::King }) => 7843720177123328,
    //         (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::King }) => 6914799923363840,
    //         (Position { file: File::D, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::King }) => 5649524412383232,
    //         (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::King }) => 1862404132569088,
    //         (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::King }) => 5169926725697536,
    //         (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::King }) => 339672387551232,
    //         (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::King }) => 2222291593199616,
    //         (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::King }) => 5028409400885248,
    //         (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::King }) => 7897769691840512,
    //         (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::King }) => 3983902198726656,
    //         (Position { file: File::E, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::King }) => 8724487820279808,
    //         (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::King }) => 6799169857519616,
    //         (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::King }) => 8139733332918272,
    //         (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::King }) => 1232925987176448,
    //         (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::King }) => 5665833212706816,
    //         (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::King }) => 7632908986613760,
    //         (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::King }) => 7144472441782272,
    //         (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::King }) => 3725430567731200,
    //         (Position { file: File::F, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::King }) => 2849324406931456,
    //         (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::King }) => 6472661867692032,
    //         (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::King }) => 7340392110358528,
    //         (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::King }) => 1717181333110784,
    //         (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::King }) => 7178158757380096,
    //         (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::King }) => 7385740656771072,
    //         (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::King }) => 8609935577317376,
    //         (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::King }) => 6165364792623104,
    //         (Position { file: File::G, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::King }) => 8966430330454016,
    //         (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::King }) => 1490466430255104,
    //         (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::King }) => 3900860482453504,
    //         (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::King }) => 2336344187600896,
    //         (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::King }) => 8545593523699712,
    //         (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::King }) => 6959844540022784,
    //         (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::King }) => 4136342179020800,
    //         (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::King }) => 1393761712078848,
    //         (Position { file: File::H, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::King }) => 1278686047240192,
    //         (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::King }) => 7304385044414464,
    //         (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::King }) => 4629846449717248,
    //         (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::King }) => 7444071909949440,
    //         (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::King }) => 8519133987602432,
    //         (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::King }) => 7146724636229632,
    //         (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::King }) => 8652396926337024,
    //         (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::King }) => 2415009342685184,
    //         (Position { file: File::A, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Rook }) => 4635751392215040,
    //         (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Rook }) => 3046181380816896,
    //         (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Rook }) => 3907409099620352,
    //         (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Rook }) => 2586497127546880,
    //         (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Rook }) => 6339438510604288,
    //         (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Rook }) => 2464814873968640,
    //         (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Rook }) => 741615626878976,
    //         (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Rook }) => 3942807622385664,
    //         (Position { file: File::B, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Rook }) => 7250123683790848,
    //         (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Rook }) => 8887624370487296,
    //         (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Rook }) => 1717505194196992,
    //         (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Rook }) => 622379004854272,
    //         (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Rook }) => 7698271501811712,
    //         (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Rook }) => 1861744758620160,
    //         (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Rook }) => 4322258254299136,
    //         (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Rook }) => 8942614713729024,
    //         (Position { file: File::C, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Rook }) => 7755667037224960,
    //         (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Rook }) => 3098665140879360,
    //         (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Rook }) => 4376601246564352,
    //         (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Rook }) => 7354943704924160,
    //         (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Rook }) => 2320709980258304,
    //         (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Rook }) => 8804257694220288,
    //         (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Rook }) => 1527408704356352,
    //         (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Rook }) => 5962083034202112,
    //         (Position { file: File::D, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Rook }) => 6252140389466112,
    //         (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Rook }) => 2928425522167808,
    //         (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Rook }) => 421873730977792,
    //         (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Rook }) => 6634785931788288,
    //         (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Rook }) => 8114062256766976,
    //         (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Rook }) => 3144965777522688,
    //         (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Rook }) => 7788853813313536,
    //         (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Rook }) => 4412304969957376,
    //         (Position { file: File::E, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Rook }) => 952795410726912,
    //         (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Rook }) => 4938015461867520,
    //         (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Rook }) => 3314295928520704,
    //         (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Rook }) => 5087465482223616,
    //         (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Rook }) => 6728510898962432,
    //         (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Rook }) => 7423549104979968,
    //         (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Rook }) => 6524669752508416,
    //         (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Rook }) => 4327581776084992,
    //         (Position { file: File::F, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Rook }) => 129827155738624,
    //         (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Rook }) => 5568282868318208,
    //         (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Rook }) => 2379904096141312,
    //         (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Rook }) => 4970448393601024,
    //         (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Rook }) => 2962012130770944,
    //         (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Rook }) => 2572104306262016,
    //         (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Rook }) => 2656260837081088,
    //         (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Rook }) => 5704671301730304,
    //         (Position { file: File::G, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Rook }) => 6627431379107840,
    //         (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Rook }) => 5161132723535872,
    //         (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Rook }) => 8594821478350848,
    //         (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Rook }) => 3218309302452224,
    //         (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Rook }) => 4864602435223552,
    //         (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Rook }) => 3197468802023424,
    //         (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Rook }) => 682105447120896,
    //         (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Rook }) => 677127366115328,
    //         (Position { file: File::H, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Rook }) => 3128172444909568,
    //         (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Rook }) => 5200484545593344,
    //         (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Rook }) => 6877924001054720,
    //         (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Rook }) => 2974218557849600,
    //         (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Rook }) => 5297862753648640,
    //         (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Rook }) => 3355953040719872,
    //         (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Rook }) => 7834605566558208,
    //         (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Rook }) => 1662496695910400,
    //         (Position { file: File::A, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Bishop }) => 6975721920528384,
    //         (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Bishop }) => 1353224181252096,
    //         (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Bishop }) => 3662320586719232,
    //         (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Bishop }) => 1168672496287744,
    //         (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Bishop }) => 4110626618081280,
    //         (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Bishop }) => 7603401347039232,
    //         (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Bishop }) => 1252645455003648,
    //         (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Bishop }) => 4973012524728320,
    //         (Position { file: File::B, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Bishop }) => 2896315728003072,
    //         (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Bishop }) => 6834800262381568,
    //         (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Bishop }) => 8508231276036096,
    //         (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Bishop }) => 7146806951542784,
    //         (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Bishop }) => 3745810393071616,
    //         (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Bishop }) => 6429323670061056,
    //         (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Bishop }) => 3576203204100096,
    //         (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Bishop }) => 1425031460552704,
    //         (Position { file: File::C, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Bishop }) => 515746434121728,
    //         (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Bishop }) => 4593873481367552,
    //         (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Bishop }) => 2777059736682496,
    //         (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Bishop }) => 3612866909306880,
    //         (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Bishop }) => 2149803295244288,
    //         (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Bishop }) => 4661784633933824,
    //         (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Bishop }) => 1684288300908544,
    //         (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Bishop }) => 3491298463973376,
    //         (Position { file: File::D, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Bishop }) => 4673996593299456,
    //         (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Bishop }) => 3456491837194240,
    //         (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Bishop }) => 1235696492740608,
    //         (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Bishop }) => 1758014828707840,
    //         (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Bishop }) => 2028619352244224,
    //         (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Bishop }) => 7663976569634816,
    //         (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Bishop }) => 4517176618778624,
    //         (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Bishop }) => 2162395581513728,
    //         (Position { file: File::E, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Bishop }) => 8744688848732160,
    //         (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Bishop }) => 733897925066752,
    //         (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Bishop }) => 7890102288121856,
    //         (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Bishop }) => 4490296043765760,
    //         (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Bishop }) => 8219436743917568,
    //         (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Bishop }) => 2237426363793408,
    //         (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Bishop }) => 330353128505344,
    //         (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Bishop }) => 5363424229851136,
    //         (Position { file: File::F, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Bishop }) => 7525667551838208,
    //         (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Bishop }) => 3443855395389440,
    //         (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Bishop }) => 3250055228162048,
    //         (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Bishop }) => 3688218067206144,
    //         (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Bishop }) => 7664827570847744,
    //         (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Bishop }) => 6066110919081984,
    //         (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Bishop }) => 843351200890880,
    //         (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Bishop }) => 2160398790819840,
    //         (Position { file: File::G, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Bishop }) => 706266179567616,
    //         (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Bishop }) => 4862166572204032,
    //         (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Bishop }) => 413330577555456,
    //         (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Bishop }) => 8951908861476864,
    //         (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Bishop }) => 5731355394572288,
    //         (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Bishop }) => 3212143367290880,
    //         (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Bishop }) => 7099040757972992,
    //         (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Bishop }) => 7602059117330432,
    //         (Position { file: File::H, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Bishop }) => 6844276407271424,
    //         (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Bishop }) => 3401113889931264,
    //         (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Bishop }) => 631122291589120,
    //         (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Bishop }) => 6273313496104960,
    //         (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Bishop }) => 4204160417267712,
    //         (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Bishop }) => 7944926406901760,
    //         (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Bishop }) => 6520999990263808,
    //         (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Bishop }) => 2043423404064768,
    //         (Position { file: File::A, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Knight }) => 104624111484928,
    //         (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Knight }) => 5603283024478208,
    //         (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Knight }) => 2026733318438912,
    //         (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Knight }) => 916139798429696,
    //         (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Knight }) => 4622540785844224,
    //         (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Knight }) => 567599073067008,
    //         (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Knight }) => 8083372937052160,
    //         (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Knight }) => 5703731452575744,
    //         (Position { file: File::B, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Knight }) => 4799622448939008,
    //         (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Knight }) => 6249586523897856,
    //         (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Knight }) => 2462861290897408,
    //         (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Knight }) => 7919640531435520,
    //         (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Knight }) => 6006128263561216,
    //         (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Knight }) => 1384508983083008,
    //         (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Knight }) => 8273811692584960,
    //         (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Knight }) => 2626035940065280,
    //         (Position { file: File::C, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Knight }) => 109190305546240,
    //         (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Knight }) => 2685358267432960,
    //         (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Knight }) => 4822347424989184,
    //         (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Knight }) => 3877650856673280,
    //         (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Knight }) => 2201262516862976,
    //         (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Knight }) => 2478050149138432,
    //         (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Knight }) => 4946309165350912,
    //         (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Knight }) => 6673707904794624,
    //         (Position { file: File::D, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Knight }) => 828569200623616,
    //         (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Knight }) => 8502445518880768,
    //         (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Knight }) => 4666507938234368,
    //         (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Knight }) => 8440288819281920,
    //         (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Knight }) => 2773721309773824,
    //         (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Knight }) => 520139315347456,
    //         (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Knight }) => 3405947145814016,
    //         (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Knight }) => 5642227254558720,
    //         (Position { file: File::E, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Knight }) => 2790967633510400,
    //         (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Knight }) => 7570731263066112,
    //         (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Knight }) => 7629558815653888,
    //         (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Knight }) => 1613995777720320,
    //         (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Knight }) => 431950089158656,
    //         (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Knight }) => 5859597711572992,
    //         (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Knight }) => 967991202152448,
    //         (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Knight }) => 2427345950474240,
    //         (Position { file: File::F, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Knight }) => 761979169931264,
    //         (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Knight }) => 6842804636483584,
    //         (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Knight }) => 2473161138372608,
    //         (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Knight }) => 5228832097304576,
    //         (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Knight }) => 155926109618176,
    //         (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Knight }) => 4183094959538176,
    //         (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Knight }) => 8914041321816064,
    //         (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Knight }) => 6522304121012224,
    //         (Position { file: File::G, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Knight }) => 1614835399786496,
    //         (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Knight }) => 4357169535254528,
    //         (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Knight }) => 8897014414704640,
    //         (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Knight }) => 6102527810469888,
    //         (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Knight }) => 8148245402353664,
    //         (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Knight }) => 5621180354527232,
    //         (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Knight }) => 5508784847847424,
    //         (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Knight }) => 6744184767643648,
    //         (Position { file: File::H, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Knight }) => 6978920643559424,
    //         (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Knight }) => 7354743215095808,
    //         (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Knight }) => 3417406762385408,
    //         (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Knight }) => 877776596893696,
    //         (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Knight }) => 6512531629670400,
    //         (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Knight }) => 5623064289083392,
    //         (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Knight }) => 7191605568077824,
    //         (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Knight }) => 5010471836975104,
    //         (Position { file: File::A, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Pawn }) => 4525458448187392,
    //         (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Pawn }) => 8103563301486592,
    //         (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Pawn }) => 6674844838002688,
    //         (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Pawn }) => 861351626932224,
    //         (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Pawn }) => 773994137518080,
    //         (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Pawn }) => 6511376814047232,
    //         (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Pawn }) => 7499417854672896,
    //         (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Pawn }) => 6197198855340032,
    //         (Position { file: File::B, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Pawn }) => 7630430748540928,
    //         (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Pawn }) => 387971398238208,
    //         (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Pawn }) => 6862109931470848,
    //         (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Pawn }) => 8119297454899200,
    //         (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Pawn }) => 79924624883712,
    //         (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Pawn }) => 7976427913740288,
    //         (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Pawn }) => 6041632780582912,
    //         (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Pawn }) => 463887726018560,
    //         (Position { file: File::C, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Pawn }) => 6897832248213504,
    //         (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Pawn }) => 3043715933274112,
    //         (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Pawn }) => 4211977668788224,
    //         (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Pawn }) => 4618456393580544,
    //         (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Pawn }) => 168120977195008,
    //         (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Pawn }) => 4672191994003456,
    //         (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Pawn }) => 3346140204367872,
    //         (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Pawn }) => 6907591496040448,
    //         (Position { file: File::D, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Pawn }) => 4132117659254784,
    //         (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Pawn }) => 30157167394816,
    //         (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Pawn }) => 1554597992726528,
    //         (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Pawn }) => 6492881416093696,
    //         (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Pawn }) => 8849058558902272,
    //         (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Pawn }) => 8212595519520768,
    //         (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Pawn }) => 2285075704053760,
    //         (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Pawn }) => 8807641591578624,
    //         (Position { file: File::E, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Pawn }) => 6992388188798976,
    //         (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Pawn }) => 7161378427109376,
    //         (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Pawn }) => 4918250806706176,
    //         (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Pawn }) => 5818017449508864,
    //         (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Pawn }) => 7038995385024512,
    //         (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Pawn }) => 1026205807542272,
    //         (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Pawn }) => 3037660805332992,
    //         (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Pawn }) => 6643524275535872,
    //         (Position { file: File::F, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Pawn }) => 3034904954667008,
    //         (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Pawn }) => 1605453242957824,
    //         (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Pawn }) => 6200621659062272,
    //         (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Pawn }) => 3511862459105280,
    //         (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Pawn }) => 1220749295616000,
    //         (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Pawn }) => 7276851701481472,
    //         (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Pawn }) => 1104016884891648,
    //         (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Pawn }) => 3935487213436928,
    //         (Position { file: File::G, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Pawn }) => 8877762559344640,
    //         (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Pawn }) => 2803171132964864,
    //         (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Pawn }) => 7338393383993344,
    //         (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Pawn }) => 5546547441827840,
    //         (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Pawn }) => 3884371652116480,
    //         (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Pawn }) => 1736093558898688,
    //         (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Pawn }) => 352896671547392,
    //         (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Pawn }) => 9002200959287296,
    //         (Position { file: File::H, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Pawn }) => 2400306306482176,
    //         (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Pawn }) => 2120128902725632,
    //         (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Pawn }) => 5905716665647104,
    //         (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Pawn }) => 8396560536698880,
    //         (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Pawn }) => 8168478752112640,
    //         (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Pawn }) => 4311006146723840,
    //         (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Pawn }) => 2441597140598784,
    //         (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Pawn }) => 6251198229250048,
    //         (Position { file: File::A, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Queen }) => 4222491845197824,
    //         (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Queen }) => 3527492511989760,
    //         (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Queen }) => 3713491332497408,
    //         (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Queen }) => 6086185564766208,
    //         (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Queen }) => 1298163717636096,
    //         (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Queen }) => 1759705854640128,
    //         (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Queen }) => 3604984069357568,
    //         (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Queen }) => 919513541378048,
    //         (Position { file: File::B, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Queen }) => 3299983734341632,
    //         (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Queen }) => 7887444345094144,
    //         (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Queen }) => 419519872892928,
    //         (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Queen }) => 7137531132903424,
    //         (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Queen }) => 8083798793125888,
    //         (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Queen }) => 3582219721375744,
    //         (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Queen }) => 3460663458398208,
    //         (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Queen }) => 5089162931732480,
    //         (Position { file: File::C, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Queen }) => 4545473215987712,
    //         (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Queen }) => 3679735651500032,
    //         (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Queen }) => 6884255026118656,
    //         (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Queen }) => 7637694255464448,
    //         (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Queen }) => 1689908544536576,
    //         (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Queen }) => 4054312216952832,
    //         (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Queen }) => 7169241637715968,
    //         (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Queen }) => 3620537454559232,
    //         (Position { file: File::D, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Queen }) => 1421750516580352,
    //         (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Queen }) => 4244969176956928,
    //         (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Queen }) => 2353351555547136,
    //         (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Queen }) => 6095615408734208,
    //         (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Queen }) => 6103241515335680,
    //         (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Queen }) => 1887228240330752,
    //         (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Queen }) => 4070693822005248,
    //         (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Queen }) => 1181592489820160,
    //         (Position { file: File::E, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Queen }) => 7344054184443904,
    //         (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Queen }) => 7308155230879744,
    //         (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Queen }) => 1055438932017152,
    //         (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Queen }) => 4159865320636416,
    //         (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Queen }) => 4912013585154048,
    //         (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Queen }) => 1080803446489088,
    //         (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Queen }) => 3832100482449408,
    //         (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Queen }) => 282804055179264,
    //         (Position { file: File::F, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Queen }) => 4077708094996480,
    //         (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Queen }) => 5132640287981568,
    //         (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Queen }) => 6116876960661504,
    //         (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Queen }) => 6709594644545536,
    //         (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Queen }) => 1626854104498176,
    //         (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Queen }) => 4797958599999488,
    //         (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Queen }) => 5181516843843584,
    //         (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Queen }) => 2187089894440960,
    //         (Position { file: File::G, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Queen }) => 1434418986090496,
    //         (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Queen }) => 2978282268000256,
    //         (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Queen }) => 4230446736998400,
    //         (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Queen }) => 8170734499135488,
    //         (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Queen }) => 4398550595141632,
    //         (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Queen }) => 6778177284210688,
    //         (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Queen }) => 8943027825410048,
    //         (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Queen }) => 8745728654442496,
    //         (Position { file: File::H, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Queen }) => 6185021645783040,
    //         (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Queen }) => 92996078927872,
    //         (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Queen }) => 311653975457792,
    //         (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Queen }) => 109089612890112,
    //         (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Queen }) => 6378806516383744,
    //         (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Queen }) => 4922253659602944,
    //         (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Queen }) => 7733662514675712,
    //         (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Queen }) => 2756426338402304,
    //         (Position { file: File::A, rank: Rank::One }, Piece { color: Color::White, kind: Kind::King }) => 8950472066990080,
    //         (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::King }) => 2744554956521472,
    //         (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::King }) => 1005121276215296,
    //         (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::King }) => 6148439054221312,
    //         (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::King }) => 1570129812914176,
    //         (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::King }) => 6696005135237120,
    //         (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::King }) => 4317043469844480,
    //         (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::King }) => 3794094503493632,
    //         (Position { file: File::B, rank: Rank::One }, Piece { color: Color::White, kind: Kind::King }) => 2270865523736576,
    //         (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::King }) => 2915563787517952,
    //         (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::King }) => 974970754695168,
    //         (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::King }) => 5291917111721984,
    //         (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::King }) => 5642082658025472,
    //         (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::King }) => 5227242718232576,
    //         (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::King }) => 286310902267904,
    //         (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::King }) => 3258047944196096,
    //         (Position { file: File::C, rank: Rank::One }, Piece { color: Color::White, kind: Kind::King }) => 1115095381835776,
    //         (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::King }) => 2612469939830784,
    //         (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::King }) => 7116340840103936,
    //         (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::King }) => 6486600898838528,
    //         (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::King }) => 5744770943025152,
    //         (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::King }) => 7747830590472192,
    //         (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::King }) => 1083763949830144,
    //         (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::King }) => 7062591274745856,
    //         (Position { file: File::D, rank: Rank::One }, Piece { color: Color::White, kind: Kind::King }) => 1317780758462464,
    //         (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::King }) => 430058562912256,
    //         (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::King }) => 7733020649848832,
    //         (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::King }) => 4132437911142400,
    //         (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::King }) => 3282695387873280,
    //         (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::King }) => 3161902966374400,
    //         (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::King }) => 4534478061961216,
    //         (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::King }) => 4361640107573248,
    //         (Position { file: File::E, rank: Rank::One }, Piece { color: Color::White, kind: Kind::King }) => 6400288044875776,
    //         (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::King }) => 6565484054970368,
    //         (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::King }) => 7412993195245568,
    //         (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::King }) => 2475944757428224,
    //         (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::King }) => 5089460731510784,
    //         (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::King }) => 4516327937015808,
    //         (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::King }) => 3640605775233024,
    //         (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::King }) => 6402148912857088,
    //         (Position { file: File::F, rank: Rank::One }, Piece { color: Color::White, kind: Kind::King }) => 6412646821658624,
    //         (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::King }) => 6519301299240960,
    //         (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::King }) => 75043184836608,
    //         (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::King }) => 7032106406379520,
    //         (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::King }) => 8479044175659008,
    //         (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::King }) => 34216886665216,
    //         (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::King }) => 1813437774561280,
    //         (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::King }) => 838225455218688,
    //         (Position { file: File::G, rank: Rank::One }, Piece { color: Color::White, kind: Kind::King }) => 3574272429654016,
    //         (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::King }) => 7250776594317312,
    //         (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::King }) => 97769440149504,
    //         (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::King }) => 6525646230519808,
    //         (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::King }) => 3577648601104384,
    //         (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::King }) => 106702066155520,
    //         (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::King }) => 7635503025225728,
    //         (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::King }) => 5518784659980288,
    //         (Position { file: File::H, rank: Rank::One }, Piece { color: Color::White, kind: Kind::King }) => 95385636831232,
    //         (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::King }) => 1429701365071872,
    //         (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::King }) => 1608113408966656,
    //         (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::King }) => 6333067606097920,
    //         (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::King }) => 3249565570433024,
    //         (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::King }) => 2833706660134912,
    //         (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::King }) => 3318107747647488,
    //         (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::King }) => 4796336392110080,
    //     }
    // }
}

pub fn north_stepper(p: &Position) -> Option<Position> {
    p.step(Direction::None, Direction::Inc)
}

pub fn north_east_stepper(p: &Position) -> Option<Position> {
    p.step(Direction::Inc, Direction::Inc)
}

pub fn north_west_stepper(p: &Position) -> Option<Position> {
    p.step(Direction::Dec, Direction::Inc)
}

pub fn east_stepper(p: &Position) -> Option<Position> {
    p.step(Direction::Inc, Direction::None)
}

pub fn west_stepper(p: &Position) -> Option<Position> {
    p.step(Direction::Dec, Direction::None)
}

pub fn south_east_stepper(p: &Position) -> Option<Position> {
    p.step(Direction::Inc, Direction::Dec)
}

pub fn south_west_stepper(p: &Position) -> Option<Position> {
    p.step(Direction::Dec, Direction::Dec)
}

pub fn south_stepper(p: &Position) -> Option<Position> {
    p.step(Direction::None, Direction::Dec)
}
