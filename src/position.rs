use crate::direction::Direction;
use crate::file::File;
use crate::rank::Rank;
use crate::piece::Piece;
use crate::color::Color;
use crate::kind::Kind;

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

    pub fn hash(&self, piece: &Piece) -> u64 {
        match (self, piece) {
            (Position { file: File::A, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Rook }) => 1047498355768714921,
            (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Rook }) => 11732260526934782710,
            (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Rook }) => 7601703913300951192,
            (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Rook }) => 4020548701448569424,
            (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Rook }) => 4208089238702391346,
            (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Rook }) => 13248906572563743535,
            (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Rook }) => 13698386655415632463,
            (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Rook }) => 8689285492023231471,
            (Position { file: File::B, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Rook }) => 11441732661102516205,
            (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Rook }) => 16153402082206941240,
            (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Rook }) => 6695713994235183488,
            (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Rook }) => 1216422059207295596,
            (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Rook }) => 13919116737132888424,
            (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Rook }) => 847738471631028242,
            (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Rook }) => 1677008911165764412,
            (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Rook }) => 10864327539979977548,
            (Position { file: File::C, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Rook }) => 2166697046759702555,
            (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Rook }) => 8196009859582263478,
            (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Rook }) => 5826886979777724226,
            (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Rook }) => 9763081376776585723,
            (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Rook }) => 12710097938823512546,
            (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Rook }) => 479030449126821141,
            (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Rook }) => 9755960458438772319,
            (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Rook }) => 5561651476276707727,
            (Position { file: File::D, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Rook }) => 13849642823621542885,
            (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Rook }) => 12131126567682186580,
            (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Rook }) => 11845277441658454638,
            (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Rook }) => 8140715707584217971,
            (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Rook }) => 18398996986704435861,
            (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Rook }) => 11199957866726487754,
            (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Rook }) => 6434807980977291404,
            (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Rook }) => 8005121524170228371,
            (Position { file: File::E, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Rook }) => 5289165694317887290,
            (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Rook }) => 9764233909775634340,
            (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Rook }) => 2719966199730405436,
            (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Rook }) => 2972212025616236580,
            (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Rook }) => 15006029207195093119,
            (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Rook }) => 18045639577607078173,
            (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Rook }) => 4056206245789434916,
            (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Rook }) => 4994414715997258105,
            (Position { file: File::F, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Rook }) => 18169993363456553615,
            (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Rook }) => 10148342116481237159,
            (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Rook }) => 9938649641153253723,
            (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Rook }) => 4634940835383738101,
            (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Rook }) => 9227635629414679838,
            (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Rook }) => 16684122648283716068,
            (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Rook }) => 8014416997334909783,
            (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Rook }) => 16418907155035324644,
            (Position { file: File::G, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Rook }) => 1341826483821635223,
            (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Rook }) => 3442525457222729786,
            (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Rook }) => 6513440693204025868,
            (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Rook }) => 1200290786531763847,
            (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Rook }) => 4219109325920012662,
            (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Rook }) => 17924346316733809537,
            (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Rook }) => 5901406599393575262,
            (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Rook }) => 15673069050244628532,
            (Position { file: File::H, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Rook }) => 167619164674981913,
            (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Rook }) => 5373044339729498117,
            (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Rook }) => 11976424117717959435,
            (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Rook }) => 13778430350298448577,
            (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Rook }) => 16296674249806973321,
            (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Rook }) => 18196983788347589823,
            (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Rook }) => 1297836793550341290,
            (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Rook }) => 10953027208871936694,
            (Position { file: File::A, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Bishop }) => 10721156510269833133,
            (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Bishop }) => 13313123252831257619,
            (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Bishop }) => 5460597969612964820,
            (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Bishop }) => 1695524561564467219,
            (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Bishop }) => 14618828499482313497,
            (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Bishop }) => 6025582148394686499,
            (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Bishop }) => 1471189711648194627,
            (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Bishop }) => 15153087749243798237,
            (Position { file: File::B, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Bishop }) => 7196999594654302891,
            (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Bishop }) => 8393311186757616715,
            (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Bishop }) => 4752665674218734412,
            (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Bishop }) => 16571198410359574761,
            (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Bishop }) => 17853468812205097658,
            (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Bishop }) => 32351006215477663,
            (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Bishop }) => 8931609745886806254,
            (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Bishop }) => 11567849887508726341,
            (Position { file: File::C, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Bishop }) => 1172543893345298441,
            (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Bishop }) => 11538257811947389488,
            (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Bishop }) => 16146985504145932671,
            (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Bishop }) => 3702049448789016633,
            (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Bishop }) => 13789421012695122505,
            (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Bishop }) => 5399217466704396983,
            (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Bishop }) => 4169448689250599939,
            (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Bishop }) => 11048815347623789522,
            (Position { file: File::D, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Bishop }) => 11136214998634725183,
            (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Bishop }) => 15029858531861856779,
            (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Bishop }) => 15717207823555232566,
            (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Bishop }) => 266910176103733660,
            (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Bishop }) => 15896917127894925118,
            (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Bishop }) => 458472848010038516,
            (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Bishop }) => 6897067502795227133,
            (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Bishop }) => 17032921675445839766,
            (Position { file: File::E, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Bishop }) => 765576880270920944,
            (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Bishop }) => 16170187157997093701,
            (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Bishop }) => 237984987845387918,
            (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Bishop }) => 1419487674016517390,
            (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Bishop }) => 3341466337036730480,
            (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Bishop }) => 6342516688633725767,
            (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Bishop }) => 3955628887790059558,
            (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Bishop }) => 3581109887062507571,
            (Position { file: File::F, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Bishop }) => 775274358938533989,
            (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Bishop }) => 14560318161488905448,
            (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Bishop }) => 1485625678698119279,
            (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Bishop }) => 1437623483520516385,
            (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Bishop }) => 2335766248228389207,
            (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Bishop }) => 212231200431931418,
            (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Bishop }) => 18109307262269915747,
            (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Bishop }) => 224418294688382980,
            (Position { file: File::G, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Bishop }) => 13383504010032447821,
            (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Bishop }) => 6264700449689962393,
            (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Bishop }) => 10417646946876391361,
            (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Bishop }) => 2176954090087514868,
            (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Bishop }) => 13648190492158984360,
            (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Bishop }) => 5268583699934347671,
            (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Bishop }) => 5078611386557268688,
            (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Bishop }) => 11277259770425246178,
            (Position { file: File::H, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Bishop }) => 4984530359866622504,
            (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Bishop }) => 13867359456343364794,
            (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Bishop }) => 12015615964876898585,
            (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Bishop }) => 15908500143591139103,
            (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Bishop }) => 10667564337847075972,
            (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Bishop }) => 10805824433231495232,
            (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Bishop }) => 10390432118533521108,
            (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Bishop }) => 14003874850112799581,
            (Position { file: File::A, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Knight }) => 6596669256660681224,
            (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Knight }) => 12335186532703928296,
            (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Knight }) => 534430657611300860,
            (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Knight }) => 17317169590416966201,
            (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Knight }) => 9397263545999032927,
            (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Knight }) => 7389196486342345144,
            (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Knight }) => 5764048366464716911,
            (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Knight }) => 16850940964766745321,
            (Position { file: File::B, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Knight }) => 1409576861746331633,
            (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Knight }) => 8839497630988894225,
            (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Knight }) => 1402362187278922562,
            (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Knight }) => 15315537375317197811,
            (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Knight }) => 13589743752561295133,
            (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Knight }) => 1664268298274576841,
            (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Knight }) => 2561628407842472726,
            (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Knight }) => 11404448105840509373,
            (Position { file: File::C, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Knight }) => 1994472886869426223,
            (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Knight }) => 18183016331362697283,
            (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Knight }) => 17878482310894977565,
            (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Knight }) => 10156910473157542644,
            (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Knight }) => 627584931720698340,
            (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Knight }) => 11711249491083395745,
            (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Knight }) => 10228778994145165183,
            (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Knight }) => 12505011541042528363,
            (Position { file: File::D, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Knight }) => 7703788869245731521,
            (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Knight }) => 15018936713495971162,
            (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Knight }) => 13640219449469436728,
            (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Knight }) => 1383409833624993840,
            (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Knight }) => 970885689497354232,
            (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Knight }) => 8472983173695799678,
            (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Knight }) => 17877154551820199184,
            (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Knight }) => 11034437158621413641,
            (Position { file: File::E, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Knight }) => 16392161423020524970,
            (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Knight }) => 969112060848137899,
            (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Knight }) => 1625044452750288560,
            (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Knight }) => 7950062397077586410,
            (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Knight }) => 622047214349110514,
            (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Knight }) => 9411464026434568433,
            (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Knight }) => 1389262794294663665,
            (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Knight }) => 16159183580633768298,
            (Position { file: File::F, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Knight }) => 615129438085972223,
            (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Knight }) => 14793190822941557148,
            (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Knight }) => 17294313681433133495,
            (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Knight }) => 12330368416916448877,
            (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Knight }) => 3660248945868669106,
            (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Knight }) => 12282236994762834742,
            (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Knight }) => 6331263973102453670,
            (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Knight }) => 17172464497172939976,
            (Position { file: File::G, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Knight }) => 382652522029908521,
            (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Knight }) => 10259715420239954757,
            (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Knight }) => 16434759097640813469,
            (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Knight }) => 16016185719710024846,
            (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Knight }) => 11886571615179244727,
            (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Knight }) => 15307721750314222197,
            (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Knight }) => 16946565564048867304,
            (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Knight }) => 1778869677548634337,
            (Position { file: File::H, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Knight }) => 11809451577549259888,
            (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Knight }) => 12804903900159148942,
            (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Knight }) => 8891713305047994176,
            (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Knight }) => 10638594650291569146,
            (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Knight }) => 5552270464543359978,
            (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Knight }) => 12936319393926742179,
            (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Knight }) => 6708830931731351260,
            (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Knight }) => 17940437690881147195,
            (Position { file: File::A, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Pawn }) => 12606257831608326627,
            (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Pawn }) => 2973089779492585511,
            (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Pawn }) => 6713041713373381263,
            (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Pawn }) => 4642095692553126406,
            (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Pawn }) => 6846297098116137930,
            (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Pawn }) => 8478051497098084429,
            (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Pawn }) => 17948161588267581619,
            (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Pawn }) => 27614483210200687,
            (Position { file: File::B, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Pawn }) => 14267881670844613764,
            (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Pawn }) => 3932271528489517304,
            (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Pawn }) => 1262497702995610827,
            (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Pawn }) => 24714488220693358,
            (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Pawn }) => 4956207146493542462,
            (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Pawn }) => 1454521598554931234,
            (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Pawn }) => 1298110069434482788,
            (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Pawn }) => 15935848944848863485,
            (Position { file: File::C, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Pawn }) => 17661382893882049713,
            (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Pawn }) => 8141839094935192771,
            (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Pawn }) => 7685298266237305459,
            (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Pawn }) => 12006419641032245827,
            (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Pawn }) => 841617430269657195,
            (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Pawn }) => 14191224046573584286,
            (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Pawn }) => 3655514951310639779,
            (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Pawn }) => 12024024467165413846,
            (Position { file: File::D, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Pawn }) => 9667951046625853730,
            (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Pawn }) => 6845618321484677188,
            (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Pawn }) => 8795735337331589524,
            (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Pawn }) => 18372498958338494804,
            (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Pawn }) => 9177307211230085120,
            (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Pawn }) => 17214993117309043461,
            (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Pawn }) => 13641820793666011913,
            (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Pawn }) => 4377384020443398327,
            (Position { file: File::E, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Pawn }) => 8934873401340723548,
            (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Pawn }) => 6481730949657657250,
            (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Pawn }) => 10789214317229113280,
            (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Pawn }) => 1769307057418141771,
            (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Pawn }) => 1385619804752183352,
            (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Pawn }) => 339513458192023550,
            (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Pawn }) => 6808952104074346979,
            (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Pawn }) => 4370259730556256284,
            (Position { file: File::F, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Pawn }) => 474460858185941502,
            (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Pawn }) => 7304624401280926439,
            (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Pawn }) => 16847917226185982588,
            (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Pawn }) => 12744937416448543827,
            (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Pawn }) => 940736539497857265,
            (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Pawn }) => 11933214023710933494,
            (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Pawn }) => 1567735319422304322,
            (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Pawn }) => 8469387689068593253,
            (Position { file: File::G, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Pawn }) => 11655411471671624583,
            (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Pawn }) => 5548953508545298478,
            (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Pawn }) => 13283143788399694405,
            (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Pawn }) => 4602349433835749492,
            (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Pawn }) => 16734977904336730676,
            (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Pawn }) => 7654354229460468555,
            (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Pawn }) => 4832675576298865970,
            (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Pawn }) => 4256801396268990530,
            (Position { file: File::H, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Pawn }) => 7564935488553878795,
            (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Pawn }) => 13010552934360941057,
            (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Pawn }) => 18205789283053208941,
            (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Pawn }) => 6772136739629695485,
            (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Pawn }) => 700640517244426625,
            (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Pawn }) => 14895214189055312408,
            (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Pawn }) => 937790097443520540,
            (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Pawn }) => 2864627742006051132,
            (Position { file: File::A, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Queen }) => 6037874241716617793,
            (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Queen }) => 1377958827348145846,
            (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Queen }) => 10314208547952394599,
            (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Queen }) => 16210149548058739720,
            (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Queen }) => 1729746672429931297,
            (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Queen }) => 6088612855047782853,
            (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Queen }) => 12859102332985541446,
            (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Queen }) => 16486279536753778345,
            (Position { file: File::B, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Queen }) => 2742491464951922793,
            (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Queen }) => 2153141008987062364,
            (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Queen }) => 12622579892744094838,
            (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Queen }) => 2603116189899030544,
            (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Queen }) => 10880415229045375287,
            (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Queen }) => 13546258376920448282,
            (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Queen }) => 7381790463780454535,
            (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Queen }) => 10548434348921586466,
            (Position { file: File::C, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Queen }) => 2978990699485069361,
            (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Queen }) => 5613823985914479112,
            (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Queen }) => 16390895365445911859,
            (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Queen }) => 412463915801247740,
            (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Queen }) => 18133687012765467552,
            (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Queen }) => 10760994646927606331,
            (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Queen }) => 15380551390493416578,
            (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Queen }) => 17691775596782065738,
            (Position { file: File::D, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Queen }) => 6189324190256988314,
            (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Queen }) => 7620142414061109725,
            (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Queen }) => 2629322181963677736,
            (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Queen }) => 60308809354471015,
            (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Queen }) => 10503438450462556813,
            (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Queen }) => 6595750868098744474,
            (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Queen }) => 2849348052253147704,
            (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Queen }) => 121195764815002346,
            (Position { file: File::E, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Queen }) => 1718029801071470279,
            (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Queen }) => 3811417075834945539,
            (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Queen }) => 17329176626759467364,
            (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Queen }) => 9535157916328264960,
            (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Queen }) => 11009302065112416424,
            (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Queen }) => 5404870425349983,
            (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Queen }) => 13681453897175007230,
            (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Queen }) => 9908046405736333484,
            (Position { file: File::F, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Queen }) => 1979378640919462893,
            (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Queen }) => 17810802053035852775,
            (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Queen }) => 3767355285518156886,
            (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Queen }) => 14181269493497463864,
            (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Queen }) => 12380701477960683928,
            (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Queen }) => 1063848401193402463,
            (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Queen }) => 5934265104957702890,
            (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Queen }) => 9333908547840246157,
            (Position { file: File::G, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Queen }) => 6385625588916295279,
            (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Queen }) => 7827298805690139942,
            (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Queen }) => 6534181713046667780,
            (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Queen }) => 8543256526649819264,
            (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Queen }) => 105937278757293265,
            (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Queen }) => 14091114977448428830,
            (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Queen }) => 2971012630229025879,
            (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Queen }) => 16633464372315816166,
            (Position { file: File::H, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::Queen }) => 11785703457829159263,
            (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::Queen }) => 5122406639793603933,
            (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::Queen }) => 13727954971217887772,
            (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::Queen }) => 2496070765770703665,
            (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::Queen }) => 1346695946372644930,
            (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::Queen }) => 1801549880708877512,
            (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::Queen }) => 3639169285034606642,
            (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::Queen }) => 17582716925723542418,
            (Position { file: File::A, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::King }) => 7355374061267976110,
            (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::King }) => 9118810933479604601,
            (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::King }) => 5489106575249375991,
            (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::King }) => 9623243812918788933,
            (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::King }) => 7116850681662669513,
            (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::King }) => 4288723633322328450,
            (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::King }) => 16351565967664356211,
            (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::King }) => 11615148798971478165,
            (Position { file: File::B, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::King }) => 9191844687684567113,
            (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::King }) => 11827230139404517931,
            (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::King }) => 3950819160073699392,
            (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::King }) => 18184789293862486206,
            (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::King }) => 1733485015829846352,
            (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::King }) => 7384437860146872239,
            (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::King }) => 10222167261360167542,
            (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::King }) => 7393559026358813206,
            (Position { file: File::C, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::King }) => 13502362149002936749,
            (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::King }) => 439849628007923778,
            (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::King }) => 15966746660403413778,
            (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::King }) => 16751838734760018764,
            (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::King }) => 8073766813936124744,
            (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::King }) => 8906713224926220345,
            (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::King }) => 7570997392510556071,
            (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::King }) => 4985217129432221331,
            (Position { file: File::D, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::King }) => 1069240591329106392,
            (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::King }) => 1392588553687190793,
            (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::King }) => 12445771405021151478,
            (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::King }) => 14163176106244447355,
            (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::King }) => 6246733115432108263,
            (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::King }) => 2078862573259718738,
            (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::King }) => 896929337662505818,
            (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::King }) => 946413682674788471,
            (Position { file: File::E, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::King }) => 5748011498819551780,
            (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::King }) => 16316889247825002137,
            (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::King }) => 16331730902453322274,
            (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::King }) => 16597235365396349275,
            (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::King }) => 10594017820381544919,
            (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::King }) => 6897844174616265265,
            (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::King }) => 268968594585419780,
            (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::King }) => 82342228665145757,
            (Position { file: File::F, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::King }) => 1148399109343595742,
            (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::King }) => 5541810797543424394,
            (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::King }) => 14912498941340681495,
            (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::King }) => 1710278820050763815,
            (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::King }) => 1217436656325152235,
            (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::King }) => 17533584041750562382,
            (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::King }) => 15377262703970094183,
            (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::King }) => 7086740525671252290,
            (Position { file: File::G, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::King }) => 1243995787472824214,
            (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::King }) => 272342098122899462,
            (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::King }) => 13046752628128612619,
            (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::King }) => 10359733032331515603,
            (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::King }) => 18286248254028906443,
            (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::King }) => 5134237561002131220,
            (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::King }) => 15136741082240385252,
            (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::King }) => 1619453256163917819,
            (Position { file: File::H, rank: Rank::One }, Piece { color: Color::Black, kind: Kind::King }) => 9433902340043178852,
            (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::Black, kind: Kind::King }) => 9963739701930099314,
            (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::Black, kind: Kind::King }) => 4856488833492976881,
            (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::Black, kind: Kind::King }) => 12447585543372407360,
            (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::Black, kind: Kind::King }) => 16546659123303816936,
            (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::Black, kind: Kind::King }) => 1143526109457521690,
            (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::Black, kind: Kind::King }) => 9837509913020999903,
            (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::Black, kind: Kind::King }) => 11114508225330807921,
            (Position { file: File::A, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Rook }) => 992760245698691112,
            (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Rook }) => 246850625407549449,
            (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Rook }) => 16161663241347422356,
            (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Rook }) => 1830281170521161730,
            (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Rook }) => 927459064714100754,
            (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Rook }) => 12255281956106797152,
            (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Rook }) => 11366537155368196720,
            (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Rook }) => 14080694489140167182,
            (Position { file: File::B, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Rook }) => 14505246646230581240,
            (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Rook }) => 7674614771311706890,
            (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Rook }) => 8614871309559529583,
            (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Rook }) => 2892633948137985138,
            (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Rook }) => 411288202863706138,
            (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Rook }) => 14412106449305993216,
            (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Rook }) => 7415050664750875492,
            (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Rook }) => 14310483266770043301,
            (Position { file: File::C, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Rook }) => 624226934733144545,
            (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Rook }) => 14273898224241607961,
            (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Rook }) => 2517518664215822347,
            (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Rook }) => 10414704116889552564,
            (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Rook }) => 15739111959196336989,
            (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Rook }) => 4137448769123778669,
            (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Rook }) => 356317732485988350,
            (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Rook }) => 12509521252408361826,
            (Position { file: File::D, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Rook }) => 18267357922709215596,
            (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Rook }) => 7046518650865451259,
            (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Rook }) => 5800968217945965214,
            (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Rook }) => 13950335999709024689,
            (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Rook }) => 3902857814330048528,
            (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Rook }) => 15420745201982898751,
            (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Rook }) => 11872709316173627150,
            (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Rook }) => 4349253500778053693,
            (Position { file: File::E, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Rook }) => 11749942447726133908,
            (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Rook }) => 1737445132872555734,
            (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Rook }) => 9115279877656805478,
            (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Rook }) => 1298449071203156715,
            (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Rook }) => 6009482918752158161,
            (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Rook }) => 17681343208929362713,
            (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Rook }) => 889556407394133621,
            (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Rook }) => 14840101958987022333,
            (Position { file: File::F, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Rook }) => 17108270498627715277,
            (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Rook }) => 3890324833472348723,
            (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Rook }) => 3647501573128454282,
            (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Rook }) => 14060437331723158472,
            (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Rook }) => 16939212751836807990,
            (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Rook }) => 7440655584732381620,
            (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Rook }) => 17974738205945102482,
            (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Rook }) => 6142680274191778677,
            (Position { file: File::G, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Rook }) => 172892709775107694,
            (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Rook }) => 10615670571586814561,
            (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Rook }) => 6952416080618848164,
            (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Rook }) => 1357444428448399454,
            (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Rook }) => 9769598010395722977,
            (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Rook }) => 14088663732238418561,
            (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Rook }) => 15733591087974973796,
            (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Rook }) => 11886348233930179105,
            (Position { file: File::H, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Rook }) => 4084149955852239293,
            (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Rook }) => 11474458430702354326,
            (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Rook }) => 10711147892145394155,
            (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Rook }) => 1125462773231675388,
            (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Rook }) => 12048504054175761223,
            (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Rook }) => 10869333672255816677,
            (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Rook }) => 13091256954695713426,
            (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Rook }) => 8639502354296603156,
            (Position { file: File::A, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Bishop }) => 11481449924495671926,
            (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Bishop }) => 9171590545334678555,
            (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Bishop }) => 3287349336790273680,
            (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Bishop }) => 12634111325883073384,
            (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Bishop }) => 2007204853561950268,
            (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Bishop }) => 17040922993359847291,
            (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Bishop }) => 16468570231246165976,
            (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Bishop }) => 3949170476747587665,
            (Position { file: File::B, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Bishop }) => 14357539465335935166,
            (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Bishop }) => 468811395643736060,
            (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Bishop }) => 2890591334706446372,
            (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Bishop }) => 9929097208685134208,
            (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Bishop }) => 9769766819790324445,
            (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Bishop }) => 1236081915522449483,
            (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Bishop }) => 6620441445377507390,
            (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Bishop }) => 13470239225307726737,
            (Position { file: File::C, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Bishop }) => 1142039603237512828,
            (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Bishop }) => 13462713480532467372,
            (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Bishop }) => 5301846104222997576,
            (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Bishop }) => 5150833709770867921,
            (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Bishop }) => 935230497510077276,
            (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Bishop }) => 2055643138243952667,
            (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Bishop }) => 164461634856471766,
            (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Bishop }) => 1301052877516992122,
            (Position { file: File::D, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Bishop }) => 8022246602456433501,
            (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Bishop }) => 18369262099185533965,
            (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Bishop }) => 15847020980988805938,
            (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Bishop }) => 4961327748237754879,
            (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Bishop }) => 3905707361922187321,
            (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Bishop }) => 16482235202274329625,
            (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Bishop }) => 959617366484425120,
            (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Bishop }) => 4498078756149133340,
            (Position { file: File::E, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Bishop }) => 279161619278200833,
            (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Bishop }) => 11414082082557657375,
            (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Bishop }) => 18394166522885964982,
            (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Bishop }) => 6090299209532047302,
            (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Bishop }) => 13402352839083164806,
            (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Bishop }) => 15794173302492103759,
            (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Bishop }) => 1313334418104280745,
            (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Bishop }) => 4341995766257025954,
            (Position { file: File::F, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Bishop }) => 7348599678071472732,
            (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Bishop }) => 17453819223389241318,
            (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Bishop }) => 5714416788616249538,
            (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Bishop }) => 6405681368201691489,
            (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Bishop }) => 1588129162026025711,
            (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Bishop }) => 15812265976780554843,
            (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Bishop }) => 15049797200273998320,
            (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Bishop }) => 12238773227102405936,
            (Position { file: File::G, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Bishop }) => 3527974402807374755,
            (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Bishop }) => 15823862942530535503,
            (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Bishop }) => 5352852066383432432,
            (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Bishop }) => 14531104376577065418,
            (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Bishop }) => 5513036470870344991,
            (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Bishop }) => 3305364976545300510,
            (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Bishop }) => 9348833640798224774,
            (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Bishop }) => 2149729542233653289,
            (Position { file: File::H, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Bishop }) => 15306616311631577790,
            (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Bishop }) => 16533080038736658462,
            (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Bishop }) => 17497404349645586888,
            (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Bishop }) => 13204166924738494148,
            (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Bishop }) => 11402376003163524015,
            (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Bishop }) => 3627534042536083545,
            (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Bishop }) => 16470139526396772409,
            (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Bishop }) => 13897513244907536219,
            (Position { file: File::A, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Knight }) => 6247124996838139444,
            (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Knight }) => 4618746721323713743,
            (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Knight }) => 4105169186496446516,
            (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Knight }) => 16515823229508518826,
            (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Knight }) => 446076170945953856,
            (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Knight }) => 11727801642207151608,
            (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Knight }) => 4806925258789487501,
            (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Knight }) => 13032295106785313782,
            (Position { file: File::B, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Knight }) => 16141230363933412298,
            (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Knight }) => 6965019563364712921,
            (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Knight }) => 5393662871399825574,
            (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Knight }) => 2679509879839785610,
            (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Knight }) => 591718347885563199,
            (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Knight }) => 15140973330128962883,
            (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Knight }) => 13942191612139405108,
            (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Knight }) => 7125830165493449754,
            (Position { file: File::C, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Knight }) => 9830520407797534379,
            (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Knight }) => 12149846904196364039,
            (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Knight }) => 12567281686296926106,
            (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Knight }) => 17545467838301474004,
            (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Knight }) => 1683114511260713459,
            (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Knight }) => 55886222259231131,
            (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Knight }) => 16186310967737456461,
            (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Knight }) => 3851093129016901668,
            (Position { file: File::D, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Knight }) => 3850982031097856186,
            (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Knight }) => 1889261486790934558,
            (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Knight }) => 9923952086972634023,
            (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Knight }) => 5404239318560539243,
            (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Knight }) => 16024384275407176740,
            (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Knight }) => 14871436927982109674,
            (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Knight }) => 15158260981517255205,
            (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Knight }) => 6699789707745698078,
            (Position { file: File::E, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Knight }) => 8571885679133327220,
            (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Knight }) => 12415623483382331322,
            (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Knight }) => 1349313174459279736,
            (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Knight }) => 7394486026330178904,
            (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Knight }) => 17809482424334156464,
            (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Knight }) => 13793239801261982812,
            (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Knight }) => 2519814315645659897,
            (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Knight }) => 3411709839968043502,
            (Position { file: File::F, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Knight }) => 7780031024150348989,
            (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Knight }) => 8745038685830906565,
            (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Knight }) => 13400283712818512916,
            (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Knight }) => 17527777920276234299,
            (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Knight }) => 12009974834571248797,
            (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Knight }) => 10423478906873976603,
            (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Knight }) => 15214852278063202341,
            (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Knight }) => 5512910142997266230,
            (Position { file: File::G, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Knight }) => 14714384692820312913,
            (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Knight }) => 10857588607443534348,
            (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Knight }) => 283048523878891520,
            (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Knight }) => 11396105484055347960,
            (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Knight }) => 12475548713932036748,
            (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Knight }) => 7461595903942459203,
            (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Knight }) => 1756683710878030230,
            (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Knight }) => 13762773691265974627,
            (Position { file: File::H, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Knight }) => 876265008755900431,
            (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Knight }) => 4666130252232131333,
            (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Knight }) => 5876887163676656208,
            (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Knight }) => 17956357498264889864,
            (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Knight }) => 1311554510212877537,
            (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Knight }) => 223879349307179320,
            (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Knight }) => 1880197525248409648,
            (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Knight }) => 14816677134340194903,
            (Position { file: File::A, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Pawn }) => 13357375623731872833,
            (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Pawn }) => 14185182745575039294,
            (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Pawn }) => 14997645229169836731,
            (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Pawn }) => 3907176232147484789,
            (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Pawn }) => 12026088125936697522,
            (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Pawn }) => 3907160770265219961,
            (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Pawn }) => 9886314558413344616,
            (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Pawn }) => 17243492196707992706,
            (Position { file: File::B, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Pawn }) => 13743433930273456959,
            (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Pawn }) => 11785600619132223537,
            (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Pawn }) => 14383190667885029988,
            (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Pawn }) => 2669037375182602180,
            (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Pawn }) => 16256970777553797704,
            (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Pawn }) => 11742957370613957866,
            (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Pawn }) => 793549818445168661,
            (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Pawn }) => 12084487930131775540,
            (Position { file: File::C, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Pawn }) => 2055370815842549872,
            (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Pawn }) => 209800720529804559,
            (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Pawn }) => 11091570218047111430,
            (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Pawn }) => 461190080544689468,
            (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Pawn }) => 16892615912908128305,
            (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Pawn }) => 1745384781069156473,
            (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Pawn }) => 2012969992458338317,
            (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Pawn }) => 8454712584682078711,
            (Position { file: File::D, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Pawn }) => 13081675870500815192,
            (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Pawn }) => 8356235465690448952,
            (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Pawn }) => 17901385735102005635,
            (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Pawn }) => 18267217765041439369,
            (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Pawn }) => 391427799063619234,
            (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Pawn }) => 5573288432467182131,
            (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Pawn }) => 3329804229517771704,
            (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Pawn }) => 10730246366170186388,
            (Position { file: File::E, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Pawn }) => 6782237162680091505,
            (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Pawn }) => 14034674657592869679,
            (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Pawn }) => 15393426800503685353,
            (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Pawn }) => 8425170936331764611,
            (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Pawn }) => 17277963191908827629,
            (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Pawn }) => 18145926295169532668,
            (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Pawn }) => 7715977990726746710,
            (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Pawn }) => 449420384524461670,
            (Position { file: File::F, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Pawn }) => 10141192275833651987,
            (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Pawn }) => 8876027487482544957,
            (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Pawn }) => 17543850813179298442,
            (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Pawn }) => 15211023271769145693,
            (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Pawn }) => 18359478520167532344,
            (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Pawn }) => 16307996518038634927,
            (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Pawn }) => 959038920718496615,
            (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Pawn }) => 9100174134079392485,
            (Position { file: File::G, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Pawn }) => 244377697484642761,
            (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Pawn }) => 13487975224037081301,
            (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Pawn }) => 18433891860817641203,
            (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Pawn }) => 2575115704663539730,
            (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Pawn }) => 3033045014683844692,
            (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Pawn }) => 16498999709466187670,
            (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Pawn }) => 13081529317626744485,
            (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Pawn }) => 17427317131984246401,
            (Position { file: File::H, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Pawn }) => 16681782269064511695,
            (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Pawn }) => 18070145436932375214,
            (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Pawn }) => 15900435749787402555,
            (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Pawn }) => 2957019691203166396,
            (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Pawn }) => 14536238975619695544,
            (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Pawn }) => 5736813501171634397,
            (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Pawn }) => 16138238897736909751,
            (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Pawn }) => 18008956193212465513,
            (Position { file: File::A, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Queen }) => 13457986100224066938,
            (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Queen }) => 7177303587599417998,
            (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Queen }) => 1654998972011800292,
            (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Queen }) => 8994604645397561486,
            (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Queen }) => 252588319462916167,
            (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Queen }) => 1004898208106256866,
            (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Queen }) => 14240100959085658952,
            (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Queen }) => 10524750237099098675,
            (Position { file: File::B, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Queen }) => 17086621922130657360,
            (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Queen }) => 7604912335475442785,
            (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Queen }) => 4458709698639560727,
            (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Queen }) => 9488156616034353569,
            (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Queen }) => 5310338551647306112,
            (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Queen }) => 10426219684419338262,
            (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Queen }) => 6889213618223579552,
            (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Queen }) => 6168392881888297930,
            (Position { file: File::C, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Queen }) => 10349029604462692851,
            (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Queen }) => 13982185995412446879,
            (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Queen }) => 1390713569393246280,
            (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Queen }) => 17013829764131062901,
            (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Queen }) => 13450617452467585331,
            (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Queen }) => 9194851396727301590,
            (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Queen }) => 2011837628625715225,
            (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Queen }) => 1477387868717868625,
            (Position { file: File::D, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Queen }) => 9194617213628187302,
            (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Queen }) => 4210075111320977456,
            (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Queen }) => 6263811421524463745,
            (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Queen }) => 5865938299901706808,
            (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Queen }) => 17030713426600526967,
            (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Queen }) => 14682076011789559819,
            (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Queen }) => 1694889347497800542,
            (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Queen }) => 5256383059351044634,
            (Position { file: File::E, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Queen }) => 1120955961827760595,
            (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Queen }) => 13365700674805826630,
            (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Queen }) => 11778949900929073295,
            (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Queen }) => 1746598916784128482,
            (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Queen }) => 772956901959794794,
            (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Queen }) => 10887913460260143705,
            (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Queen }) => 12915829041260397330,
            (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Queen }) => 1248687861861554637,
            (Position { file: File::F, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Queen }) => 15898558896323695076,
            (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Queen }) => 4477129589306425361,
            (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Queen }) => 9484618280241988103,
            (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Queen }) => 476842959146949493,
            (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Queen }) => 3101966432192168936,
            (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Queen }) => 9252059253001159613,
            (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Queen }) => 4865744226022851536,
            (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Queen }) => 15868350686228906921,
            (Position { file: File::G, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Queen }) => 4464520261110071362,
            (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Queen }) => 11957928773065638481,
            (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Queen }) => 2736916223739564159,
            (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Queen }) => 7291084482520547684,
            (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Queen }) => 1012218121098361779,
            (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Queen }) => 11504714868984906425,
            (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Queen }) => 15239033841587847329,
            (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Queen }) => 6362228818995839222,
            (Position { file: File::H, rank: Rank::One }, Piece { color: Color::White, kind: Kind::Queen }) => 15749515276225348223,
            (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::Queen }) => 3602592304159785150,
            (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::Queen }) => 14528451508422312455,
            (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::Queen }) => 17238632862119363124,
            (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::Queen }) => 11401273966094975394,
            (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::Queen }) => 2270346581981003869,
            (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::Queen }) => 16103440011847795386,
            (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::Queen }) => 14528602815825183230,
            (Position { file: File::A, rank: Rank::One }, Piece { color: Color::White, kind: Kind::King }) => 13406214860790825251,
            (Position { file: File::A, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::King }) => 5140884564058571461,
            (Position { file: File::A, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::King }) => 9486211459705733627,
            (Position { file: File::A, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::King }) => 820209989986969278,
            (Position { file: File::A, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::King }) => 4911989311424954821,
            (Position { file: File::A, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::King }) => 9547543661412095041,
            (Position { file: File::A, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::King }) => 11064473067513184664,
            (Position { file: File::A, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::King }) => 7728512882844893695,
            (Position { file: File::B, rank: Rank::One }, Piece { color: Color::White, kind: Kind::King }) => 1906753123355984684,
            (Position { file: File::B, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::King }) => 10529439654191563434,
            (Position { file: File::B, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::King }) => 11950196255679845066,
            (Position { file: File::B, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::King }) => 13226032676256874689,
            (Position { file: File::B, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::King }) => 6050214786564624032,
            (Position { file: File::B, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::King }) => 12319880317233005356,
            (Position { file: File::B, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::King }) => 794563585345847386,
            (Position { file: File::B, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::King }) => 14489178812798992982,
            (Position { file: File::C, rank: Rank::One }, Piece { color: Color::White, kind: Kind::King }) => 720768377247970158,
            (Position { file: File::C, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::King }) => 5527713357668286153,
            (Position { file: File::C, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::King }) => 2218245879901454355,
            (Position { file: File::C, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::King }) => 13624904729963266771,
            (Position { file: File::C, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::King }) => 1087966903860723728,
            (Position { file: File::C, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::King }) => 8190442843232272693,
            (Position { file: File::C, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::King }) => 2616909580449349644,
            (Position { file: File::C, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::King }) => 4038973557097627619,
            (Position { file: File::D, rank: Rank::One }, Piece { color: Color::White, kind: Kind::King }) => 16799262049950499957,
            (Position { file: File::D, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::King }) => 17452851026091573256,
            (Position { file: File::D, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::King }) => 281506235482636317,
            (Position { file: File::D, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::King }) => 179514497508048963,
            (Position { file: File::D, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::King }) => 5400850138392429575,
            (Position { file: File::D, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::King }) => 3308951699439223296,
            (Position { file: File::D, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::King }) => 16270296373151138848,
            (Position { file: File::D, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::King }) => 10325074424369254933,
            (Position { file: File::E, rank: Rank::One }, Piece { color: Color::White, kind: Kind::King }) => 14452805662482104150,
            (Position { file: File::E, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::King }) => 899114200410882669,
            (Position { file: File::E, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::King }) => 8106832092110782164,
            (Position { file: File::E, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::King }) => 1381102225006264317,
            (Position { file: File::E, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::King }) => 9477808635363983774,
            (Position { file: File::E, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::King }) => 13691951085009247525,
            (Position { file: File::E, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::King }) => 16693768689563992882,
            (Position { file: File::E, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::King }) => 14861304713583788687,
            (Position { file: File::F, rank: Rank::One }, Piece { color: Color::White, kind: Kind::King }) => 5862527915480056826,
            (Position { file: File::F, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::King }) => 145678062250360830,
            (Position { file: File::F, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::King }) => 14867280159848792745,
            (Position { file: File::F, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::King }) => 12785473669975507392,
            (Position { file: File::F, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::King }) => 12149665712410984520,
            (Position { file: File::F, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::King }) => 17980122372652489710,
            (Position { file: File::F, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::King }) => 2300181705446654748,
            (Position { file: File::F, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::King }) => 15086838368329278635,
            (Position { file: File::G, rank: Rank::One }, Piece { color: Color::White, kind: Kind::King }) => 1667185547006705788,
            (Position { file: File::G, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::King }) => 6291971778327609446,
            (Position { file: File::G, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::King }) => 17966130885325488943,
            (Position { file: File::G, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::King }) => 1365273578303389710,
            (Position { file: File::G, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::King }) => 850652606941364278,
            (Position { file: File::G, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::King }) => 12731666869447033186,
            (Position { file: File::G, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::King }) => 11578575065091932476,
            (Position { file: File::G, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::King }) => 8776700914154078131,
            (Position { file: File::H, rank: Rank::One }, Piece { color: Color::White, kind: Kind::King }) => 11795499719885586957,
            (Position { file: File::H, rank: Rank::Two }, Piece { color: Color::White, kind: Kind::King }) => 6989088383997837527,
            (Position { file: File::H, rank: Rank::Three }, Piece { color: Color::White, kind: Kind::King }) => 11136314594631352526,
            (Position { file: File::H, rank: Rank::Four }, Piece { color: Color::White, kind: Kind::King }) => 13368847107717333849,
            (Position { file: File::H, rank: Rank::Five }, Piece { color: Color::White, kind: Kind::King }) => 4153736264362754717,
            (Position { file: File::H, rank: Rank::Six }, Piece { color: Color::White, kind: Kind::King }) => 3466392036435820574,
            (Position { file: File::H, rank: Rank::Seven }, Piece { color: Color::White, kind: Kind::King }) => 17070582143425446773,
            (Position { file: File::H, rank: Rank::Eight }, Piece { color: Color::White, kind: Kind::King }) => 7719205491145966421,
        }
    }
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

#[cfg(test)]
mod tests {
    use crate::piece::Piece;
    use crate::color::Color;
    use crate::kind::Kind;
    use super::ALL_POSITIONS;

    #[test]
    fn it_doesnt_have_duplicate_position_hashes() {
        let mut used_hashes = Vec::new();
        let pieces = vec![
            Piece { color: Color::White, kind: Kind::Pawn },
            Piece { color: Color::White, kind: Kind::Knight },
            Piece { color: Color::White, kind: Kind::Bishop },
            Piece { color: Color::White, kind: Kind::Rook },
            Piece { color: Color::White, kind: Kind::Queen },
            Piece { color: Color::White, kind: Kind::King },
            Piece { color: Color::Black, kind: Kind::Pawn },
            Piece { color: Color::Black, kind: Kind::Knight },
            Piece { color: Color::Black, kind: Kind::Bishop },
            Piece { color: Color::Black, kind: Kind::Rook },
            Piece { color: Color::Black, kind: Kind::Queen },
            Piece { color: Color::Black, kind: Kind::King },
        ];
        let mut has_dupe = false;
        for pos in ALL_POSITIONS.iter() {
            for piece in &pieces {
                let hash = pos.hash(piece);
                if used_hashes.contains(&hash) {
                    has_dupe = true;
                    break
                }
                used_hashes.push(hash);
            }
        }
        assert_eq!(has_dupe, false);
    }
}