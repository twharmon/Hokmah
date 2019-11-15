use crate::color::Color;
use crate::direction::Direction;
use crate::kind::Kind;
use crate::piece::Piece;

pub static ALL_POSITIONS: [Position; 64] = [
    Position(4, 7),
    Position(4, 0),
    Position(0, 7),
    Position(1, 7),
    Position(2, 7),
    Position(3, 7),
    Position(5, 7),
    Position(6, 7),
    Position(7, 7),
    Position(0, 0),
    Position(1, 0),
    Position(2, 0),
    Position(3, 0),
    Position(5, 0),
    Position(6, 0),
    Position(7, 0),
    Position(0, 1),
    Position(1, 1),
    Position(2, 1),
    Position(3, 1),
    Position(4, 1),
    Position(5, 1),
    Position(6, 1),
    Position(7, 1),
    Position(0, 6),
    Position(1, 6),
    Position(2, 6),
    Position(3, 6),
    Position(4, 6),
    Position(5, 6),
    Position(6, 6),
    Position(7, 6),
    Position(0, 2),
    Position(1, 2),
    Position(2, 2),
    Position(3, 2),
    Position(4, 2),
    Position(5, 2),
    Position(6, 2),
    Position(7, 2),
    Position(0, 3),
    Position(1, 3),
    Position(2, 3),
    Position(3, 3),
    Position(4, 3),
    Position(5, 3),
    Position(6, 3),
    Position(7, 3),
    Position(0, 4),
    Position(1, 4),
    Position(2, 4),
    Position(3, 4),
    Position(4, 4),
    Position(5, 4),
    Position(6, 4),
    Position(7, 4),
    Position(0, 5),
    Position(1, 5),
    Position(2, 5),
    Position(3, 5),
    Position(4, 5),
    Position(5, 5),
    Position(6, 5),
    Position(7, 5),
];

pub static CENTER_POSITIONS: [Position; 4] = [
    Position(3, 3),
    Position(3, 4),
    Position(4, 3),
    Position(4, 4),
];

#[derive(Copy, Clone, PartialEq)]
pub struct Position(pub usize, pub usize);

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = match self.0 {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            _ => "h",
        };
        write!(f, "{}{}", file, self.1 + 1)
    }
}

impl Position {
    pub fn step(&self, file_dir: Direction, rank_dir: Direction) -> Option<Self> {
        let mut file = self.0;
        let mut rank = self.1;

        match file_dir {
            Direction::Inc => {
                if file == 7 {
                    return None;
                } else {
                    file += 1;
                }
            }
            Direction::None => (),
            Direction::Dec => {
                if file == 0 {
                    return None;
                } else {
                    file -= 1;
                }
            }
        };

        match rank_dir {
            Direction::Inc => {
                if rank == 7 {
                    return None;
                } else {
                    rank += 1;
                }
            }
            Direction::None => (),
            Direction::Dec => {
                if rank == 0 {
                    return None;
                } else {
                    rank -= 1;
                }
            }
        };

        Some(Position(file, rank))
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
        (self.0 + self.1) % 2 == 0
    }

    pub fn hash(&self, piece: &Piece) -> u64 {
        match (self, piece) {
            (
                Position(0, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 1047498355768714921,
            (
                Position(0, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 11732260526934782710,
            (
                Position(0, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 7601703913300951192,
            (
                Position(0, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 4020548701448569424,
            (
                Position(0, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 4208089238702391346,
            (
                Position(0, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 13248906572563743535,
            (
                Position(0, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 13698386655415632463,
            (
                Position(0, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 8689285492023231471,
            (
                Position(1, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 11441732661102516205,
            (
                Position(1, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 16153402082206941240,
            (
                Position(1, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 6695713994235183488,
            (
                Position(1, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 1216422059207295596,
            (
                Position(1, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 13919116737132888424,
            (
                Position(1, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 847738471631028242,
            (
                Position(1, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 1677008911165764412,
            (
                Position(1, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 10864327539979977548,
            (
                Position(2, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 2166697046759702555,
            (
                Position(2, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 8196009859582263478,
            (
                Position(2, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 5826886979777724226,
            (
                Position(2, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 9763081376776585723,
            (
                Position(2, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 12710097938823512546,
            (
                Position(2, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 479030449126821141,
            (
                Position(2, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 9755960458438772319,
            (
                Position(2, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 5561651476276707727,
            (
                Position(3, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 13849642823621542885,
            (
                Position(3, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 12131126567682186580,
            (
                Position(3, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 11845277441658454638,
            (
                Position(3, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 8140715707584217971,
            (
                Position(3, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 18398996986704435861,
            (
                Position(3, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 11199957866726487754,
            (
                Position(3, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 6434807980977291404,
            (
                Position(3, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 8005121524170228371,
            (
                Position(4, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 5289165694317887290,
            (
                Position(4, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 9764233909775634340,
            (
                Position(4, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 2719966199730405436,
            (
                Position(4, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 2972212025616236580,
            (
                Position(4, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 15006029207195093119,
            (
                Position(4, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 18045639577607078173,
            (
                Position(4, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 4056206245789434916,
            (
                Position(4, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 4994414715997258105,
            (
                Position(5, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 18169993363456553615,
            (
                Position(5, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 10148342116481237159,
            (
                Position(5, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 9938649641153253723,
            (
                Position(5, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 4634940835383738101,
            (
                Position(5, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 9227635629414679838,
            (
                Position(5, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 16684122648283716068,
            (
                Position(5, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 8014416997334909783,
            (
                Position(5, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 16418907155035324644,
            (
                Position(6, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 1341826483821635223,
            (
                Position(6, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 3442525457222729786,
            (
                Position(6, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 6513440693204025868,
            (
                Position(6, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 1200290786531763847,
            (
                Position(6, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 4219109325920012662,
            (
                Position(6, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 17924346316733809537,
            (
                Position(6, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 5901406599393575262,
            (
                Position(6, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 15673069050244628532,
            (
                Position(7, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 167619164674981913,
            (
                Position(7, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 5373044339729498117,
            (
                Position(7, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 11976424117717959435,
            (
                Position(7, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 13778430350298448577,
            (
                Position(7, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 16296674249806973321,
            (
                Position(7, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 18196983788347589823,
            (
                Position(7, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 1297836793550341290,
            (
                Position(7, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Rook,
                },
            ) => 10953027208871936694,
            (
                Position(0, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 10721156510269833133,
            (
                Position(0, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 13313123252831257619,
            (
                Position(0, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 5460597969612964820,
            (
                Position(0, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 1695524561564467219,
            (
                Position(0, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 14618828499482313497,
            (
                Position(0, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 6025582148394686499,
            (
                Position(0, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 1471189711648194627,
            (
                Position(0, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 15153087749243798237,
            (
                Position(1, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 7196999594654302891,
            (
                Position(1, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 8393311186757616715,
            (
                Position(1, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 4752665674218734412,
            (
                Position(1, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 16571198410359574761,
            (
                Position(1, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 17853468812205097658,
            (
                Position(1, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 32351006215477663,
            (
                Position(1, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 8931609745886806254,
            (
                Position(1, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 11567849887508726341,
            (
                Position(2, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 1172543893345298441,
            (
                Position(2, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 11538257811947389488,
            (
                Position(2, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 16146985504145932671,
            (
                Position(2, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 3702049448789016633,
            (
                Position(2, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 13789421012695122505,
            (
                Position(2, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 5399217466704396983,
            (
                Position(2, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 4169448689250599939,
            (
                Position(2, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 11048815347623789522,
            (
                Position(3, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 11136214998634725183,
            (
                Position(3, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 15029858531861856779,
            (
                Position(3, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 15717207823555232566,
            (
                Position(3, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 266910176103733660,
            (
                Position(3, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 15896917127894925118,
            (
                Position(3, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 458472848010038516,
            (
                Position(3, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 6897067502795227133,
            (
                Position(3, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 17032921675445839766,
            (
                Position(4, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 765576880270920944,
            (
                Position(4, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 16170187157997093701,
            (
                Position(4, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 237984987845387918,
            (
                Position(4, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 1419487674016517390,
            (
                Position(4, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 3341466337036730480,
            (
                Position(4, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 6342516688633725767,
            (
                Position(4, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 3955628887790059558,
            (
                Position(4, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 3581109887062507571,
            (
                Position(5, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 775274358938533989,
            (
                Position(5, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 14560318161488905448,
            (
                Position(5, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 1485625678698119279,
            (
                Position(5, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 1437623483520516385,
            (
                Position(5, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 2335766248228389207,
            (
                Position(5, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 212231200431931418,
            (
                Position(5, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 18109307262269915747,
            (
                Position(5, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 224418294688382980,
            (
                Position(6, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 13383504010032447821,
            (
                Position(6, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 6264700449689962393,
            (
                Position(6, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 10417646946876391361,
            (
                Position(6, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 2176954090087514868,
            (
                Position(6, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 13648190492158984360,
            (
                Position(6, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 5268583699934347671,
            (
                Position(6, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 5078611386557268688,
            (
                Position(6, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 11277259770425246178,
            (
                Position(7, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 4984530359866622504,
            (
                Position(7, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 13867359456343364794,
            (
                Position(7, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 12015615964876898585,
            (
                Position(7, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 15908500143591139103,
            (
                Position(7, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 10667564337847075972,
            (
                Position(7, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 10805824433231495232,
            (
                Position(7, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 10390432118533521108,
            (
                Position(7, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Bishop,
                },
            ) => 14003874850112799581,
            (
                Position(0, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 6596669256660681224,
            (
                Position(0, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 12335186532703928296,
            (
                Position(0, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 534430657611300860,
            (
                Position(0, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 17317169590416966201,
            (
                Position(0, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 9397263545999032927,
            (
                Position(0, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 7389196486342345144,
            (
                Position(0, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 5764048366464716911,
            (
                Position(0, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 16850940964766745321,
            (
                Position(1, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 1409576861746331633,
            (
                Position(1, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 8839497630988894225,
            (
                Position(1, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 1402362187278922562,
            (
                Position(1, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 15315537375317197811,
            (
                Position(1, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 13589743752561295133,
            (
                Position(1, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 1664268298274576841,
            (
                Position(1, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 2561628407842472726,
            (
                Position(1, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 11404448105840509373,
            (
                Position(2, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 1994472886869426223,
            (
                Position(2, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 18183016331362697283,
            (
                Position(2, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 17878482310894977565,
            (
                Position(2, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 10156910473157542644,
            (
                Position(2, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 627584931720698340,
            (
                Position(2, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 11711249491083395745,
            (
                Position(2, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 10228778994145165183,
            (
                Position(2, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 12505011541042528363,
            (
                Position(3, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 7703788869245731521,
            (
                Position(3, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 15018936713495971162,
            (
                Position(3, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 13640219449469436728,
            (
                Position(3, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 1383409833624993840,
            (
                Position(3, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 970885689497354232,
            (
                Position(3, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 8472983173695799678,
            (
                Position(3, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 17877154551820199184,
            (
                Position(3, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 11034437158621413641,
            (
                Position(4, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 16392161423020524970,
            (
                Position(4, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 969112060848137899,
            (
                Position(4, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 1625044452750288560,
            (
                Position(4, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 7950062397077586410,
            (
                Position(4, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 622047214349110514,
            (
                Position(4, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 9411464026434568433,
            (
                Position(4, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 1389262794294663665,
            (
                Position(4, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 16159183580633768298,
            (
                Position(5, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 615129438085972223,
            (
                Position(5, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 14793190822941557148,
            (
                Position(5, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 17294313681433133495,
            (
                Position(5, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 12330368416916448877,
            (
                Position(5, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 3660248945868669106,
            (
                Position(5, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 12282236994762834742,
            (
                Position(5, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 6331263973102453670,
            (
                Position(5, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 17172464497172939976,
            (
                Position(6, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 382652522029908521,
            (
                Position(6, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 10259715420239954757,
            (
                Position(6, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 16434759097640813469,
            (
                Position(6, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 16016185719710024846,
            (
                Position(6, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 11886571615179244727,
            (
                Position(6, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 15307721750314222197,
            (
                Position(6, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 16946565564048867304,
            (
                Position(6, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 1778869677548634337,
            (
                Position(7, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 11809451577549259888,
            (
                Position(7, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 12804903900159148942,
            (
                Position(7, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 8891713305047994176,
            (
                Position(7, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 10638594650291569146,
            (
                Position(7, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 5552270464543359978,
            (
                Position(7, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 12936319393926742179,
            (
                Position(7, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 6708830931731351260,
            (
                Position(7, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Knight,
                },
            ) => 17940437690881147195,
            (
                Position(0, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 12606257831608326627,
            (
                Position(0, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 2973089779492585511,
            (
                Position(0, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 6713041713373381263,
            (
                Position(0, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 4642095692553126406,
            (
                Position(0, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 6846297098116137930,
            (
                Position(0, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 8478051497098084429,
            (
                Position(0, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 17948161588267581619,
            (
                Position(0, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 27614483210200687,
            (
                Position(1, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 14267881670844613764,
            (
                Position(1, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 3932271528489517304,
            (
                Position(1, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 1262497702995610827,
            (
                Position(1, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 24714488220693358,
            (
                Position(1, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 4956207146493542462,
            (
                Position(1, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 1454521598554931234,
            (
                Position(1, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 1298110069434482788,
            (
                Position(1, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 15935848944848863485,
            (
                Position(2, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 17661382893882049713,
            (
                Position(2, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 8141839094935192771,
            (
                Position(2, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 7685298266237305459,
            (
                Position(2, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 12006419641032245827,
            (
                Position(2, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 841617430269657195,
            (
                Position(2, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 14191224046573584286,
            (
                Position(2, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 3655514951310639779,
            (
                Position(2, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 12024024467165413846,
            (
                Position(3, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 9667951046625853730,
            (
                Position(3, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 6845618321484677188,
            (
                Position(3, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 8795735337331589524,
            (
                Position(3, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 18372498958338494804,
            (
                Position(3, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 9177307211230085120,
            (
                Position(3, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 17214993117309043461,
            (
                Position(3, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 13641820793666011913,
            (
                Position(3, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 4377384020443398327,
            (
                Position(4, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 8934873401340723548,
            (
                Position(4, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 6481730949657657250,
            (
                Position(4, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 10789214317229113280,
            (
                Position(4, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 1769307057418141771,
            (
                Position(4, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 1385619804752183352,
            (
                Position(4, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 339513458192023550,
            (
                Position(4, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 6808952104074346979,
            (
                Position(4, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 4370259730556256284,
            (
                Position(5, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 474460858185941502,
            (
                Position(5, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 7304624401280926439,
            (
                Position(5, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 16847917226185982588,
            (
                Position(5, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 12744937416448543827,
            (
                Position(5, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 940736539497857265,
            (
                Position(5, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 11933214023710933494,
            (
                Position(5, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 1567735319422304322,
            (
                Position(5, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 8469387689068593253,
            (
                Position(6, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 11655411471671624583,
            (
                Position(6, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 5548953508545298478,
            (
                Position(6, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 13283143788399694405,
            (
                Position(6, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 4602349433835749492,
            (
                Position(6, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 16734977904336730676,
            (
                Position(6, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 7654354229460468555,
            (
                Position(6, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 4832675576298865970,
            (
                Position(6, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 4256801396268990530,
            (
                Position(7, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 7564935488553878795,
            (
                Position(7, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 13010552934360941057,
            (
                Position(7, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 18205789283053208941,
            (
                Position(7, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 6772136739629695485,
            (
                Position(7, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 700640517244426625,
            (
                Position(7, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 14895214189055312408,
            (
                Position(7, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 937790097443520540,
            (
                Position(7, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Pawn,
                },
            ) => 2864627742006051132,
            (
                Position(0, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 6037874241716617793,
            (
                Position(0, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 1377958827348145846,
            (
                Position(0, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 10314208547952394599,
            (
                Position(0, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 16210149548058739720,
            (
                Position(0, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 1729746672429931297,
            (
                Position(0, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 6088612855047782853,
            (
                Position(0, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 12859102332985541446,
            (
                Position(0, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 16486279536753778345,
            (
                Position(1, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 2742491464951922793,
            (
                Position(1, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 2153141008987062364,
            (
                Position(1, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 12622579892744094838,
            (
                Position(1, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 2603116189899030544,
            (
                Position(1, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 10880415229045375287,
            (
                Position(1, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 13546258376920448282,
            (
                Position(1, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 7381790463780454535,
            (
                Position(1, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 10548434348921586466,
            (
                Position(2, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 2978990699485069361,
            (
                Position(2, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 5613823985914479112,
            (
                Position(2, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 16390895365445911859,
            (
                Position(2, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 412463915801247740,
            (
                Position(2, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 18133687012765467552,
            (
                Position(2, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 10760994646927606331,
            (
                Position(2, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 15380551390493416578,
            (
                Position(2, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 17691775596782065738,
            (
                Position(3, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 6189324190256988314,
            (
                Position(3, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 7620142414061109725,
            (
                Position(3, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 2629322181963677736,
            (
                Position(3, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 60308809354471015,
            (
                Position(3, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 10503438450462556813,
            (
                Position(3, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 6595750868098744474,
            (
                Position(3, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 2849348052253147704,
            (
                Position(3, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 121195764815002346,
            (
                Position(4, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 1718029801071470279,
            (
                Position(4, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 3811417075834945539,
            (
                Position(4, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 17329176626759467364,
            (
                Position(4, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 9535157916328264960,
            (
                Position(4, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 11009302065112416424,
            (
                Position(4, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 5404870425349983,
            (
                Position(4, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 13681453897175007230,
            (
                Position(4, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 9908046405736333484,
            (
                Position(5, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 1979378640919462893,
            (
                Position(5, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 17810802053035852775,
            (
                Position(5, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 3767355285518156886,
            (
                Position(5, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 14181269493497463864,
            (
                Position(5, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 12380701477960683928,
            (
                Position(5, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 1063848401193402463,
            (
                Position(5, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 5934265104957702890,
            (
                Position(5, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 9333908547840246157,
            (
                Position(6, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 6385625588916295279,
            (
                Position(6, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 7827298805690139942,
            (
                Position(6, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 6534181713046667780,
            (
                Position(6, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 8543256526649819264,
            (
                Position(6, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 105937278757293265,
            (
                Position(6, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 14091114977448428830,
            (
                Position(6, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 2971012630229025879,
            (
                Position(6, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 16633464372315816166,
            (
                Position(7, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 11785703457829159263,
            (
                Position(7, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 5122406639793603933,
            (
                Position(7, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 13727954971217887772,
            (
                Position(7, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 2496070765770703665,
            (
                Position(7, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 1346695946372644930,
            (
                Position(7, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 1801549880708877512,
            (
                Position(7, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 3639169285034606642,
            (
                Position(7, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::Queen,
                },
            ) => 17582716925723542418,
            (
                Position(0, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 7355374061267976110,
            (
                Position(0, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 9118810933479604601,
            (
                Position(0, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 5489106575249375991,
            (
                Position(0, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 9623243812918788933,
            (
                Position(0, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 7116850681662669513,
            (
                Position(0, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 4288723633322328450,
            (
                Position(0, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 16351565967664356211,
            (
                Position(0, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 11615148798971478165,
            (
                Position(1, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 9191844687684567113,
            (
                Position(1, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 11827230139404517931,
            (
                Position(1, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 3950819160073699392,
            (
                Position(1, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 18184789293862486206,
            (
                Position(1, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 1733485015829846352,
            (
                Position(1, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 7384437860146872239,
            (
                Position(1, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 10222167261360167542,
            (
                Position(1, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 7393559026358813206,
            (
                Position(2, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 13502362149002936749,
            (
                Position(2, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 439849628007923778,
            (
                Position(2, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 15966746660403413778,
            (
                Position(2, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 16751838734760018764,
            (
                Position(2, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 8073766813936124744,
            (
                Position(2, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 8906713224926220345,
            (
                Position(2, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 7570997392510556071,
            (
                Position(2, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 4985217129432221331,
            (
                Position(3, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 1069240591329106392,
            (
                Position(3, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 1392588553687190793,
            (
                Position(3, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 12445771405021151478,
            (
                Position(3, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 14163176106244447355,
            (
                Position(3, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 6246733115432108263,
            (
                Position(3, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 2078862573259718738,
            (
                Position(3, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 896929337662505818,
            (
                Position(3, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 946413682674788471,
            (
                Position(4, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 5748011498819551780,
            (
                Position(4, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 16316889247825002137,
            (
                Position(4, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 16331730902453322274,
            (
                Position(4, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 16597235365396349275,
            (
                Position(4, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 10594017820381544919,
            (
                Position(4, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 6897844174616265265,
            (
                Position(4, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 268968594585419780,
            (
                Position(4, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 82342228665145757,
            (
                Position(5, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 1148399109343595742,
            (
                Position(5, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 5541810797543424394,
            (
                Position(5, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 14912498941340681495,
            (
                Position(5, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 1710278820050763815,
            (
                Position(5, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 1217436656325152235,
            (
                Position(5, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 17533584041750562382,
            (
                Position(5, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 15377262703970094183,
            (
                Position(5, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 7086740525671252290,
            (
                Position(6, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 1243995787472824214,
            (
                Position(6, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 272342098122899462,
            (
                Position(6, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 13046752628128612619,
            (
                Position(6, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 10359733032331515603,
            (
                Position(6, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 18286248254028906443,
            (
                Position(6, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 5134237561002131220,
            (
                Position(6, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 15136741082240385252,
            (
                Position(6, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 1619453256163917819,
            (
                Position(7, 0),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 9433902340043178852,
            (
                Position(7, 1),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 9963739701930099314,
            (
                Position(7, 2),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 4856488833492976881,
            (
                Position(7, 3),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 12447585543372407360,
            (
                Position(7, 4),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 16546659123303816936,
            (
                Position(7, 5),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 1143526109457521690,
            (
                Position(7, 6),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 9837509913020999903,
            (
                Position(7, 7),
                Piece {
                    color: Color::Black,
                    kind: Kind::King,
                },
            ) => 11114508225330807921,
            (
                Position(0, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 992760245698691112,
            (
                Position(0, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 246850625407549449,
            (
                Position(0, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 16161663241347422356,
            (
                Position(0, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 1830281170521161730,
            (
                Position(0, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 927459064714100754,
            (
                Position(0, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 12255281956106797152,
            (
                Position(0, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 11366537155368196720,
            (
                Position(0, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 14080694489140167182,
            (
                Position(1, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 14505246646230581240,
            (
                Position(1, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 7674614771311706890,
            (
                Position(1, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 8614871309559529583,
            (
                Position(1, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 2892633948137985138,
            (
                Position(1, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 411288202863706138,
            (
                Position(1, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 14412106449305993216,
            (
                Position(1, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 7415050664750875492,
            (
                Position(1, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 14310483266770043301,
            (
                Position(2, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 624226934733144545,
            (
                Position(2, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 14273898224241607961,
            (
                Position(2, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 2517518664215822347,
            (
                Position(2, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 10414704116889552564,
            (
                Position(2, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 15739111959196336989,
            (
                Position(2, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 4137448769123778669,
            (
                Position(2, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 356317732485988350,
            (
                Position(2, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 12509521252408361826,
            (
                Position(3, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 18267357922709215596,
            (
                Position(3, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 7046518650865451259,
            (
                Position(3, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 5800968217945965214,
            (
                Position(3, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 13950335999709024689,
            (
                Position(3, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 3902857814330048528,
            (
                Position(3, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 15420745201982898751,
            (
                Position(3, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 11872709316173627150,
            (
                Position(3, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 4349253500778053693,
            (
                Position(4, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 11749942447726133908,
            (
                Position(4, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 1737445132872555734,
            (
                Position(4, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 9115279877656805478,
            (
                Position(4, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 1298449071203156715,
            (
                Position(4, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 6009482918752158161,
            (
                Position(4, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 17681343208929362713,
            (
                Position(4, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 889556407394133621,
            (
                Position(4, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 14840101958987022333,
            (
                Position(5, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 17108270498627715277,
            (
                Position(5, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 3890324833472348723,
            (
                Position(5, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 3647501573128454282,
            (
                Position(5, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 14060437331723158472,
            (
                Position(5, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 16939212751836807990,
            (
                Position(5, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 7440655584732381620,
            (
                Position(5, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 17974738205945102482,
            (
                Position(5, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 6142680274191778677,
            (
                Position(6, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 172892709775107694,
            (
                Position(6, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 10615670571586814561,
            (
                Position(6, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 6952416080618848164,
            (
                Position(6, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 1357444428448399454,
            (
                Position(6, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 9769598010395722977,
            (
                Position(6, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 14088663732238418561,
            (
                Position(6, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 15733591087974973796,
            (
                Position(6, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 11886348233930179105,
            (
                Position(7, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 4084149955852239293,
            (
                Position(7, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 11474458430702354326,
            (
                Position(7, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 10711147892145394155,
            (
                Position(7, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 1125462773231675388,
            (
                Position(7, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 12048504054175761223,
            (
                Position(7, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 10869333672255816677,
            (
                Position(7, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 13091256954695713426,
            (
                Position(7, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Rook,
                },
            ) => 8639502354296603156,
            (
                Position(0, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 11481449924495671926,
            (
                Position(0, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 9171590545334678555,
            (
                Position(0, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 3287349336790273680,
            (
                Position(0, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 12634111325883073384,
            (
                Position(0, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 2007204853561950268,
            (
                Position(0, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 17040922993359847291,
            (
                Position(0, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 16468570231246165976,
            (
                Position(0, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 3949170476747587665,
            (
                Position(1, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 14357539465335935166,
            (
                Position(1, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 468811395643736060,
            (
                Position(1, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 2890591334706446372,
            (
                Position(1, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 9929097208685134208,
            (
                Position(1, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 9769766819790324445,
            (
                Position(1, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 1236081915522449483,
            (
                Position(1, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 6620441445377507390,
            (
                Position(1, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 13470239225307726737,
            (
                Position(2, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 1142039603237512828,
            (
                Position(2, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 13462713480532467372,
            (
                Position(2, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 5301846104222997576,
            (
                Position(2, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 5150833709770867921,
            (
                Position(2, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 935230497510077276,
            (
                Position(2, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 2055643138243952667,
            (
                Position(2, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 164461634856471766,
            (
                Position(2, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 1301052877516992122,
            (
                Position(3, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 8022246602456433501,
            (
                Position(3, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 18369262099185533965,
            (
                Position(3, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 15847020980988805938,
            (
                Position(3, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 4961327748237754879,
            (
                Position(3, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 3905707361922187321,
            (
                Position(3, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 16482235202274329625,
            (
                Position(3, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 959617366484425120,
            (
                Position(3, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 4498078756149133340,
            (
                Position(4, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 279161619278200833,
            (
                Position(4, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 11414082082557657375,
            (
                Position(4, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 18394166522885964982,
            (
                Position(4, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 6090299209532047302,
            (
                Position(4, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 13402352839083164806,
            (
                Position(4, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 15794173302492103759,
            (
                Position(4, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 1313334418104280745,
            (
                Position(4, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 4341995766257025954,
            (
                Position(5, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 7348599678071472732,
            (
                Position(5, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 17453819223389241318,
            (
                Position(5, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 5714416788616249538,
            (
                Position(5, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 6405681368201691489,
            (
                Position(5, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 1588129162026025711,
            (
                Position(5, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 15812265976780554843,
            (
                Position(5, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 15049797200273998320,
            (
                Position(5, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 12238773227102405936,
            (
                Position(6, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 3527974402807374755,
            (
                Position(6, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 15823862942530535503,
            (
                Position(6, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 5352852066383432432,
            (
                Position(6, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 14531104376577065418,
            (
                Position(6, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 5513036470870344991,
            (
                Position(6, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 3305364976545300510,
            (
                Position(6, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 9348833640798224774,
            (
                Position(6, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 2149729542233653289,
            (
                Position(7, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 15306616311631577790,
            (
                Position(7, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 16533080038736658462,
            (
                Position(7, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 17497404349645586888,
            (
                Position(7, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 13204166924738494148,
            (
                Position(7, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 11402376003163524015,
            (
                Position(7, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 3627534042536083545,
            (
                Position(7, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 16470139526396772409,
            (
                Position(7, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Bishop,
                },
            ) => 13897513244907536219,
            (
                Position(0, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 6247124996838139444,
            (
                Position(0, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 4618746721323713743,
            (
                Position(0, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 4105169186496446516,
            (
                Position(0, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 16515823229508518826,
            (
                Position(0, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 446076170945953856,
            (
                Position(0, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 11727801642207151608,
            (
                Position(0, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 4806925258789487501,
            (
                Position(0, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 13032295106785313782,
            (
                Position(1, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 16141230363933412298,
            (
                Position(1, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 6965019563364712921,
            (
                Position(1, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 5393662871399825574,
            (
                Position(1, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 2679509879839785610,
            (
                Position(1, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 591718347885563199,
            (
                Position(1, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 15140973330128962883,
            (
                Position(1, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 13942191612139405108,
            (
                Position(1, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 7125830165493449754,
            (
                Position(2, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 9830520407797534379,
            (
                Position(2, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 12149846904196364039,
            (
                Position(2, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 12567281686296926106,
            (
                Position(2, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 17545467838301474004,
            (
                Position(2, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 1683114511260713459,
            (
                Position(2, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 55886222259231131,
            (
                Position(2, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 16186310967737456461,
            (
                Position(2, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 3851093129016901668,
            (
                Position(3, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 3850982031097856186,
            (
                Position(3, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 1889261486790934558,
            (
                Position(3, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 9923952086972634023,
            (
                Position(3, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 5404239318560539243,
            (
                Position(3, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 16024384275407176740,
            (
                Position(3, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 14871436927982109674,
            (
                Position(3, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 15158260981517255205,
            (
                Position(3, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 6699789707745698078,
            (
                Position(4, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 8571885679133327220,
            (
                Position(4, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 12415623483382331322,
            (
                Position(4, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 1349313174459279736,
            (
                Position(4, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 7394486026330178904,
            (
                Position(4, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 17809482424334156464,
            (
                Position(4, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 13793239801261982812,
            (
                Position(4, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 2519814315645659897,
            (
                Position(4, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 3411709839968043502,
            (
                Position(5, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 7780031024150348989,
            (
                Position(5, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 8745038685830906565,
            (
                Position(5, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 13400283712818512916,
            (
                Position(5, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 17527777920276234299,
            (
                Position(5, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 12009974834571248797,
            (
                Position(5, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 10423478906873976603,
            (
                Position(5, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 15214852278063202341,
            (
                Position(5, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 5512910142997266230,
            (
                Position(6, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 14714384692820312913,
            (
                Position(6, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 10857588607443534348,
            (
                Position(6, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 283048523878891520,
            (
                Position(6, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 11396105484055347960,
            (
                Position(6, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 12475548713932036748,
            (
                Position(6, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 7461595903942459203,
            (
                Position(6, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 1756683710878030230,
            (
                Position(6, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 13762773691265974627,
            (
                Position(7, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 876265008755900431,
            (
                Position(7, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 4666130252232131333,
            (
                Position(7, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 5876887163676656208,
            (
                Position(7, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 17956357498264889864,
            (
                Position(7, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 1311554510212877537,
            (
                Position(7, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 223879349307179320,
            (
                Position(7, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 1880197525248409648,
            (
                Position(7, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Knight,
                },
            ) => 14816677134340194903,
            (
                Position(0, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 13357375623731872833,
            (
                Position(0, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 14185182745575039294,
            (
                Position(0, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 14997645229169836731,
            (
                Position(0, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 3907176232147484789,
            (
                Position(0, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 12026088125936697522,
            (
                Position(0, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 3907160770265219961,
            (
                Position(0, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 9886314558413344616,
            (
                Position(0, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 17243492196707992706,
            (
                Position(1, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 13743433930273456959,
            (
                Position(1, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 11785600619132223537,
            (
                Position(1, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 14383190667885029988,
            (
                Position(1, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 2669037375182602180,
            (
                Position(1, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 16256970777553797704,
            (
                Position(1, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 11742957370613957866,
            (
                Position(1, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 793549818445168661,
            (
                Position(1, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 12084487930131775540,
            (
                Position(2, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 2055370815842549872,
            (
                Position(2, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 209800720529804559,
            (
                Position(2, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 11091570218047111430,
            (
                Position(2, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 461190080544689468,
            (
                Position(2, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 16892615912908128305,
            (
                Position(2, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 1745384781069156473,
            (
                Position(2, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 2012969992458338317,
            (
                Position(2, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 8454712584682078711,
            (
                Position(3, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 13081675870500815192,
            (
                Position(3, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 8356235465690448952,
            (
                Position(3, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 17901385735102005635,
            (
                Position(3, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 18267217765041439369,
            (
                Position(3, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 391427799063619234,
            (
                Position(3, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 5573288432467182131,
            (
                Position(3, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 3329804229517771704,
            (
                Position(3, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 10730246366170186388,
            (
                Position(4, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 6782237162680091505,
            (
                Position(4, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 14034674657592869679,
            (
                Position(4, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 15393426800503685353,
            (
                Position(4, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 8425170936331764611,
            (
                Position(4, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 17277963191908827629,
            (
                Position(4, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 18145926295169532668,
            (
                Position(4, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 7715977990726746710,
            (
                Position(4, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 449420384524461670,
            (
                Position(5, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 10141192275833651987,
            (
                Position(5, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 8876027487482544957,
            (
                Position(5, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 17543850813179298442,
            (
                Position(5, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 15211023271769145693,
            (
                Position(5, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 18359478520167532344,
            (
                Position(5, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 16307996518038634927,
            (
                Position(5, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 959038920718496615,
            (
                Position(5, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 9100174134079392485,
            (
                Position(6, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 244377697484642761,
            (
                Position(6, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 13487975224037081301,
            (
                Position(6, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 18433891860817641203,
            (
                Position(6, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 2575115704663539730,
            (
                Position(6, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 3033045014683844692,
            (
                Position(6, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 16498999709466187670,
            (
                Position(6, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 13081529317626744485,
            (
                Position(6, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 17427317131984246401,
            (
                Position(7, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 16681782269064511695,
            (
                Position(7, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 18070145436932375214,
            (
                Position(7, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 15900435749787402555,
            (
                Position(7, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 2957019691203166396,
            (
                Position(7, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 14536238975619695544,
            (
                Position(7, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 5736813501171634397,
            (
                Position(7, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 16138238897736909751,
            (
                Position(7, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Pawn,
                },
            ) => 18008956193212465513,
            (
                Position(0, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 13457986100224066938,
            (
                Position(0, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 7177303587599417998,
            (
                Position(0, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 1654998972011800292,
            (
                Position(0, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 8994604645397561486,
            (
                Position(0, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 252588319462916167,
            (
                Position(0, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 1004898208106256866,
            (
                Position(0, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 14240100959085658952,
            (
                Position(0, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 10524750237099098675,
            (
                Position(1, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 17086621922130657360,
            (
                Position(1, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 7604912335475442785,
            (
                Position(1, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 4458709698639560727,
            (
                Position(1, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 9488156616034353569,
            (
                Position(1, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 5310338551647306112,
            (
                Position(1, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 10426219684419338262,
            (
                Position(1, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 6889213618223579552,
            (
                Position(1, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 6168392881888297930,
            (
                Position(2, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 10349029604462692851,
            (
                Position(2, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 13982185995412446879,
            (
                Position(2, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 1390713569393246280,
            (
                Position(2, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 17013829764131062901,
            (
                Position(2, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 13450617452467585331,
            (
                Position(2, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 9194851396727301590,
            (
                Position(2, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 2011837628625715225,
            (
                Position(2, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 1477387868717868625,
            (
                Position(3, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 9194617213628187302,
            (
                Position(3, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 4210075111320977456,
            (
                Position(3, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 6263811421524463745,
            (
                Position(3, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 5865938299901706808,
            (
                Position(3, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 17030713426600526967,
            (
                Position(3, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 14682076011789559819,
            (
                Position(3, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 1694889347497800542,
            (
                Position(3, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 5256383059351044634,
            (
                Position(4, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 1120955961827760595,
            (
                Position(4, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 13365700674805826630,
            (
                Position(4, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 11778949900929073295,
            (
                Position(4, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 1746598916784128482,
            (
                Position(4, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 772956901959794794,
            (
                Position(4, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 10887913460260143705,
            (
                Position(4, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 12915829041260397330,
            (
                Position(4, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 1248687861861554637,
            (
                Position(5, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 15898558896323695076,
            (
                Position(5, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 4477129589306425361,
            (
                Position(5, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 9484618280241988103,
            (
                Position(5, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 476842959146949493,
            (
                Position(5, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 3101966432192168936,
            (
                Position(5, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 9252059253001159613,
            (
                Position(5, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 4865744226022851536,
            (
                Position(5, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 15868350686228906921,
            (
                Position(6, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 4464520261110071362,
            (
                Position(6, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 11957928773065638481,
            (
                Position(6, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 2736916223739564159,
            (
                Position(6, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 7291084482520547684,
            (
                Position(6, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 1012218121098361779,
            (
                Position(6, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 11504714868984906425,
            (
                Position(6, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 15239033841587847329,
            (
                Position(6, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 6362228818995839222,
            (
                Position(7, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 15749515276225348223,
            (
                Position(7, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 3602592304159785150,
            (
                Position(7, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 14528451508422312455,
            (
                Position(7, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 17238632862119363124,
            (
                Position(7, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 11401273966094975394,
            (
                Position(7, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 2270346581981003869,
            (
                Position(7, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 16103440011847795386,
            (
                Position(7, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::Queen,
                },
            ) => 14528602815825183230,
            (
                Position(0, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 13406214860790825251,
            (
                Position(0, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 5140884564058571461,
            (
                Position(0, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 9486211459705733627,
            (
                Position(0, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 820209989986969278,
            (
                Position(0, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 4911989311424954821,
            (
                Position(0, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 9547543661412095041,
            (
                Position(0, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 11064473067513184664,
            (
                Position(0, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 7728512882844893695,
            (
                Position(1, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 1906753123355984684,
            (
                Position(1, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 10529439654191563434,
            (
                Position(1, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 11950196255679845066,
            (
                Position(1, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 13226032676256874689,
            (
                Position(1, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 6050214786564624032,
            (
                Position(1, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 12319880317233005356,
            (
                Position(1, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 794563585345847386,
            (
                Position(1, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 14489178812798992982,
            (
                Position(2, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 720768377247970158,
            (
                Position(2, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 5527713357668286153,
            (
                Position(2, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 2218245879901454355,
            (
                Position(2, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 13624904729963266771,
            (
                Position(2, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 1087966903860723728,
            (
                Position(2, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 8190442843232272693,
            (
                Position(2, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 2616909580449349644,
            (
                Position(2, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 4038973557097627619,
            (
                Position(3, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 16799262049950499957,
            (
                Position(3, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 17452851026091573256,
            (
                Position(3, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 281506235482636317,
            (
                Position(3, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 179514497508048963,
            (
                Position(3, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 5400850138392429575,
            (
                Position(3, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 3308951699439223296,
            (
                Position(3, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 16270296373151138848,
            (
                Position(3, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 10325074424369254933,
            (
                Position(4, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 14452805662482104150,
            (
                Position(4, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 899114200410882669,
            (
                Position(4, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 8106832092110782164,
            (
                Position(4, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 1381102225006264317,
            (
                Position(4, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 9477808635363983774,
            (
                Position(4, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 13691951085009247525,
            (
                Position(4, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 16693768689563992882,
            (
                Position(4, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 14861304713583788687,
            (
                Position(5, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 5862527915480056826,
            (
                Position(5, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 145678062250360830,
            (
                Position(5, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 14867280159848792745,
            (
                Position(5, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 12785473669975507392,
            (
                Position(5, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 12149665712410984520,
            (
                Position(5, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 17980122372652489710,
            (
                Position(5, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 2300181705446654748,
            (
                Position(5, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 15086838368329278635,
            (
                Position(6, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 1667185547006705788,
            (
                Position(6, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 6291971778327609446,
            (
                Position(6, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 17966130885325488943,
            (
                Position(6, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 1365273578303389710,
            (
                Position(6, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 850652606941364278,
            (
                Position(6, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 12731666869447033186,
            (
                Position(6, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 11578575065091932476,
            (
                Position(6, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 8776700914154078131,
            (
                Position(7, 0),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 11795499719885586957,
            (
                Position(7, 1),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 6989088383997837527,
            (
                Position(7, 2),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 11136314594631352526,
            (
                Position(7, 3),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 13368847107717333849,
            (
                Position(7, 4),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 4153736264362754717,
            (
                Position(7, 5),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 3466392036435820574,
            (
                Position(7, 6),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 17070582143425446773,
            (
                Position(7, 7),
                Piece {
                    color: Color::White,
                    kind: Kind::King,
                },
            ) => 7719205491145966421,
            _ => panic!("invalid position / piece combination {} / {}", self, piece.color),
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
    use super::ALL_POSITIONS;
    use crate::color::Color;
    use crate::kind::Kind;
    use crate::piece::Piece;

    #[test]
    fn it_doesnt_have_duplicate_position_hashes() {
        let mut used_hashes = Vec::new();
        let pieces = vec![
            Piece {
                color: Color::White,
                kind: Kind::Pawn,
            },
            Piece {
                color: Color::White,
                kind: Kind::Knight,
            },
            Piece {
                color: Color::White,
                kind: Kind::Bishop,
            },
            Piece {
                color: Color::White,
                kind: Kind::Rook,
            },
            Piece {
                color: Color::White,
                kind: Kind::Queen,
            },
            Piece {
                color: Color::White,
                kind: Kind::King,
            },
            Piece {
                color: Color::Black,
                kind: Kind::Pawn,
            },
            Piece {
                color: Color::Black,
                kind: Kind::Knight,
            },
            Piece {
                color: Color::Black,
                kind: Kind::Bishop,
            },
            Piece {
                color: Color::Black,
                kind: Kind::Rook,
            },
            Piece {
                color: Color::Black,
                kind: Kind::Queen,
            },
            Piece {
                color: Color::Black,
                kind: Kind::King,
            },
        ];
        let mut has_dupe = false;
        for pos in ALL_POSITIONS.iter() {
            for piece in &pieces {
                let hash = pos.hash(piece);
                if used_hashes.contains(&hash) {
                    has_dupe = true;
                    break;
                }
                used_hashes.push(hash);
            }
        }
        assert_eq!(has_dupe, false);
    }
}
