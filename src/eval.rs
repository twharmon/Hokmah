use crate::color::Color;
use crate::file::File;
use crate::game::Game;
use crate::kind::Kind;
use crate::position::Position;
use crate::params::Params;
use crate::position::{ALL_POSITIONS, CENTER_POSITIONS};
use crate::rank::Rank;

pub const MIN_EVAL: i16 = -30_000;
pub const MAX_EVAL: i16 = 30_000;

pub fn evaluate(game: &mut Game, params: &Params, maximizing_player_color: Color) -> i16 {
    let has_valid_plies = game.has_valid_plies();
    if !has_valid_plies {
        if game.is_in_check() {
            return if maximizing_player_color == game.turn {
                MIN_EVAL
            } else {
                MAX_EVAL
            };
        }
        return 0
    }
    if game.is_draw(has_valid_plies) {
        return 0
    }

    let hist_len = game.history.len();
    let mut value = 0;
    let mut player_bishop_cnt: u8 = 0;
    let mut enemy_bishop_cnt: u8 = 0;
    let mut player_pawn_cnts: [i16; 8] = [0; 8];
    let mut enemy_pawn_cnts: [i16; 8] = [0; 8];
    for position in ALL_POSITIONS.iter() {
        match game.get_piece(position) {
            Some(p) => {
                let val = p.value(position, params);
                if p.color == game.turn {
                    value += val;
                    match p.kind {
                        Kind::Bishop => player_bishop_cnt += 1,
                        Kind::Pawn => player_pawn_cnts[usize::from(position.file)] += 1,
                        _ => (),
                    };
                } else {
                    value -= val;
                    match p.kind {
                        Kind::Bishop => enemy_bishop_cnt += 1,
                        Kind::Pawn => enemy_pawn_cnts[usize::from(position.file)] += 1,
                        _ => (),
                    };
                }
            }
            _ => (),
        };
    }

    if player_bishop_cnt == 2 {
        value += params.double_bishop_reward;
    }
    if enemy_bishop_cnt == 2 {
        value -= params.double_bishop_reward;
    }
    for cnt in &player_pawn_cnts {
        if cnt > &1 {
            value -= params.doubled_pawn_penalty * (cnt - 1);
        }
    }
    for cnt in &enemy_pawn_cnts {
        if cnt > &1 {
            value += params.doubled_pawn_penalty * (cnt - 1);
        }
    }

    if hist_len < 30 {
        let enemy_color = match game.turn {
            Color::White => Color::Black,
            _ => Color::White,
        };
        for position in &CENTER_POSITIONS {
            value -=
                params.center_control_bonus * game.count_attackers(position, &enemy_color);
            value +=
                params.center_control_bonus * game.count_attackers(position, &game.turn);
        }
    }

    let player_king_position = game.get_king_position();
    game.switch_turns();
    let enemy_king_position = game.get_king_position();
    game.switch_turns();

    let player_naive_material = game.get_player_naive_material();
    let enemy_naive_material = game.get_enemy_naive_material();
    
    if player_naive_material + enemy_naive_material > 40 {
        let valid_ply_cnt_diff = game.valid_non_king_plies_diff();
        value += ((valid_ply_cnt_diff as f32 / 15f32).min(1f32) * params.many_moves_bonus as f32) as i16;
    }

    if player_naive_material < 10 {
        value -=
            get_lonely_king_cornered_penalty(game, &player_king_position) * params.lonely_king_cornered_penalty_factor;
    }
    if enemy_naive_material < 10 {
        game.switch_turns();
        value +=
            get_lonely_king_cornered_penalty(game, &enemy_king_position) * params.lonely_king_cornered_penalty_factor;
        game.switch_turns();
    }

    if player_naive_material > 20 {
        value += get_king_protection_reward(game, params, &player_king_position);
    }

    if enemy_naive_material > 20 {
        game.switch_turns();
        value -= get_king_protection_reward(game, params, &enemy_king_position);
        game.switch_turns();
    }

    if maximizing_player_color == game.turn {
        value
    } else {
        0 - value
    }
}

fn get_king_protection_reward(game: &Game, params: &Params, king_position: &Position) -> i16 {
    match king_position.file {
        File::D | File::E => return 0,
        _ => (),
    };
    if (game.turn == Color::White && king_position.rank != Rank::One) || (game.turn == Color::Black && king_position.rank != Rank::Eight) {
        return 0
    }
    
    let mut reward = 0;
    match king_position.file {
        File::A => reward += params.max_king_corner_reward / 2,
        File::B => if has_starting_rank_left_rook(game, king_position.file.into(), king_position.rank.into()) { return 0 } else { reward += params.max_king_corner_reward },
        File::C => if has_starting_rank_left_rook(game, king_position.file.into(), king_position.rank.into()) { return 0 } else { reward += params.max_king_corner_reward / 2 },
        
        File::H => reward += params.max_king_corner_reward / 2,
        File::G => if has_starting_rank_right_rook(game, king_position.file.into(), king_position.rank.into()) { return 0 } else { reward += params.max_king_corner_reward },
        File::F => if has_starting_rank_right_rook(game, king_position.file.into(), king_position.rank.into()) { return 0 } else { reward += params.max_king_corner_reward / 2 },

        _ => (),
    };
    
    match game.turn {
        Color::White => {
            reward += get_protecting_pawn_reward(game, king_position.step_n(), params);
            reward += get_protecting_pawn_reward(game, king_position.step_ne(), params);
            reward += get_protecting_pawn_reward(game, king_position.step_nw(), params);
            reward += get_protecting_pawn_reward(game, king_position.step_nn(), params);
            match king_position.file {
                File::A | File::B | File::C => reward += get_protecting_pawn_reward(game, king_position.step_nnw(), params),
                _ => reward += get_protecting_pawn_reward(game, king_position.step_nne(), params),
            };
        },
        _ => {
            reward += get_protecting_pawn_reward(game, king_position.step_s(), params);
            reward += get_protecting_pawn_reward(game, king_position.step_se(), params);
            reward += get_protecting_pawn_reward(game, king_position.step_sw(), params);
            reward += get_protecting_pawn_reward(game, king_position.step_ss(), params);
            match king_position.file {
                File::A | File::B | File::C => reward += get_protecting_pawn_reward(game, king_position.step_ssw(), params),
                _ => reward += get_protecting_pawn_reward(game, king_position.step_sse(), params),
            };
        },
    };
    reward
}

fn has_starting_rank_right_rook(game: &Game, king_file: usize, king_rank: usize) -> bool {
     for f in king_file + 1..8 {
        match game.board[king_rank][f] {
            Some(p) => {
                if p.kind == Kind::Rook && p.color == game.turn {
                    return true
                }
            }
            None => (),
        };
    }
    false
}

fn has_starting_rank_left_rook(game: &Game, king_file: usize, king_rank: usize) -> bool {
     for f in 0..king_file {
        match game.board[king_rank][f] {
            Some(p) => {
                if p.kind == Kind::Rook && p.color == game.turn {
                    return true
                }
            }
            None => (),
        };
    }
    false
}

fn get_protecting_pawn_reward(game: &Game, position: Option<Position>, params: &Params) -> i16 {
    match position {
        Some(pos) => match game.get_piece(&pos) {
            Some(p) => {
                if p.color == game.turn && p.kind == Kind::Pawn {
                    params.max_pawn_protection_reward
                } else {
                    0
                }
            }
            None => 0,
        },
        None => 0,
    }
}

fn get_lonely_king_cornered_penalty(game: &Game, king_position: &Position) -> i16 {
    let mut penalty = 0;
    match king_position.file {
        File::A | File::H => penalty += 20,
        File::B | File::G => penalty += 15,
        File::C | File::F => penalty += 10,
        _ => (),
    };
    match king_position.rank {
        Rank::One | Rank::Eight => penalty += 20,
        Rank::Two | Rank::Seven => penalty += 15,
        Rank::Three | Rank::Six => penalty += 10,
        _ => (),
    };

    penalty += get_immediate_surrounding_penalty(game, king_position.step_n());
    penalty += get_immediate_surrounding_penalty(game, king_position.step_ne());
    penalty += get_immediate_surrounding_penalty(game, king_position.step_e());
    penalty += get_immediate_surrounding_penalty(game, king_position.step_se());
    penalty += get_immediate_surrounding_penalty(game, king_position.step_s());
    penalty += get_immediate_surrounding_penalty(game, king_position.step_sw());
    penalty += get_immediate_surrounding_penalty(game, king_position.step_w());
    penalty += get_immediate_surrounding_penalty(game, king_position.step_nw());

    penalty += get_secondary_surrounding_penalty(game, king_position.step_nn());
    penalty += get_secondary_surrounding_penalty(game, king_position.step_nne());
    penalty += get_secondary_surrounding_penalty(game, king_position.step_nene());
    penalty += get_secondary_surrounding_penalty(game, king_position.step_ene());
    penalty += get_secondary_surrounding_penalty(game, king_position.step_ee());
    penalty += get_secondary_surrounding_penalty(game, king_position.step_ese());
    penalty += get_secondary_surrounding_penalty(game, king_position.step_sese());
    penalty += get_secondary_surrounding_penalty(game, king_position.step_sse());
    penalty += get_secondary_surrounding_penalty(game, king_position.step_ss());
    penalty += get_secondary_surrounding_penalty(game, king_position.step_ssw());
    penalty += get_secondary_surrounding_penalty(game, king_position.step_swsw());
    penalty += get_secondary_surrounding_penalty(game, king_position.step_wsw());
    penalty += get_secondary_surrounding_penalty(game, king_position.step_ww());
    penalty += get_secondary_surrounding_penalty(game, king_position.step_wnw());
    penalty += get_secondary_surrounding_penalty(game, king_position.step_nwnw());
    penalty += get_secondary_surrounding_penalty(game, king_position.step_nnw());

    penalty
}

fn get_immediate_surrounding_penalty(game: &Game, position: Option<Position>) -> i16 {
    match position {
        Some(p) => match game.get_piece(&p) {
            Some(p) => if p.color != game.turn {
                2
            } else {
                0
            }
            None => 0,
        }
        None => 0,
    }
}

fn get_secondary_surrounding_penalty(game: &Game, position: Option<Position>) -> i16 {
    match position {
        Some(p) => match game.get_piece(&p) {
            Some(p) => if p.color != game.turn {
                1
            } else {
                0
            }
            None => 0,
        }
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::evaluate;
    use super::MIN_EVAL;
    use crate::color::Color;
    use crate::params::Params;
    use crate::tests::make_game;
    use test::Bencher;

    #[test]
    fn it_evals_checkmate_correctly() {
        let mut g = make_game("1. e4 d5 2. exd5 Qxd5 3. Nc3 Qd4 4. Nf3 Qf6 5. d4 Bg4 6. Bg5 Qe6+ 7. Be2 Bxf3 8. gxf3 h6 9. Bf4 Nd7 10. Bxc7 Qc6 11. Be5 Nxe5 12. dxe5 Rd8 13. Bd3 h5 14. Qd2 g6 15. O-O-O Bh6 16. f4 Kf8 17. Kb1 Bg7 18. Be4 Rxd2 19. Rxd2 Qb6 20. Nd5 Qd8 21. Nf6 Qxd2 22. Nxg8 Kxg8 23. Bxb7 Rh6 24. Re1 Qxe1#");
        let eval = evaluate(&mut g, &Params::get(), Color::White);
        assert_eq!(eval, MIN_EVAL);
    }

    #[bench]
    fn bench_eval(b: &mut Bencher) {
        let mut g = make_game("d4 d5 e4 e5 Nc3 Nc6 Nf3 Nf6");
        let params = Params::get();
        b.iter(|| evaluate(&mut g, &params, Color::White));
    }
}
