use crate::board::{self, Board};
use crate::color::Color;
use crate::direction::Direction;
use crate::file::File;
use crate::kind::Kind;
use crate::piece::Piece;
use crate::ply::Ply;
use crate::position::{
    east_stepper, north_east_stepper, north_stepper, north_west_stepper, south_east_stepper,
    south_stepper, south_west_stepper, west_stepper, Position, ALL_POSITIONS,
};
use crate::rank::Rank;
use crate::traits::PushSome;
use regex::Regex;
use std::ops::BitXor;

lazy_static! {
    static ref PGN_NUMBER: Regex = Regex::new("^[0-9]+.$").unwrap();
}

#[derive(Clone)]
pub struct Game {
    pub board: Board,
    pub turn: Color,
    pub forward: Direction,
    pub victor: Option<Color>,
    pub is_over: bool,
    pub history: Vec<Ply>,
    pub hash: u64,
}

impl Game {
    pub fn new() -> Self {
        let mut g = Game {
            history: Vec::with_capacity(1000),
            turn: Color::White,
            forward: Direction::Inc,
            victor: None,
            is_over: false,
            board: board::new(),
            hash: 0,
        };

        g.hash = g.hash();

        g
    }

    pub fn hash(&mut self) -> u64 {
        let mut hash = 12528355439915565194;

        for pos in ALL_POSITIONS.iter() {
            match self.get_piece(&pos) {
                Some(p) => hash = hash.bitxor(pos.hash(&p)),
                None => (),
            };
        }

        match self.turn {
            Color::White => hash = hash.bitxor(1),
            _ => (),
        };

        hash
    }

    pub fn try_ply(&mut self, s: &str) -> Result<(), String> {
        if self.is_over {
            return Err(String::from("Game is already over"));
        }
        let valid_plies = self.get_valid_plies();
        for ply in &valid_plies {
            if ply.uci() == s {
                self.do_ply(*ply, true);
                return Ok(());
            }
        }
        let san = s.replace("+", "");
        let san = san.replace("#", "");
        for ply in &valid_plies {
            if ply.san(&valid_plies) == san {
                self.do_ply(*ply, true);
                return Ok(());
            }
        }
        let mut plies_str = String::new();
        for ply in &valid_plies {
            plies_str.push_str(&ply.uci());
            plies_str.push_str(", ");
        }
        Err(format!(
            "{} is not a valid ply; valid plies are {}",
            s, plies_str
        ))
    }

    pub fn try_plies(&mut self, plies: &str) -> Result<(), String> {
        for ply in plies.split(" ") {
            if PGN_NUMBER.is_match(ply) {
                continue;
            }
            if self.is_over {
                return Err(String::from("Game is already over"));
            }
            self.try_ply(ply)?;
        }
        Ok(())
    }

    pub fn do_ply(&mut self, ply: Ply, set_victor: bool) {
        let destination_rank_usize: usize = ply.destination.rank.into();
        let destination_file_usize: usize = ply.destination.file.into();
        let origin_rank_usize: usize = ply.origin.rank.into();
        let origin_file_usize: usize = ply.origin.file.into();
        if let Some(t) = ply.target {
            match self.board[ply.destination.1][ply.destination.0] {
                None => {
                    let position = Position {
                        rank: ply.origin.rank,
                        file: ply.destination.file,
                    };
                    self.bitxor(position.hash(&t));
                    self.board[origin_rank_usize][ply.destination.0] = None;
                }
                Some(p) => self.bitxor(ply.destination.hash(&p)),
            };
        }
        let new_piece = match ply.promotion {
            Some(p) => p,
            None => ply.piece,
        };
        self.board[ply.destination.1][ply.destination.0] = Some(new_piece);
        self.bitxor(ply.destination.hash(&new_piece));

        if ply.piece.kind == Kind::King && ply.origin.file == File::E {
            let rook = Piece {
                color: self.turn,
                kind: Kind::Rook,
            };
            if ply.destination.file == File::C {
                self.board[origin_rank_usize][0] = None;
                self.board[origin_rank_usize][3] = Some(rook);
                let position = Position {
                    file: File::A,
                    rank: ply.origin.rank,
                };
                self.bitxor(position.hash(&rook));
                let position = Position {
                    file: File::D,
                    rank: ply.origin.rank,
                };
                self.bitxor(position.hash(&rook));
            } else if ply.destination.file == File::G {
                self.board[origin_rank_usize][7] = None;
                self.board[origin_rank_usize][5] = Some(rook);
                let position = Position {
                    file: File::H,
                    rank: ply.origin.rank,
                };
                self.bitxor(position.hash(&rook));
                let position = Position {
                    file: File::F,
                    rank: ply.origin.rank,
                };
                self.bitxor(position.hash(&rook));
            }
        }
        self.board[origin_rank_usize][origin_file_usize] = None;
        self.bitxor(ply.origin.hash(&ply.piece));

        self.history.push(ply);
        self.switch_turns();
        self.bitxor(1);

        if set_victor {
            let has_valid_plies = self.has_valid_plies();
            if !has_valid_plies {
                self.is_over = true;
                if self.is_in_check() {
                    self.victor = Some(self.turn);
                }
            } else if self.is_draw(has_valid_plies) {
                self.is_over = true;
            }
        }
    }

    pub fn revert_last_ply(&mut self) {
        let ply = self.history.pop().unwrap();
        let destination_rank_usize: usize = ply.destination.rank.into();
        let destination_file_usize: usize = ply.destination.file.into();
        let origin_rank_usize: usize = ply.origin.rank.into();
        let origin_file_usize: usize = ply.origin.file.into();
        if let Some(t) = ply.target {
            if ply.piece.kind == Kind::Pawn {
                let is_dest_rank_en_passant = match self.turn {
                    Color::White => 2 == destination_rank_usize,
                    _ => 5 == destination_rank_usize,
                };
                if is_dest_rank_en_passant {
                    match self.history.last() {
                        Some(p) => {
                            if p.piece.kind == Kind::Pawn
                                && p.destination.file == ply.destination.file
                                && ((p.origin.rank == Rank::Two
                                    && p.destination.rank == Rank::Four)
                                    || (p.origin.rank == Rank::Seven
                                        && p.destination.rank == Rank::Five))
                            {
                                let (en_passant_target_rank_usize, en_passant_target_rank) =
                                    match self.turn {
                                        Color::White => (3, Rank::Four),
                                        Color::Black => (4, Rank::Five),
                                    };
                                self.board[en_passant_target_rank_usize][destination_file_usize] =
                                    Some(t);
                                let position = Position {
                                    file: ply.destination.file,
                                    rank: en_passant_target_rank,
                                };
                                self.bitxor(position.hash(&t));
                                self.board[destination_rank_usize][destination_file_usize] = None;
                            } else {
                                self.board[destination_rank_usize][destination_file_usize] =
                                    Some(t);
                                self.bitxor(ply.destination.hash(&t));
                            }
                        }
                        None => {
                            self.board[destination_rank_usize][destination_file_usize] = Some(t);
                            self.bitxor(ply.destination.hash(&t));
                        }
                    };
                } else {
                    self.board[destination_rank_usize][destination_file_usize] = Some(t);
                    self.bitxor(ply.destination.hash(&t));
                }
            } else {
                self.board[destination_rank_usize][destination_file_usize] = Some(t);
                self.bitxor(ply.destination.hash(&t));
            }
        } else {
            self.board[destination_rank_usize][destination_file_usize] = None;
        }
        let new_piece = match ply.promotion {
            Some(p) => p,
            None => ply.piece,
        };
        self.bitxor(ply.destination.hash(&new_piece));

        self.switch_turns();
        self.bitxor(1);

        if ply.piece.kind == Kind::King && ply.origin.file == File::E {
            let rook = Piece {
                color: self.turn,
                kind: Kind::Rook,
            };
            if ply.destination.file == File::C {
                self.board[origin_rank_usize][0] = Some(rook);
                self.board[origin_rank_usize][3] = None;
                let position = Position {
                    file: File::A,
                    rank: ply.origin.rank,
                };
                self.bitxor(position.hash(&rook));
                let position = Position {
                    file: File::D,
                    rank: ply.origin.rank,
                };
                self.bitxor(position.hash(&rook));
            } else if ply.destination.file == File::G {
                self.board[origin_rank_usize][7] = Some(rook);
                self.board[origin_rank_usize][5] = None;
                let position = Position {
                    file: File::H,
                    rank: ply.origin.rank,
                };
                self.bitxor(position.hash(&rook));
                let position = Position {
                    file: File::F,
                    rank: ply.origin.rank,
                };
                self.bitxor(position.hash(&rook));
            }
        }
        self.board[origin_rank_usize][origin_file_usize] = Some(ply.piece);
        self.bitxor(ply.origin.hash(&ply.piece));
    }

    fn bitxor(&mut self, hash: u64) {
        self.hash = self.hash.bitxor(hash);
    }

    pub fn is_draw(&mut self, has_valid_plies: bool) -> bool {
        if !has_valid_plies {
            return !self.is_in_check();
        }

        let hist_len = self.history.len();

        // Fifty non-progressing moves
        if hist_len >= 100 {
            let mut has_capture_or_pawn_ply_occurred = false;
            let mut recent_plies = self.history.clone();
            recent_plies.reverse();
            recent_plies.truncate(100);

            for ply in recent_plies {
                if ply.piece.kind == Kind::Pawn {
                    has_capture_or_pawn_ply_occurred = true;
                    break;
                }
                if let Some(_) = ply.target {
                    has_capture_or_pawn_ply_occurred = true;
                    break;
                }
            }
            if !has_capture_or_pawn_ply_occurred {
                return true;
            }
        }

        // Three-fold repetition
        if hist_len >= 8 {
            let mut has_capture_or_pawn_ply_occurred = false;
            let mut recent_plies = self.history.clone();
            recent_plies.reverse();
            recent_plies.truncate(8);
            recent_plies.reverse();

            for ply in &recent_plies {
                if ply.piece.kind == Kind::Pawn {
                    has_capture_or_pawn_ply_occurred = true;
                    break;
                }
                if let Some(_) = ply.target {
                    has_capture_or_pawn_ply_occurred = true;
                    break;
                }
            }
            if !has_capture_or_pawn_ply_occurred {
                let mut are_positions_eq = false;
                let mut valid_plies_b = Vec::with_capacity(50);
                let mut valid_plies_c = Vec::with_capacity(50);
                let board_a = self.board;
                self.revert_last_ply();
                self.revert_last_ply();
                self.revert_last_ply();
                self.revert_last_ply();
                let board_b = self.board;
                if board::eq(&board_a, &board_b) {
                    self.revert_last_ply();
                    self.revert_last_ply();
                    self.revert_last_ply();
                    self.revert_last_ply();
                    if board::eq(&board_a, &self.board) {
                        are_positions_eq = true
                    }
                    if are_positions_eq {
                        valid_plies_c = self.get_valid_plies();
                    }
                    self.do_ply(recent_plies[0], false);
                    self.do_ply(recent_plies[1], false);
                    self.do_ply(recent_plies[2], false);
                    self.do_ply(recent_plies[3], false);
                }
                if are_positions_eq {
                    valid_plies_b = self.get_valid_plies();
                }
                self.do_ply(recent_plies[4], false);
                self.do_ply(recent_plies[5], false);
                self.do_ply(recent_plies[6], false);
                self.do_ply(recent_plies[7], false);
                if are_positions_eq {
                    let valid_plies_a = self.get_valid_plies();
                    if valid_plies_a == valid_plies_b && valid_plies_a == valid_plies_c {
                        return true;
                    }
                }
            }
        }

        // Insufficient material
        let mut white_piece_cnt = 0;
        let mut black_piece_cnt = 0;
        let mut white_has_non_knight_bishop = false;
        let mut black_has_non_knight_bishop = false;

        let mut white_on_white_bishop = false;
        let mut white_on_black_bishop = false;
        let mut black_on_white_bishop = false;
        let mut black_on_black_bishop = false;

        for pos in ALL_POSITIONS.iter() {
            match self.get_piece(&pos) {
                Some(p) => match p.color {
                    Color::White => match p.kind {
                        Kind::King => (),
                        _ => {
                            white_piece_cnt += 1;
                            if white_piece_cnt > 1 {
                                return false;
                            }
                            match p.kind {
                                Kind::Bishop => {
                                    if pos.is_black() {
                                        white_on_black_bishop = true;
                                    } else {
                                        white_on_white_bishop = true;
                                    }
                                }
                                Kind::Knight => (),
                                _ => white_has_non_knight_bishop = true,
                            };
                        }
                    },
                    _ => match p.kind {
                        Kind::King => (),
                        _ => {
                            black_piece_cnt += 1;
                            if black_piece_cnt > 1 {
                                return false;
                            }
                            match p.kind {
                                Kind::Bishop => {
                                    if pos.is_black() {
                                        black_on_black_bishop = true;
                                    } else {
                                        black_on_white_bishop = true;
                                    }
                                }
                                Kind::Knight => (),
                                _ => black_has_non_knight_bishop = true,
                            };
                        }
                    },
                },
                None => (),
            };
        }

        if white_piece_cnt == 0 && black_piece_cnt == 0 {
            return true;
        }

        if white_piece_cnt < 2 && !white_has_non_knight_bishop && black_piece_cnt == 0 {
            return true;
        }

        if black_piece_cnt < 2 && !black_has_non_knight_bishop && white_piece_cnt == 0 {
            return true;
        }

        if white_piece_cnt == 1
            && black_piece_cnt == 1
            && ((white_on_white_bishop && black_on_white_bishop)
                || (white_on_black_bishop && black_on_black_bishop))
        {
            return true;
        }

        false
    }

    pub fn switch_turns(&mut self) {
        match self.turn {
            Color::White => {
                self.turn = Color::Black;
                self.forward = Direction::Dec;
            }
            _ => {
                self.turn = Color::White;
                self.forward = Direction::Inc;
            }
        };
    }

    pub fn is_in_check(&mut self) -> bool {
        let king_position = self.get_king_position();
        self.can_enemy_attack(&king_position)
    }

    pub fn can_enemy_attack(&self, destination: &Position) -> bool {
        let enemy_color = match self.turn {
            Color::White => &Color::Black,
            _ => &Color::White,
        };
        if self.does_stepper_encounter_piece(
            destination,
            north_east_stepper,
            &Kind::Queen,
            enemy_color,
        ) || self.does_stepper_encounter_piece(
            destination,
            north_west_stepper,
            &Kind::Queen,
            enemy_color,
        ) || self.does_stepper_encounter_piece(
            destination,
            south_east_stepper,
            &Kind::Queen,
            enemy_color,
        ) || self.does_stepper_encounter_piece(
            destination,
            south_west_stepper,
            &Kind::Queen,
            enemy_color,
        ) || self.does_stepper_encounter_piece(
            destination,
            north_stepper,
            &Kind::Queen,
            enemy_color,
        ) || self.does_stepper_encounter_piece(
            destination,
            west_stepper,
            &Kind::Queen,
            enemy_color,
        ) || self.does_stepper_encounter_piece(
            destination,
            south_stepper,
            &Kind::Queen,
            enemy_color,
        ) || self.does_stepper_encounter_piece(
            destination,
            east_stepper,
            &Kind::Queen,
            enemy_color,
        ) {
            return true;
        }

        if self.does_stepper_encounter_piece(
            destination,
            north_east_stepper,
            &Kind::Bishop,
            enemy_color,
        ) || self.does_stepper_encounter_piece(
            destination,
            north_west_stepper,
            &Kind::Bishop,
            enemy_color,
        ) || self.does_stepper_encounter_piece(
            destination,
            south_east_stepper,
            &Kind::Bishop,
            enemy_color,
        ) || self.does_stepper_encounter_piece(
            destination,
            south_west_stepper,
            &Kind::Bishop,
            enemy_color,
        ) {
            return true;
        }

        if self.does_stepper_encounter_piece(destination, north_stepper, &Kind::Rook, enemy_color)
            || self.does_stepper_encounter_piece(
                destination,
                west_stepper,
                &Kind::Rook,
                enemy_color,
            )
            || self.does_stepper_encounter_piece(
                destination,
                south_stepper,
                &Kind::Rook,
                enemy_color,
            )
            || self.does_stepper_encounter_piece(
                destination,
                east_stepper,
                &Kind::Rook,
                enemy_color,
            )
        {
            return true;
        }

        let valid_knight_destinations = self.get_valid_knight_destinations(destination);
        for dest in valid_knight_destinations {
            match self.get_piece(&dest) {
                Some(p) => {
                    if p.kind == Kind::Knight && p.color != self.turn {
                        return true;
                    }
                }
                None => (),
            }
        }
        match destination.step(Direction::Inc, self.forward) {
            Some(p) => match self.get_piece(&p) {
                Some(p) => {
                    if p.kind == Kind::Pawn && p.color != self.turn {
                        return true;
                    }
                }
                None => (),
            },
            None => (),
        };

        match destination.step(Direction::Dec, self.forward) {
            Some(p) => match self.get_piece(&p) {
                Some(p) => {
                    if p.kind == Kind::Pawn && p.color != self.turn {
                        return true;
                    }
                }
                None => (),
            },
            None => (),
        };
        let valid_king_destinations = self.get_valid_king_destinations(destination);
        for dest in valid_king_destinations {
            match self.get_piece(&dest) {
                Some(p) => {
                    if p.kind == Kind::King && p.color != self.turn {
                        return true;
                    }
                }
                None => (),
            }
        }
        false
    }

    pub fn count_attackers(&self, destination: &Position, color: &Color) -> i16 {
        let mut cnt = 0;
        // if self.does_stepper_encounter_piece(destination, north_east_stepper, &Kind::Queen, color) {
        //     cnt += 1;
        // }
        // if self.does_stepper_encounter_piece(destination, north_west_stepper, &Kind::Queen, color) {
        //     cnt += 1;
        // }
        // if self.does_stepper_encounter_piece(destination, south_east_stepper, &Kind::Queen, color) {
        //     cnt += 1;
        // }
        // if self.does_stepper_encounter_piece(destination, south_west_stepper, &Kind::Queen, color) {
        //     cnt += 1;
        // }
        // if self.does_stepper_encounter_piece(destination, north_stepper, &Kind::Queen, color) {
        //     cnt += 1;
        // }
        // if self.does_stepper_encounter_piece(destination, west_stepper, &Kind::Queen, color) {
        //     cnt += 1;
        // }
        // if self.does_stepper_encounter_piece(destination, south_stepper, &Kind::Queen, color) {
        //     cnt += 1;
        // }
        // if self.does_stepper_encounter_piece(destination, east_stepper, &Kind::Queen, color) {
        //     cnt += 1;
        // }

        if self.does_stepper_encounter_piece(destination, north_east_stepper, &Kind::Bishop, color)
        {
            cnt += 1;
        }
        if self.does_stepper_encounter_piece(destination, north_west_stepper, &Kind::Bishop, color)
        {
            cnt += 1;
        }
        if self.does_stepper_encounter_piece(destination, south_east_stepper, &Kind::Bishop, color)
        {
            cnt += 1;
        }
        if self.does_stepper_encounter_piece(destination, south_west_stepper, &Kind::Bishop, color)
        {
            cnt += 1;
        }

        if self.does_stepper_encounter_piece(destination, north_stepper, &Kind::Rook, color) {
            cnt += 1;
        }
        if self.does_stepper_encounter_piece(destination, west_stepper, &Kind::Rook, color) {
            cnt += 1;
        }
        if self.does_stepper_encounter_piece(destination, south_stepper, &Kind::Rook, color) {
            cnt += 1;
        }
        if self.does_stepper_encounter_piece(destination, east_stepper, &Kind::Rook, color) {
            cnt += 1;
        }

        let valid_knight_destinations = self.get_valid_knight_destinations(destination);
        for dest in valid_knight_destinations {
            match self.get_piece(&dest) {
                Some(p) => {
                    if p.kind == Kind::Knight && &p.color == color {
                        cnt += 1;
                    }
                }
                None => (),
            }
        }
        let forward = if color == &self.turn {
            self.forward
        } else {
            match self.forward {
                Direction::Inc => Direction::Dec,
                _ => Direction::Inc,
            }
        };
        match destination.step(Direction::Inc, forward) {
            Some(p) => match self.get_piece(&p) {
                Some(p) => {
                    if p.kind == Kind::Pawn && &p.color == color {
                        cnt += 1;
                    }
                }
                None => (),
            },
            None => (),
        };

        match destination.step(Direction::Dec, forward) {
            Some(p) => match self.get_piece(&p) {
                Some(p) => {
                    if p.kind == Kind::Pawn && &p.color == color {
                        cnt += 1;
                    }
                }
                None => (),
            },
            None => (),
        };
        let valid_king_destinations = self.get_valid_king_destinations(destination);
        for dest in valid_king_destinations {
            match self.get_piece(&dest) {
                Some(p) => {
                    if p.kind == Kind::King && &p.color == color {
                        cnt += 1;
                    }
                }
                None => (),
            }
        }

        cnt
    }

    fn does_stepper_encounter_piece<S>(
        &self,
        origin: &Position,
        stepper: S,
        kind: &Kind,
        color: &Color,
    ) -> bool
    where
        S: Fn(&Position) -> Option<Position>,
    {
        let mut next = stepper(origin);
        loop {
            match next {
                Some(p) => match self.get_piece(&p) {
                    Some(p) => return kind == &p.kind && color == &p.color,
                    None => (),
                },
                None => return false,
            };
            next = stepper(&next.unwrap());
        }
    }

    fn get_valid_knight_destinations(&self, origin: &Position) -> Vec<Position> {
        let mut valid_destinations = Vec::with_capacity(8);

        valid_destinations.push_some(origin.step_nne());
        valid_destinations.push_some(origin.step_nnw());
        valid_destinations.push_some(origin.step_sse());
        valid_destinations.push_some(origin.step_ssw());
        valid_destinations.push_some(origin.step_ene());
        valid_destinations.push_some(origin.step_ese());
        valid_destinations.push_some(origin.step_wnw());
        valid_destinations.push_some(origin.step_wsw());

        valid_destinations
    }

    fn get_valid_king_destinations(&self, origin: &Position) -> Vec<Position> {
        let mut valid_destinations = Vec::with_capacity(8);

        valid_destinations.push_some(origin.step_n());
        valid_destinations.push_some(origin.step_s());
        valid_destinations.push_some(origin.step_e());
        valid_destinations.push_some(origin.step_w());
        valid_destinations.push_some(origin.step_ne());
        valid_destinations.push_some(origin.step_se());
        valid_destinations.push_some(origin.step_nw());
        valid_destinations.push_some(origin.step_sw());

        valid_destinations
    }

    pub fn get_king_position(&self) -> Position {
        for position in ALL_POSITIONS.iter() {
            match self.get_piece(&position) {
                Some(p) => {
                    if p.color == self.turn && p.kind == Kind::King {
                        return *position;
                    }
                }
                None => (),
            };
        }
        self.show_history();
        panic!("coudn't find king!")
    }

    pub fn show_history(&self) {
        let mut hist = String::new();
        for pl in &self.history {
            hist.push_str(&pl.san(&Vec::new()));
            hist.push_str(", ");
        }
        println!("{} plies: {}", self.history.len(), hist);
    }

    pub fn get_valid_plies(&mut self) -> Vec<Ply> {
        let mut valid_plies = Vec::with_capacity(50);

        for position in ALL_POSITIONS.iter() {
            match self.get_piece(&position) {
                Some(v) => {
                    if self.turn == v.color {
                        match v.kind {
                            Kind::Pawn => {
                                valid_plies.append(&mut self.get_valid_pawn_plies(v, position))
                            }
                            Kind::Rook => {
                                valid_plies.append(&mut self.get_valid_rook_plies(v, position))
                            }
                            Kind::Knight => {
                                valid_plies.append(&mut self.get_valid_knight_plies(v, position))
                            }
                            Kind::Bishop => {
                                valid_plies.append(&mut self.get_valid_bishop_plies(v, position))
                            }
                            Kind::King => {
                                valid_plies.append(&mut self.get_valid_king_plies(v, position))
                            }
                            Kind::Queen => {
                                valid_plies.append(&mut self.get_valid_queen_plies(v, position))
                            }
                        };
                    }
                }
                None => (),
            };
        }

        valid_plies
    }

    fn does_ply_put_in_check(&mut self, ply: Ply) -> bool {
        self.do_ply(ply, false);
        self.switch_turns();
        if self.is_in_check() {
            self.switch_turns();
            self.revert_last_ply();
            return true;
        }
        self.switch_turns();
        self.revert_last_ply();
        false
    }

    pub fn has_valid_plies(&mut self) -> bool {
        for position in ALL_POSITIONS.iter() {
            match self.get_piece(&position) {
                Some(v) => {
                    if self.turn == v.color {
                        match v.kind {
                            Kind::Pawn => {
                                if self.has_valid_pawn_plies(v, position) {
                                    return true;
                                }
                            }
                            Kind::Rook => {
                                if self.has_valid_rook_plies(v, position) {
                                    return true;
                                }
                            }
                            Kind::Knight => {
                                if self.has_valid_knight_plies(v, position) {
                                    return true;
                                }
                            }
                            Kind::Bishop => {
                                if self.has_valid_bishop_plies(v, position) {
                                    return true;
                                }
                            }
                            Kind::King => {
                                if self.has_valid_king_plies(v, position) {
                                    return true;
                                }
                            }
                            Kind::Queen => {
                                if self.has_valid_bishop_plies(v, position)
                                    || self.has_valid_rook_plies(v, position)
                                {
                                    return true;
                                }
                            }
                        };
                    }
                }
                None => (),
            };
        }

        false
    }

    pub fn valid_non_king_plies_diff(&mut self) -> i8 {
        let mut diff = 0;
        for pos in ALL_POSITIONS.iter() {
            match self.get_piece(&pos) {
                Some(v) => {
                    let switched_turns = self.turn != v.color;
                    if switched_turns {
                        self.switch_turns();
                    }
                    let cnt = match v.kind {
                        Kind::Pawn => self.count_valid_pawn_plies(pos),
                        Kind::Rook => self.count_valid_rook_plies(pos),
                        Kind::Knight => self.count_valid_knight_plies(pos),
                        Kind::Bishop => self.count_valid_bishop_plies(pos),
                        // Kind::Queen => {
                        //     self.count_valid_rook_plies(pos)
                        //         + self.count_valid_bishop_plies(pos)
                        // }
                        _ => 0,
                    };
                    if switched_turns {
                        self.switch_turns();
                    }
                    if self.turn == v.color {
                        diff += cnt as i8;
                    } else {
                        diff -= cnt as i8;
                    }
                }
                None => (),
            };
        }

        diff
    }

    pub fn get_piece(&self, position: &Position) -> Option<Piece> {
        self.board[usize::from(position.rank)][usize::from(position.file)]
    }

    fn get_valid_stepping_plies<S>(
        &mut self,
        piece: Piece,
        origin: &Position,
        stepper: S,
    ) -> Vec<Ply>
    where
        S: Fn(&Position) -> Option<Position>,
    {
        let mut valid_plies = Vec::with_capacity(7);

        let mut destination = stepper(origin);

        loop {
            match destination {
                Some(d) => match self.get_piece(&d) {
                    Some(p) => {
                        if p.color != self.turn {
                            let pl = Ply {
                                piece: piece,
                                origin: *origin,
                                destination: d,
                                target: Some(p),
                                promotion: None,
                            };
                            if !self.does_ply_put_in_check(pl) {
                                valid_plies.push(pl);
                            }
                        }
                        break;
                    }
                    None => {
                        let pl = Ply {
                            piece: piece,
                            origin: *origin,
                            destination: d,
                            target: None,
                            promotion: None,
                        };
                        if !self.does_ply_put_in_check(pl) {
                            valid_plies.push(pl);
                        }
                    }
                },
                None => break,
            };
            destination = stepper(&destination.unwrap());
        }

        valid_plies
    }

    fn has_valid_stepping_plies<S>(&mut self, piece: Piece, origin: &Position, stepper: S) -> bool
    where
        S: Fn(&Position) -> Option<Position>,
    {
        let mut destination = stepper(origin);

        loop {
            match destination {
                Some(d) => match self.get_piece(&d) {
                    Some(p) => {
                        if p.color != self.turn {
                            let pl = Ply {
                                piece: piece,
                                origin: *origin,
                                destination: d,
                                target: Some(p),
                                promotion: None,
                            };
                            if !self.does_ply_put_in_check(pl) {
                                return true;
                            }
                        }
                        break;
                    }
                    None => {
                        let pl = Ply {
                            piece: piece,
                            origin: *origin,
                            destination: d,
                            target: None,
                            promotion: None,
                        };
                        if !self.does_ply_put_in_check(pl) {
                            return true;
                        }
                    }
                },
                None => break,
            };
            destination = stepper(&destination.unwrap());
        }

        false
    }

    fn count_valid_stepping_plies<S>(&mut self, origin: &Position, stepper: S) -> u8
    where
        S: Fn(&Position) -> Option<Position>,
    {
        let mut cnt = 0;
        let mut destination = stepper(origin);

        loop {
            match destination {
                Some(d) => match self.get_piece(&d) {
                    Some(p) => {
                        if p.color != self.turn {
                            cnt += 1;
                        }
                        break;
                    }
                    None => cnt += 1,
                },
                None => break,
            };
            destination = stepper(&destination.unwrap());
        }

        cnt
    }

    fn get_valid_queen_plies(&mut self, queen: Piece, origin: &Position) -> Vec<Ply> {
        let mut valid_plies = Vec::with_capacity(28);

        valid_plies.append(&mut self.get_valid_stepping_plies(queen, origin, north_stepper));
        valid_plies.append(&mut self.get_valid_stepping_plies(queen, origin, north_east_stepper));
        valid_plies.append(&mut self.get_valid_stepping_plies(queen, origin, north_west_stepper));
        valid_plies.append(&mut self.get_valid_stepping_plies(queen, origin, east_stepper));
        valid_plies.append(&mut self.get_valid_stepping_plies(queen, origin, west_stepper));
        valid_plies.append(&mut self.get_valid_stepping_plies(queen, origin, south_east_stepper));
        valid_plies.append(&mut self.get_valid_stepping_plies(queen, origin, south_west_stepper));
        valid_plies.append(&mut self.get_valid_stepping_plies(queen, origin, south_stepper));

        valid_plies
    }

    fn get_valid_rook_plies(&mut self, rook: Piece, origin: &Position) -> Vec<Ply> {
        let mut valid_plies = Vec::with_capacity(14);

        valid_plies.append(&mut self.get_valid_stepping_plies(rook, origin, north_stepper));
        valid_plies.append(&mut self.get_valid_stepping_plies(rook, origin, south_stepper));
        valid_plies.append(&mut self.get_valid_stepping_plies(rook, origin, east_stepper));
        valid_plies.append(&mut self.get_valid_stepping_plies(rook, origin, west_stepper));

        valid_plies
    }

    fn has_valid_rook_plies(&mut self, rook: Piece, origin: &Position) -> bool {
        self.has_valid_stepping_plies(rook, origin, north_stepper)
            || self.has_valid_stepping_plies(rook, origin, south_stepper)
            || self.has_valid_stepping_plies(rook, origin, east_stepper)
            || self.has_valid_stepping_plies(rook, origin, west_stepper)
    }

    fn count_valid_rook_plies(&mut self, origin: &Position) -> u8 {
        self.count_valid_stepping_plies(origin, north_stepper)
            + self.count_valid_stepping_plies(origin, south_stepper)
            + self.count_valid_stepping_plies(origin, east_stepper)
            + self.count_valid_stepping_plies(origin, west_stepper)
    }

    fn get_valid_bishop_plies(&mut self, bishop: Piece, origin: &Position) -> Vec<Ply> {
        let mut valid_plies = Vec::with_capacity(14);

        valid_plies.append(&mut self.get_valid_stepping_plies(bishop, origin, north_east_stepper));
        valid_plies.append(&mut self.get_valid_stepping_plies(bishop, origin, south_east_stepper));
        valid_plies.append(&mut self.get_valid_stepping_plies(bishop, origin, south_west_stepper));
        valid_plies.append(&mut self.get_valid_stepping_plies(bishop, origin, north_west_stepper));

        valid_plies
    }

    fn has_valid_bishop_plies(&mut self, bishop: Piece, origin: &Position) -> bool {
        self.has_valid_stepping_plies(bishop, origin, north_east_stepper)
            || self.has_valid_stepping_plies(bishop, origin, south_east_stepper)
            || self.has_valid_stepping_plies(bishop, origin, north_west_stepper)
            || self.has_valid_stepping_plies(bishop, origin, south_west_stepper)
    }

    fn count_valid_bishop_plies(&mut self, origin: &Position) -> u8 {
        self.count_valid_stepping_plies(origin, north_east_stepper)
            + self.count_valid_stepping_plies(origin, south_east_stepper)
            + self.count_valid_stepping_plies(origin, north_west_stepper)
            + self.count_valid_stepping_plies(origin, south_west_stepper)
    }

    fn has_valid_king_plies(&mut self, king: Piece, origin: &Position) -> bool {
        let valid_destinations = self.get_valid_king_destinations(origin);

        for dest in &valid_destinations {
            match self.get_valid_attacking_ply(king, origin, dest) {
                Some(_) => return true,
                None => (),
            };
        }

        false
    }

    fn get_valid_king_plies(&mut self, king: Piece, origin: &Position) -> Vec<Ply> {
        let mut valid_plies = Vec::with_capacity(10);
        let valid_destinations = self.get_valid_king_destinations(origin);

        for dest in &valid_destinations {
            valid_plies.push_some(self.get_valid_attacking_ply(king, origin, dest));
        }

        if !self.has_king_moved() {
            let starting_rank = match self.turn {
                Color::White => Rank::One,
                _ => Rank::Eight,
            };
            if !self.has_rook_h_moved() {
                let f_start = Position {
                    file: File::F,
                    rank: starting_rank,
                };
                match self.get_piece(&f_start) {
                    Some(_) => (),
                    None => {
                        let g_start = Position {
                            file: File::G,
                            rank: starting_rank,
                        };
                        match self.get_piece(&g_start) {
                            Some(_) => (),
                            None => {
                                if !self.is_in_check() {
                                    if !self.can_enemy_attack(&g_start)
                                        && !self.can_enemy_attack(&f_start)
                                    {
                                        valid_plies.push(Ply {
                                            piece: king,
                                            origin: *origin,
                                            destination: g_start,
                                            target: None,
                                            promotion: None,
                                        })
                                    }
                                }
                            }
                        };
                    }
                };
            }

            if !self.has_rook_a_moved() {
                let d_start = Position {
                    file: File::D,
                    rank: starting_rank,
                };
                match self.get_piece(&d_start) {
                    Some(_) => (),
                    None => {
                        let c_start = Position {
                            file: File::C,
                            rank: starting_rank,
                        };
                        match self.get_piece(&c_start) {
                            Some(_) => (),
                            None => {
                                let b_start = Position {
                                    file: File::B,
                                    rank: starting_rank,
                                };
                                match self.get_piece(&b_start) {
                                    Some(_) => (),
                                    None => {
                                        if !self.is_in_check() {
                                            if !self.can_enemy_attack(&c_start)
                                                && !self.can_enemy_attack(&d_start)
                                            {
                                                valid_plies.push(Ply {
                                                    piece: king,
                                                    origin: *origin,
                                                    destination: c_start,
                                                    target: None,
                                                    promotion: None,
                                                })
                                            }
                                        }
                                    }
                                };
                            }
                        };
                    }
                };
            }
        }

        valid_plies
    }

    pub fn has_king_moved(&self) -> bool {
        let starting_position = match self.turn {
            Color::White => Position {
                file: File::E,
                rank: Rank::One,
            },
            _ => Position {
                file: File::E,
                rank: Rank::Eight,
            },
        };
        match self.get_piece(&starting_position) {
            Some(p) => {
                if p.kind != Kind::King || p.color != self.turn {
                    return true;
                }
            }
            None => return true,
        };

        for pl in &self.history {
            if pl.piece.kind == Kind::King && pl.piece.color == self.turn {
                return true;
            }
        }
        false
    }

    pub fn has_rook_a_moved(&self) -> bool {
        let starting_position = match self.turn {
            Color::White => Position {
                file: File::A,
                rank: Rank::One,
            },
            _ => Position {
                file: File::A,
                rank: Rank::Eight,
            },
        };
        match self.get_piece(&starting_position) {
            Some(p) => {
                if p.kind != Kind::Rook || p.color != self.turn {
                    return true;
                }
            }
            None => return true,
        };
        for ply in &self.history {
            if ply.origin == starting_position {
                return true;
            }
            if ply.destination == starting_position {
                return true;
            }
        }
        false
    }

    pub fn has_rook_h_moved(&self) -> bool {
        let starting_position = match self.turn {
            Color::White => Position {
                file: File::H,
                rank: Rank::One,
            },
            _ => Position {
                file: File::H,
                rank: Rank::Eight,
            },
        };
        match self.get_piece(&starting_position) {
            Some(p) => {
                if p.kind != Kind::Rook || p.color != self.turn {
                    return true;
                }
            }
            None => return true,
        };
        for ply in &self.history {
            if ply.origin == starting_position {
                return true;
            }
            if ply.destination == starting_position {
                return true;
            }
        }
        false
    }

    pub fn get_player_naive_material(&self) -> i16 {
        let mut mat = 0;
        for pos in ALL_POSITIONS.iter() {
            match self.get_piece(&pos) {
                Some(p) => {
                    if self.turn == p.color {
                        mat += p.naive_value()
                    }
                }
                None => (),
            };
        }
        mat
    }

    pub fn get_enemy_naive_material(&self) -> i16 {
        let mut mat = 0;
        for pos in ALL_POSITIONS.iter() {
            match self.get_piece(&pos) {
                Some(p) => {
                    if self.turn != p.color {
                        mat += p.naive_value()
                    }
                }
                None => (),
            };
        }
        mat
    }

    fn get_valid_knight_plies(&mut self, knight: Piece, origin: &Position) -> Vec<Ply> {
        let mut valid_plies = Vec::with_capacity(8);

        let valid_destinations = self.get_valid_knight_destinations(origin);
        for dest in &valid_destinations {
            valid_plies.push_some(self.get_valid_attacking_ply(knight, origin, dest));
        }

        valid_plies
    }

    fn has_valid_knight_plies(&mut self, knight: Piece, origin: &Position) -> bool {
        let valid_destinations = self.get_valid_knight_destinations(origin);
        for dest in &valid_destinations {
            match self.get_valid_attacking_ply(knight, origin, dest) {
                Some(_) => return true,
                None => (),
            };
        }

        false
    }

    fn count_valid_knight_plies(&mut self, origin: &Position) -> u8 {
        let mut cnt = 0;
        let valid_destinations = self.get_valid_knight_destinations(origin);
        for dest in &valid_destinations {
            if self.has_valid_attacking_ply(dest) {
                cnt += 1;
            }
        }

        cnt
    }

    fn get_valid_attacking_ply(
        &mut self,
        piece: Piece,
        origin: &Position,
        destination: &Position,
    ) -> Option<Ply> {
        match self.get_piece(&destination) {
            Some(t) => {
                if t.color != self.turn {
                    let pl = Ply {
                        piece,
                        origin: *origin,
                        destination: *destination,
                        target: Some(t),
                        promotion: None,
                    };
                    if !self.does_ply_put_in_check(pl) {
                        Some(pl)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            None => {
                let pl = Ply {
                    piece,
                    origin: *origin,
                    destination: *destination,
                    target: None,
                    promotion: None,
                };
                if !self.does_ply_put_in_check(pl) {
                    Some(pl)
                } else {
                    None
                }
            }
        }
    }

    fn has_valid_attacking_ply(&self, destination: &Position) -> bool {
        match self.get_piece(&destination) {
            Some(t) => t.color != self.turn,
            None => true,
        }
    }

    fn get_valid_pawn_plies(&mut self, pawn: Piece, origin: &Position) -> Vec<Ply> {
        let mut valid_plies = Vec::with_capacity(16);

        match origin.step(Direction::None, self.forward) {
            None => (),
            Some(destination) => match self.get_piece(&destination) {
                None => {
                    let mut pl = Ply {
                        piece: pawn,
                        origin: *origin,
                        destination,
                        target: None,
                        promotion: None,
                    };
                    if !self.does_ply_put_in_check(pl) {
                        valid_plies.append(&mut self.add_promotion_to_pawn_plies(&mut pl));
                    }
                    if (origin.rank == Rank::Two && self.turn == Color::White)
                        || (origin.rank == Rank::Seven && self.turn == Color::Black)
                    {
                        match destination.step(Direction::None, self.forward) {
                            None => (),
                            Some(destination) => match self.get_piece(&destination) {
                                None => {
                                    let pl = Ply {
                                        piece: pawn,
                                        origin: *origin,
                                        destination,
                                        target: None,
                                        promotion: None,
                                    };
                                    if !self.does_ply_put_in_check(pl) {
                                        valid_plies.push(pl);
                                    }
                                }
                                _ => (),
                            },
                        };
                    }
                }
                _ => (),
            },
        };

        match origin.step(Direction::Dec, self.forward) {
            None => (),
            Some(destination) => match self.get_piece(&destination) {
                Some(p) => {
                    if p.color != self.turn {
                        let mut pl = Ply {
                            piece: pawn,
                            origin: *origin,
                            destination,
                            target: Some(p),
                            promotion: None,
                        };
                        if !self.does_ply_put_in_check(pl) {
                            valid_plies.append(&mut self.add_promotion_to_pawn_plies(&mut pl));
                        }
                    }
                }
                None => (),
            },
        };

        match origin.step(Direction::Inc, self.forward) {
            None => (),
            Some(destination) => match self.get_piece(&destination) {
                Some(p) => {
                    if p.color != self.turn {
                        let mut pl = Ply {
                            piece: pawn,
                            origin: *origin,
                            destination,
                            target: Some(p),
                            promotion: None,
                        };
                        if !self.does_ply_put_in_check(pl) {
                            valid_plies.append(&mut self.add_promotion_to_pawn_plies(&mut pl));
                        }
                    }
                }
                None => (),
            },
        };

        let last_ply = self.history.last();
        if last_ply != None {
            let last_ply = *last_ply.unwrap();
            if last_ply.piece.kind == Kind::Pawn {
                if self.turn == Color::White && origin.rank == Rank::Five {
                    if last_ply.origin.rank == Rank::Seven
                        && last_ply.destination.rank == Rank::Five
                    {
                        if let Some(p) = last_ply.destination.step_w() {
                            if &p == origin {
                                let pl = Ply {
                                    origin: *origin,
                                    destination: Position {
                                        file: last_ply.destination.file,
                                        rank: Rank::Six,
                                    },
                                    piece: pawn,
                                    target: Some(last_ply.piece),
                                    promotion: None,
                                };
                                if !self.does_ply_put_in_check(pl) {
                                    valid_plies.push(pl);
                                }
                            }
                        }
                        if let Some(p) = last_ply.destination.step_e() {
                            if &p == origin {
                                let pl = Ply {
                                    origin: *origin,
                                    destination: Position {
                                        file: last_ply.destination.file,
                                        rank: Rank::Six,
                                    },
                                    piece: pawn,
                                    target: Some(last_ply.piece),
                                    promotion: None,
                                };
                                if !self.does_ply_put_in_check(pl) {
                                    valid_plies.push(pl);
                                }
                            }
                        }
                    }
                } else if self.turn == Color::Black && origin.rank == Rank::Four {
                    if last_ply.origin.rank == Rank::Two && last_ply.destination.rank == Rank::Four
                    {
                        if let Some(p) = last_ply.destination.step_w() {
                            if &p == origin {
                                let pl = Ply {
                                    origin: *origin,
                                    destination: Position {
                                        file: last_ply.destination.file,
                                        rank: Rank::Three,
                                    },
                                    piece: pawn,
                                    target: Some(last_ply.piece),
                                    promotion: None,
                                };
                                if !self.does_ply_put_in_check(pl) {
                                    valid_plies.push(pl);
                                }
                            }
                        }
                        if let Some(p) = last_ply.destination.step_e() {
                            if &p == origin {
                                let pl = Ply {
                                    origin: *origin,
                                    destination: Position {
                                        file: last_ply.destination.file,
                                        rank: Rank::Three,
                                    },
                                    piece: pawn,
                                    target: Some(last_ply.piece),
                                    promotion: None,
                                };
                                if !self.does_ply_put_in_check(pl) {
                                    valid_plies.push(pl);
                                }
                            }
                        }
                    }
                }
            }
        }

        valid_plies
    }

    fn has_valid_pawn_plies(&mut self, pawn: Piece, origin: &Position) -> bool {
        match origin.step(Direction::None, self.forward) {
            None => (),
            Some(destination) => match self.get_piece(&destination) {
                None => {
                    let p = Ply {
                        piece: pawn,
                        origin: *origin,
                        destination,
                        target: None,
                        promotion: None,
                    };
                    if !self.does_ply_put_in_check(p) {
                        return true;
                    }
                    if (origin.rank == Rank::Two && self.turn == Color::White)
                        || (origin.rank == Rank::Seven && self.turn == Color::Black)
                    {
                        match destination.step(Direction::None, self.forward) {
                            None => (),
                            Some(destination) => match self.get_piece(&destination) {
                                None => {
                                    let p = Ply {
                                        piece: pawn,
                                        origin: *origin,
                                        destination,
                                        target: None,
                                        promotion: None,
                                    };
                                    if !self.does_ply_put_in_check(p) {
                                        return true;
                                    }
                                }
                                _ => (),
                            },
                        };
                    }
                }
                _ => (),
            },
        };

        match origin.step(Direction::Dec, self.forward) {
            None => (),
            Some(destination) => match self.get_piece(&destination) {
                Some(p) => {
                    if p.color != self.turn {
                        let pl = Ply {
                            piece: pawn,
                            origin: *origin,
                            destination,
                            target: Some(p),
                            promotion: None,
                        };
                        if !self.does_ply_put_in_check(pl) {
                            return true;
                        }
                    }
                }
                None => (),
            },
        };

        match origin.step(Direction::Inc, self.forward) {
            None => (),
            Some(destination) => match self.get_piece(&destination) {
                Some(p) => {
                    if p.color != self.turn {
                        let pl = Ply {
                            piece: pawn,
                            origin: *origin,
                            destination,
                            target: Some(p),
                            promotion: None,
                        };
                        if !self.does_ply_put_in_check(pl) {
                            return true;
                        }
                    }
                }
                None => (),
            },
        };

        let last_ply = self.history.last();
        if last_ply != None {
            let last_ply = *last_ply.unwrap();
            if last_ply.piece.kind == Kind::Pawn {
                if self.turn == Color::White && origin.rank == Rank::Five {
                    if last_ply.origin.rank == Rank::Seven
                        && last_ply.destination.rank == Rank::Five
                    {
                        if let Some(p) = last_ply.destination.step_w() {
                            if &p == origin {
                                let pl = Ply {
                                    origin: *origin,
                                    destination: Position {
                                        file: last_ply.destination.file,
                                        rank: Rank::Six,
                                    },
                                    piece: pawn,
                                    target: Some(last_ply.piece),
                                    promotion: None,
                                };
                                if !self.does_ply_put_in_check(pl) {
                                    return true;
                                }
                            }
                        }
                        if let Some(p) = last_ply.destination.step_e() {
                            if &p == origin {
                                let pl = Ply {
                                    origin: *origin,
                                    destination: Position {
                                        file: last_ply.destination.file,
                                        rank: Rank::Six,
                                    },
                                    piece: pawn,
                                    target: Some(last_ply.piece),
                                    promotion: None,
                                };
                                if !self.does_ply_put_in_check(pl) {
                                    return true;
                                }
                            }
                        }
                    }
                } else if self.turn == Color::Black && origin.rank == Rank::Four {
                    if last_ply.origin.rank == Rank::Two && last_ply.destination.rank == Rank::Four
                    {
                        if let Some(p) = last_ply.destination.step_w() {
                            if &p == origin {
                                let pl = Ply {
                                    origin: *origin,
                                    destination: Position {
                                        file: last_ply.destination.file,
                                        rank: Rank::Three,
                                    },
                                    piece: pawn,
                                    target: Some(last_ply.piece),
                                    promotion: None,
                                };
                                if !self.does_ply_put_in_check(pl) {
                                    return true;
                                }
                            }
                        }
                        if let Some(p) = last_ply.destination.step_e() {
                            if &p == origin {
                                let pl = Ply {
                                    origin: *origin,
                                    destination: Position {
                                        file: last_ply.destination.file,
                                        rank: Rank::Three,
                                    },
                                    piece: pawn,
                                    target: Some(last_ply.piece),
                                    promotion: None,
                                };
                                if !self.does_ply_put_in_check(pl) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }

        false
    }

    fn count_valid_pawn_plies(&mut self, origin: &Position) -> u8 {
        let mut cnt = 0;
        match origin.step(Direction::None, self.forward) {
            None => (),
            Some(destination) => match self.get_piece(&destination) {
                None => {
                    cnt += 1;
                    if (origin.rank == Rank::Two && self.turn == Color::White)
                        || (origin.rank == Rank::Seven && self.turn == Color::Black)
                    {
                        match destination.step(Direction::None, self.forward) {
                            None => (),
                            Some(destination) => match self.get_piece(&destination) {
                                None => cnt += 1,
                                _ => (),
                            },
                        };
                    }
                }
                _ => (),
            },
        };

        match origin.step(Direction::Dec, self.forward) {
            None => (),
            Some(destination) => match self.get_piece(&destination) {
                Some(p) => {
                    if p.color != self.turn {
                        cnt += 1;
                    }
                }
                None => (),
            },
        };

        match origin.step(Direction::Inc, self.forward) {
            None => (),
            Some(destination) => match self.get_piece(&destination) {
                Some(p) => {
                    if p.color != self.turn {
                        cnt += 1;
                    }
                }
                None => (),
            },
        };

        if let Some(last_ply) = self.history.last() {
            if last_ply.piece.kind == Kind::Pawn {
                if self.turn == Color::White && origin.rank == Rank::Five {
                    if last_ply.origin.rank == Rank::Seven
                        && last_ply.destination.rank == Rank::Five
                    {
                        if let Some(p) = last_ply.destination.step_w() {
                            if &p == origin {
                                cnt += 1;
                            } else if let Some(p) = last_ply.destination.step_e() {
                                if &p == origin {
                                    cnt += 1;
                                }
                            }
                        }
                    }
                } else if self.turn == Color::Black && origin.rank == Rank::Four {
                    if last_ply.origin.rank == Rank::Two && last_ply.destination.rank == Rank::Four
                    {
                        if let Some(p) = last_ply.destination.step_w() {
                            if &p == origin {
                                cnt += 1;
                            } else if let Some(p) = last_ply.destination.step_e() {
                                if &p == origin {
                                    cnt += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        cnt
    }

    fn add_promotion_to_pawn_plies(&self, ply: &mut Ply) -> Vec<Ply> {
        if ply.destination.rank == Rank::Eight || ply.destination.rank == Rank::One {
            let mut plies: Vec<Ply> = Vec::with_capacity(4);
            ply.promotion = Some(Piece {
                color: self.turn,
                kind: Kind::Queen,
            });
            let mut plyn = ply.clone();
            plyn.promotion = Some(Piece {
                color: self.turn,
                kind: Kind::Knight,
            });
            plies.push(plyn);
            let mut plyb = ply.clone();
            plyb.promotion = Some(Piece {
                color: self.turn,
                kind: Kind::Bishop,
            });
            plies.push(plyb);
            let mut plyr = ply.clone();
            plyr.promotion = Some(Piece {
                color: self.turn,
                kind: Kind::Rook,
            });
            plies.push(plyr);
            plies.push(ply.clone());
            plies
        } else {
            vec![*ply]
        }
    }

    pub fn show_board(&self) {
        const HORIZ_SEP: &'static str = "|---|---|---|---|---|---|---|---|";
        for rank in self.board.iter().rev() {
            println!("{}", HORIZ_SEP);
            for piece in rank.iter() {
                print!("| ");
                match piece {
                    Some(v) => print!("{}", v.to_letter()),
                    None => print!(" "),
                };
                print!(" ");
            }
            print!("|\n");
        }
        println!("{}", HORIZ_SEP);
    }
}

#[cfg(test)]
mod tests {
    use crate::game::Game;
    use crate::tests::{
        assert_draw, assert_not_draw, assert_ply_invalid, assert_ply_valid, assert_reverts,
        make_game,
    };
    use test::Bencher;

    #[test]
    fn it_gets_correct_number_of_valid_plies_after_moves() {
        fn counter(g: &mut Game, depth: u8, cnt: &mut usize) {
            if !g.is_over {
                if depth == 1 {
                    *cnt += g.get_valid_plies().len();
                } else {
                    for p in g.get_valid_plies() {
                        g.do_ply(p, true);
                        counter(g, depth - 1, cnt);
                        g.revert_last_ply();
                    }
                }
            } else {
                *cnt += 1;
            }
        }

        fn count_possible_positions(depth: u8) -> usize {
            let mut cnt = 0;
            counter(&mut Game::new(), depth, &mut cnt);
            cnt
        }

        assert_eq!(count_possible_positions(1), 20);
        assert_eq!(count_possible_positions(2), 400);
        assert_eq!(count_possible_positions(3), 8902);
        assert_eq!(count_possible_positions(4), 197281);
        // assert_eq!(count_possible_positions(5), 4865609);
    }

    #[test]
    fn it_reverts_non_capturing_promotion_ply() {
        assert_reverts(
            "1. e4 e5 2. a3 d5 3. exd5 Qe7 4. d6 Qe6 5. d7+ Ke7 6.",
            "d8=Q+",
        );
    }

    #[test]
    fn it_reverts_capturing_promotion_ply() {
        assert_reverts("d4 a6 d5 a5 d6 a4 dxc7 a3", "cxb8=Q");
    }

    #[test]
    fn it_allows_black_west_en_passant_ply_as_valid() {
        assert_ply_valid("a2a3 e7e5 a3a4 e5e4 d2d4", "e4d3");
    }

    #[test]
    fn it_reverts_black_west_en_passant_ply() {
        assert_reverts("a3 e5 a4 e4 d4", "exd3");
    }

    #[test]
    fn it_allows_black_east_en_passant_ply_as_valid() {
        assert_ply_valid("a2a3 e7e5 a3a4 e5e4 f2f4", "e4f3");
    }

    #[test]
    fn it_reverts_black_east_en_passant_ply() {
        assert_reverts("a2a3 e7e5 a3a4 e5e4 f2f4", "e4f3");
    }

    #[test]
    fn it_forbids_black_west_expired_en_passant() {
        assert_ply_invalid("a2a3 e7e5 a3a4 e5e4 d2d4 a7a6", "e4d3");
    }

    #[test]
    fn it_forbids_black_east_expired_en_passant() {
        assert_ply_invalid("a2a3 e7e5 a3a4 e5e4 f2f4 a7a6", "e4f3");
    }

    #[test]
    fn it_allows_white_west_en_passant_ply_as_valid() {
        assert_ply_valid("d2d4 a7a5 d4d5 a5a4 a2a3 c7c5", "d5c6");
    }

    #[test]
    fn it_reverts_white_west_en_passant_ply() {
        assert_reverts("d2d4 a7a5 d4d5 a5a4 a2a3 c7c5", "d5c6");
    }

    #[test]
    fn it_allows_white_east_en_passant_ply_as_valid() {
        assert_ply_valid("d2d4 a7a5 d4d5 a5a4 a2a3 e7e5", "d5e6");
    }

    #[test]
    fn it_reverts_white_east_en_passant_ply() {
        assert_reverts("d2d4 a7a5 d4d5 a5a4 a2a3 e7e5", "d5e6");
    }

    #[test]
    fn it_forbids_white_west_expired_en_passant() {
        assert_ply_invalid("d2d4 a7a5 d4d5 a5a4 a2a3 e7e5 b2b3", "d5e6");
    }

    #[test]
    fn it_forbids_white_east_expired_en_passant() {
        assert_ply_invalid("d2d4 a7a5 d4d5 a5a4 a2a3 e7e5 b2b3", "d5c6");
    }

    #[test]
    fn it_allows_white_kingside_castle_when_h1_under_attack() {
        assert_ply_valid("a2a3 b7b6 e2e3 c8b7 f1d3 g8f6 g1e2 b8a6 g2g3 d7d6", "e1g1");
    }

    #[test]
    fn it_allows_white_queenside_castle_when_a1_under_attack() {
        assert_ply_valid("d2d3 g7g6 c1f4 f8g7 b1a3 a7a6 d1d2 a6a5 b2b3 a5a4", "O-O-O");
    }

    #[test]
    fn it_allows_white_queenside_castle_when_b1_under_attack() {
        assert_ply_valid("d4 d5 Na3 Bf5 Bf4 a6 Qd2 a5 c3 a4", "O-O-O");
    }

    #[test]
    fn it_forbids_white_kingside_castle_when_in_check() {
        assert_ply_invalid("e4 d5 Bc4 d4 Nf3 Qd5 a3 Qxe4+", "O-O");
    }

    #[test]
    fn it_forbids_white_queenside_castle_when_in_check() {
        assert_ply_invalid("e4 d5 Qf3 d4 Na3 Qd5 d3 a6 Bf4 Qxe4+", "O-O-O");
    }

    #[test]
    fn it_forbids_white_kingside_castle_when_f1_under_attack() {
        assert_ply_invalid("e3 b6 Nf3 a5 Ba6 Bxa6", "O-O");
    }

    #[test]
    fn it_forbids_white_kingside_castle_when_g1_under_attack() {
        assert_ply_invalid("e4 e5 f3 Bc5 Nh3 a6 Bc4 a5", "O-O");
    }

    #[test]
    fn it_forbids_white_queenside_castle_when_d1_under_attack() {
        assert_ply_invalid("d4 d5 Nc3 Qd6 Bf4 Qxf4 e3 a6 Qg4 Qxd4", "O-O-O");
    }

    #[test]
    fn it_forbids_white_queenside_castle_when_c1_under_attack() {
        assert_ply_invalid("d4 g6 Nc3 a6 Bh6 Bxh6 Qd3 a5", "O-O-O");
    }

    #[test]
    fn it_forbids_white_queenside_castle_when_king_has_moved() {
        assert_ply_invalid(
            "e4 e5 Nf3 a6 Bxa6 bxa6 Ke2 a5 Ke1 a4 Nc3 a3 b3 c6 Bxa3 c5 Qe2",
            "O-O-O",
        );
    }

    #[test]
    fn it_forbids_white_kingside_castle_when_king_has_moved() {
        assert_ply_invalid(
            "e4 e5 Nf3 a6 Bxa6 bxa6 Ke2 a5 Ke1 a4 Nc3 a3 b3 c6 Bxa3 c5 Qe2",
            "O-O",
        );
    }

    #[test]
    fn it_forbids_white_queenside_castle_when_rook_has_moved() {
        assert_ply_invalid(
            "1. d4 d5 2. Nc3 Nc6 3. Bf4 Nf6 4. Qd3 e5 5. Rb1 a6 6. Ra1 a5",
            "O-O-O",
        );
    }

    #[test]
    fn it_forbids_white_kingside_castle_when_rook_has_moved() {
        assert_ply_invalid("e4 e5 Nf3 Nc6 Bb5 a6 Rg1 axb5 Rh1 b4", "O-O");
    }

    #[test]
    fn it_forbids_white_kingside_castle_when_f1_occupied() {
        assert_ply_invalid("1. e4 e5 2. Nf3 Nf6", "O-O");
    }

    #[test]
    fn it_forbids_white_kingside_castle_when_g1_occupied() {
        assert_ply_invalid("1. e4 e5 2. Nf3 Bd6 3. a3", "O-O");
    }

    #[test]
    fn it_forbids_white_queenside_castle_when_b1_occupied() {
        assert_ply_invalid("1. d4 d5 2. Qd3 Nc6 3. Bf4 Nf6", "O-O-O");
    }

    #[test]
    fn it_forbids_white_queenside_castle_when_c1_occupied() {
        assert_ply_invalid("1. d4 d5 2. Qd3 Nc6 3. Nc3 Nf6", "O-O-O");
    }

    #[test]
    fn it_forbids_white_queenside_castle_when_d1_occupied() {
        assert_ply_invalid("1. d4 d5 2. Nc3 Nc6 3. Bf4 Nf6", "O-O-O");
    }

    #[test]
    fn it_allows_black_kingside_castle_when_h8_under_attack() {
        assert_ply_valid("1. b3 e6 2. Bb2 Bd6 3. h3 Ne7 4. h4 g6 5. h5", "O-O");
    }

    #[test]
    fn it_allows_black_queenside_castle_when_a8_under_attack() {
        assert_ply_valid(
            "1. g3 d6 2. Bg2 b6 3. a3 Be6 4. a4 Na6 5. a5 Qd7 6. b3",
            "O-O-O",
        );
    }

    #[test]
    fn it_allows_black_queenside_castle_when_b8_under_attack() {
        assert_ply_valid(
            "1. d4 d5 2. Bf4 Bg4 3. Nc3 Na6 4. Nf3 Qd7 5. h3 c6 6. hxg4",
            "O-O-O",
        );
    }

    #[test]
    fn it_forbids_black_kingside_castle_when_in_check() {
        assert_ply_invalid("1. e3 e5 2. Qg4 Nf6 3. Nf3 Bb4 4. Qe6+", "O-O");
    }

    #[test]
    fn it_forbids_black_queenside_castle_when_in_check() {
        assert_ply_invalid(
            "1. e3 d5 2. Qf3 Nc6 3. Nc3 Bh3 4. Qe4 Qd6 5. a3 e5 6. Qxe5+",
            "O-O-O",
        );
    }

    #[test]
    fn it_forbids_black_kingside_castle_when_f8_under_attack() {
        assert_ply_invalid("1. b3 e5 2. h3 Nh6 3. h4 Ba3 4. Bxa3", "O-O");
    }

    #[test]
    fn it_forbids_black_kingside_castle_when_g8_under_attack() {
        assert_ply_invalid("1. e3 Nh6 2. Bc4 e5 3. a3 Bxa3 4. bxa3 f6 5. a4", "O-O");
    }

    #[test]
    fn it_forbids_black_queenside_castle_when_d8_under_attack() {
        assert_ply_invalid(
            "1. d4 d5 2. Bg5 Nc6 3. a3 Bg4 4. a4 e5 5. a5 Qd6 6. a6",
            "O-O-O",
        );
    }

    #[test]
    fn it_forbids_black_queenside_castle_when_c8_under_attack() {
        assert_ply_invalid("1. g3 d5 2. a3 Bh3 3. Bxh3 Nc6 4. a4 Qd6 5. a5", "O-O-O");
    }

    #[test]
    fn it_forbids_black_queenside_castle_when_king_has_moved() {
        assert_ply_invalid(
            "1. e4 d5 2. Nc3 Bh3 3. a3 Qd6 4. a4 Nc6 5. a5 Kd8 6. a6 Ke8 7. b3",
            "O-O-O",
        );
    }

    #[test]
    fn it_forbids_black_kingside_castle_when_king_has_moved() {
        assert_ply_invalid(
            "1. e4 e5 2. Nf3 Ba3 3. Nc3 Nf6 4. d4 Ke7 5. d5 Ke8 6. d6",
            "O-O",
        );
    }

    #[test]
    fn it_forbids_black_queenside_castle_when_rook_has_moved() {
        assert_ply_invalid(
            "1. e4 d5 2. Nc3 Bh3 3. a3 Nc6 4. a4 Qd6 5. a5 Rd8 6. a6 Ra8 7. b3",
            "O-O-O",
        );
    }

    #[test]
    fn it_forbids_black_kingside_castle_when_rook_has_moved() {
        assert_ply_invalid(
            "1. e4 e5 2. Nc3 Ba3 3. Nf3 Nf6 4. b3 Rg8 5. b4 Rh8 6. b5",
            "O-O",
        );
    }

    #[test]
    fn it_forbids_black_kingside_castle_when_f8_occupied() {
        assert_ply_invalid("1. e4 e5 2. Nc3 Nf6 3. a3", "O-O");
    }

    #[test]
    fn it_forbids_black_kingside_castle_when_g8_occupied() {
        assert_ply_invalid("1. e4 e5 2. Nc3 Bb4 3. a3", "O-O");
    }

    #[test]
    fn it_forbids_black_queenside_castle_when_b8_occupied() {
        assert_ply_invalid("1. e4 d5 2. a3 Bg4 3. a4 Qd6 4. a5", "O-O-O");
    }

    #[test]
    fn it_forbids_black_queenside_castle_when_c8_occupied() {
        assert_ply_invalid("1. e4 d5 2. a4 Nc6 3. a5 Qd6 4. a6", "O-O-O");
    }

    #[test]
    fn it_forbids_black_queenside_castle_when_d8_occupied() {
        assert_ply_invalid("1. e4 d5 2. a4 Nc6 3. a5 Bf5 4. a6", "O-O-O");
    }

    #[test]
    fn it_reverts_white_kingside_castle() {
        assert_reverts("1. e4 e5 2. Nf3 Nf6 3. Bc4 Nc6", "O-O");
    }

    #[test]
    fn it_reverts_white_queenside_castle() {
        assert_reverts("1. d4 d5 2. Bf4 c6 3. Qd3 e6 4. Nc3 a6", "O-O-O");
    }

    #[test]
    fn it_reverts_black_kingside_castle() {
        assert_reverts("1. e4 e5 2. Nc3 Bb4 3. Nf3 Nf6 4. a3", "O-O");
    }

    #[test]
    fn it_reverts_black_queenside_castle() {
        assert_reverts("1. e4 d5 2. Nf3 Bg4 3. Nc3 Qd6 4. a4 Nc6 5. b3", "O-O-O");
    }

    #[test]
    fn it_detects_3_fold_repetition_draw() {
        assert_draw("1. Nf3 Nf6 2. Ng1 Ng8 3. Nf3 Nf6 4. Ng1 Ng8");
    }

    #[test]
    fn it_doesnt_draw_when_almost_3_fold_rep() {
        assert_not_draw("1. Nf3 Nf6 2. Ng1 Ng8 3. Nf3 Nf6 4. Ng1");
    }

    #[test]
    fn it_doesnt_draw_when_3_fold_rep_diff_alailable_moves() {
        assert_not_draw(
            "1. e3 e6 2. Bd3 Bd6 3. Nf3 Nf6 4. Rg1 Rg8 5. Rh1 Rh8 6. Rg1 Rg8 7. Rh1 Rh8",
        );
    }

    #[test]
    fn it_detects_stalemate_draw() {
        assert_draw("1. e4 e5 2. Nf3 Nf6 3. Nxe5 Nxe4 4. Nxf7 Nxf2 5. Nxd8 Nxd1 6. Kxd1 Kxd8 7. Nc3 Nc6 8. d4 d5 9. Nxd5 Nxd4 10. Nxc7 Nxc2 11. Nxa8 Nxa1 12. Be3 Be6 13. Bxa7 Bxa2 14. Kc1 Kc8 15. b3 b6 16. Kb2 Kb7 17. Kxa2 Kxa7 18. Kxa1 Kxa8 19. b4 b5 20. Bxb5 Bxb4 21. Bc4 Bd6 22. Bd3 Bxh2 23. Bxh7 Rxh7 24. Rxh2 Rxh2 25. Kb1 Rxg2 26. Kc1 Rc2+ 27. Kxc2 g6 28. Kd2 g5 29. Ke2 g4 30. Kf1 g3 31. Kg1 g2 32. Kf2 g1=Q+ 33. Kf3 Kb7 34. Kf4 Kc6 35. Kf3 Kd6 36. Kf4 Kd5 37. Kf3 Ke5 38. Ke2 Ke4 39. Kd2 Qe3+ 40. Kc2 Kd4 41. Kb2 Kd3 42. Kb1 Kc3 43. Ka2 Qe1 44. Ka3 Qe3 45. Ka2 Kc2 46. Ka1 Kb3 47. Kb1 Qc3");
    }

    #[test]
    fn it_detects_insufficient_material_k_k_draw() {
        assert_draw("1. e4 e5 2. Nf3 Nf6 3. Nxe5 Nxe4 4. Nxf7 Nxf2 5. Nxd8 Nxd1 6. Kxd1 Kxd8 7. Nc3 Nc6 8. d4 d5 9. Nxd5 Nxd4 10. Nxc7 Nxc2 11. Nxa8 Nxa1 12. Be3 Be6 13. Bxa7 Bxa2 14. Kc1 Kc8 15. b3 b6 16. Kb2 Kb7 17. Kxa2 Kxa7 18. Kxa1 Kxa8 19. b4 b5 20. Bxb5 Bxb4 21. Bc4 Bd6 22. Bd3 Bxh2 23. Bxh7 Rxh7 24. g4 Bf4 25. Rxh7 Be3 26. Rxg7 Bd4+ 27. Ka2 Bxg7 28. Kb3 Kb7 29. Kc4 Kc7 30. Kd5 Kd7 31. Ke4 Bd4 32. Kxd4 Ke7 33. g5 Kf7 34. g6+ Kxg6");
    }

    #[test]
    fn it_detects_insufficient_material_kb_k_draw() {
        assert_draw("1. e4 e5 2. Nf3 Nf6 3. Nxe5 Nxe4 4. Nxf7 Nxf2 5. Nxd8 Nxd1 6. Kxd1 Kxd8 7. Nc3 Nc6 8. d4 d5 9. Nxd5 Nxd4 10. Nxc7 Nxc2 11. Nxa8 Nxa1 12. Be3 Be6 13. Bxa7 Bxa2 14. Kc1 Kc8 15. b3 b6 16. Kb2 Kb7 17. Kxa2 Kxa7 18. Kxa1 Kxa8 19. b4 b5 20. Bxb5 Bxb4 21. Bc4 Bd6 22. Bd3 Bxh2 23. Bxh7 Rxh7 24. g4 Bf4 25. Rxh7 Be3 26. Rxg7 Bd4+ 27. Ka2 Bxg7 28. g5 Bf6 29. Kb3 Bxg5");
    }

    #[test]
    fn it_detects_insufficient_material_kn_k_draw() {
        assert_draw("1. e4 e5 2. Nf3 Nf6 3. Nxe5 Nxe4 4. Nxf7 Nxf2 5. Nxd8 Nxd1 6. Kxd1 Kxd8 7. Nc3 Nc6 8. d4 d5 9. Bf4 Bb4 10. a3 Bxc3 11. bxc3 Nxd4 12. c4 dxc4 13. Bxc4 Nxc2 14. Bxc7+ Kxc7 15. Bd5 Nxa1 16. Bxb7 Nc2 17. Bxa8 Nxa3 18. Be4 Bb7 19. Bxh7 g6 20. Bxg6 Bxg2 21. h3 Bxh1 22. Be8 Rxe8 23. h4 Rh8 24. Ke1 Rxh4 25. Kf1 Bg2+ 26. Kxg2 a5 27. Kg3 a4 28. Kxh4 Nc4 29. Kg3 Nd6 30. Kf3 a3 31. Ke3 a2 32. Kd2 a1=Q 33. Kc2 Qc1+ 34. Kxc1");
    }

    #[test]
    fn it_detects_insufficient_material_kb_kb_same_color_draw() {
        assert_draw("1. e4 e5 2. Nf3 Nf6 3. Nxe5 Nxe4 4. Nxf7 Nxf2 5. Nxh8 Nxh1 6. Nc3 Nc6 7. d4 d6 8. d5 Bf5 9. dxc6 Qg5 10. cxb7 Be7 11. Ne4 Rb8 12. Nxd6+ Kf8 13. Ng6+ Qxg6 14. Be3 Rxb7 15. Nxb7 c5 16. Nxc5 a6 17. Nxa6 Bd3 18. Qf3+ Bf6 19. c3 Bxa6 20. Bxa6 Ke8 21. Qf1 Qxg2 22. Qxh1 Qxh1+ 23. Kd2 Qxa1 24. a4 Qxa4 25. b3 Qxa6 26. b4 Qb6 27. c4 Qxb4+ 28. Kd3 Qxc4+ 29. Kd2 Qh4 30. Bd4 Qxh2+ 31. Kd3 Bg5 32. Bxg7 h6 33. Bxh6 Bf4 34. Bg7 Qb2 35. Bxb2");
    }

    #[test]
    fn it_doesnt_draw_insufficient_material_kb_kb_diff_color_draw() {
        assert_not_draw("1. e4 e5 2. Nf3 Nf6 3. Nxe5 Nxe4 4. Nxf7 Nxf2 5. Nxh8 Nxh1 6. Nc3 Nc6 7. d4 d6 8. d5 Bf5 9. dxc6 Qg5 10. cxb7 Be7 11. Ne4 Rb8 12. Nxd6+ Kf8 13. Ng6+ Qxg6 14. Be3 Rxb7 15. Nxb7 c5 16. Nxc5 a6 17. Nxa6 Bd3 18. Qf3+ Bf6 19. c3 Bxa6 20. Bxa6 Ke8 21. Qf1 Qxg2 22. Qxh1 Qxh1+ 23. Kd2 Qxa1 24. a4 Qxa4 25. Bd4 Qxd4+ 26. cxd4 Bxd4 27. h4 Bxb2 28. Bd3 Bf6 29. Bxh7 Bxh4 30. Bf5 g6 31. Ke3 Kd8 32. Bxg6");
    }

    #[bench]
    fn bench_get_valid_plies(b: &mut Bencher) {
        let mut g = make_game("d4 d5 e4 e5 Nc3 Nc6 Nf3 Nf6");
        b.iter(|| g.get_valid_plies());
    }

    #[bench]
    fn bench_has_valid_plies_simple(b: &mut Bencher) {
        let mut g = make_game("d4 d5 e4 e5 Nc3 Nc6 Nf3 Nf6");
        b.iter(|| g.has_valid_plies());
    }

    #[bench]
    fn bench_has_valid_plies_complex(b: &mut Bencher) {
        let mut g = make_game("1. e4 e5 2. Nf3 Nf6 3. Nxe5 Nxe4 4. Nxf7 Nxf2 5. Nxd8 Nxd1 6. Kxd1 Kxd8 7. Nc3 Nc6 8. d4 d5 9. Nxd5 Nxd4 10. Nxc7 Nxc2 11. Nxa8 Nxa1 12. Be3 Be6 13. Bxa7 Bxa2 14. Kc1 Kc8 15. b3 b6 16. Kb2 Kb7 17. Kxa2 Kxa7 18. Kxa1 Kxa8 19. b4 b5 20. Bxb5 Bxb4 21. Bc4 Bd6 22. Bd3 Bxh2 23. Bxh7 Rxh7 24. Rxh2 Rxh2 25. Kb1 Rxg2 26. Kc1 Rc2+ 27. Kxc2 g6 28. Kd2 g5 29. Ke2 g4 30. Kf1 g3 31. Kg1 g2 32. Kf2 g1=Q+ 33. Kf3 Kb7 34. Kf4 Kc6 35. Kf3 Kd6 36. Kf4 Kd5 37. Kf3 Ke5 38. Ke2 Ke4 39. Kd2 Qe3+ 40. Kc2 Kd4 41. Kb2 Kd3 42. Kb1 Kc3 43. Ka2 Qe1 44. Ka3 Qe3 45. Ka2 Kc2 46. Ka1 Kb3 47. Kb1 Qc3");
        b.iter(|| g.has_valid_plies());
    }

    #[bench]
    fn bench_is_draw_simple(b: &mut Bencher) {
        let mut g = make_game("1. e4 d5 2. Nf3 Bg4 3. Nc3 Qd6 4. a4 Nc6 5. b3");
        b.iter(|| g.is_draw(true));
    }

    #[bench]
    fn bench_is_draw_complex(b: &mut Bencher) {
        let mut g = make_game("1. Nf3 Nf6 2. Ng1 Ng8 3. Nf3 Nf6 4. Ng1 Ng8");
        b.iter(|| g.is_draw(true));
    }
}
