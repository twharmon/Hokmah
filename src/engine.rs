use crate::color::Color;
use crate::eval::evaluate;
use crate::game::Game;
use crate::kind::Kind;
use crate::params::Params;
use crate::ply::Ply;
use crate::traits::BestFirstSort;
use crossbeam::crossbeam_channel;
use std::time::{Duration, SystemTime};
use std::ops::{Sub, Div, BitXor};
use crate::cache::Cache;
use std::sync::{Arc, Mutex};

pub fn suggest(g: Game, params: Params, limit: Duration) -> (Ply, i16) {    
    let start = SystemTime::now();
    let mut depth = 1;
    let cache = Cache::new();
    let future_cache = Cache::new();

    let (mut ply, mut eval) = search(g.clone(), params, depth, 0, &cache, &future_cache);
    
    while SystemTime::now().sub(limit.div(4)) < start {
        depth += 1;
        let (p, e) = search(g.clone(), params, depth, 0, &cache, &future_cache);
        ply = p;
        eval = e;
    }

    (ply, eval)
}

pub fn search(mut g: Game, params: Params, depth: u8, extension: u8, cache: &Arc<Mutex<Cache>>, future_cache: &Arc<Mutex<Cache>>) -> (Ply, i16) {
    let valid_plies = g.get_valid_plies();
    let valid_ply_cnt = valid_plies.len();

    let (s, r) = crossbeam_channel::unbounded();
    for ply in valid_plies {
        let sender = s.clone();
        let mut g = g.clone();
        let cache = cache.clone();
        let future_cache = future_cache.clone();
        std::thread::spawn(move || {
            let color = g.turn;
            g.do_ply(ply, false);
            let value = minimax(depth - 1, &mut g, &params, -30_000, 30_000, color, extension, &cache, &future_cache);
            sender.send((ply, value)).expect("unable to send");
        });
    }

    let mut best_eval = -30_000;
    let mut best_plies_found = Vec::new();
    for _ in 0..valid_ply_cnt {
        let (ply, value) = r.recv().expect("unable to receive");
        if value == best_eval {
            best_plies_found.push(ply);
        } else if value > best_eval {
            best_eval = value;
            best_plies_found.clear();
            best_plies_found.push(ply);
        }
    }

    (*best_plies_found.first().unwrap(), best_eval)
}

fn minimax(
    mut depth: u8,
    g: &mut Game,
    params: &Params,
    mut alpha: i16,
    mut beta: i16,
    maximizing_player_color: Color,
    mut extension: u8,
    cache: &Arc<Mutex<Cache>>,
    future_cache: &Arc<Mutex<Cache>>,
) -> i16 {
    if depth == 0 {
        let mut c = cache.lock().unwrap();
        match c.get(g.hash) {
            Some(e) => return (e as f32 * (1f32 + (depth as f32 / 100f32))) as i16,
            None => (),
        };
        let eval = evaluate(g, params, maximizing_player_color);
        c.set(g.hash, eval);
        return (eval as f32 * (1f32 + (depth as f32 / 100f32))) as i16;
    }

    let mut valid_plies = g.get_valid_plies();
    if valid_plies.is_empty() {
        let mut c = cache.lock().unwrap();
        match c.get(g.hash) {
            Some(e) => return (e as f32 * (1f32 + (depth as f32 / 100f32))) as i16,
            None => (),
        };
        let eval = evaluate(g, params, maximizing_player_color);
        c.set(g.hash, eval);
        return (eval as f32 * (1f32 + (depth as f32 / 100f32))) as i16;
    }

    valid_plies.best_first_sort(depth, g, future_cache);

    if maximizing_player_color == g.turn {
        let mut best_ply = -30_000;
        for ply in valid_plies {
            g.do_ply(ply, false);
            if depth == 1 && extension > 0 {
                if let Some(t) = ply.target {
                    match t.kind {
                        Kind::Pawn => (),
                        _ => {
                            depth += 1;
                            extension -= 1;
                        }
                    };
                } else if let Some(_) = ply.promotion {
                    depth += 1;
                    extension -= 1;
                }
            }
            let eval = minimax(
                depth - 1,
                g,
                params,
                alpha,
                beta,
                maximizing_player_color,
                extension,
                cache,
                future_cache,
            );
            best_ply = best_ply.max(eval);
            if depth > 2 {
                let mut c = future_cache.lock().unwrap();
                c.set(g.hash.bitxor(g.turn.hash()), eval);
            }
            g.revert_last_ply();
            alpha = alpha.max(best_ply);
            if beta <= alpha {
                return best_ply;
            }
        }
        best_ply
    } else {
        let mut best_ply = 30_000;
        for ply in valid_plies {
            g.do_ply(ply, false);
            if depth == 1 && extension > 0 {
                if let Some(t) = ply.target {
                    match t.kind {
                        Kind::Pawn => (),
                        _ => {
                            depth += 1;
                            extension -= 1;
                        }
                    };
                } else if let Some(_) = ply.promotion {
                    depth += 1;
                    extension -= 1;
                }
            }
            let eval = minimax(
                depth - 1,
                g,
                params,
                alpha,
                beta,
                maximizing_player_color,
                extension,
                cache,
                future_cache,
            );
            best_ply = best_ply.min(eval);
            if depth > 2 {
                let mut c = future_cache.lock().unwrap();
                c.set(g.hash.bitxor(g.turn.hash()), eval);
            }
            g.revert_last_ply();
            beta = beta.min(best_ply);
            if beta <= alpha {
                return best_ply;
            }
        }
        best_ply
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::make_game;
    use crate::params::Params;
    use crate::cache::Cache;
    use super::search;
    use test::Bencher;

    const END_GAME_PGN: &'static str = "1. e4 e5 2. Nf3 Nf6 3. Nc3 Nc6 4. Nxe5 Nxe4 5. Nxe4 Nxe5 6. Qf3 Nxf3+ 7. gxf3 Qf6 8. Nxf6+ gxf6 9. d4 d5 10. Bf4 Bf5 11. Bb5+ c6 12. Bxc6+ Kd8 13. Bxb7 Bb4+ 14. c3 Bxc3+ 15. Kd1 Bxb2 16. Bxa8 Bxa1 17. Be5 Be4 18. fxe4 fxe5 19. dxe5 dxe4 20. f3 exf3 21. e6 fxe6 22. Rf1 e5 23. Rxf3 Rf8 24. Ke2 Bd4 25. Bd5 e4 26. Bxe4";
    const MID_GAME_PGN: &'static str = "1. e4 e5 2. Nf3 Nc6 3. Nc3 Nf6 4. Bb5 a6 5. Bxc6 dxc6 6. d4 exd4 7. Nxd4 Bc5 8. Be3 O-O 9. Qd3 Bg4 10. f3 Bh5 11. g4 Bg6 12. O-O-O b5 13. Nf5 Bxe3+ 14. Qxe3 Qc8";

    #[test]
    fn it_chooses_fools_mate() {
        let g = make_game("f4 e5 g4");
        let cache = Cache::new();
        let future_cache = Cache::new();
        let (ply, eval) = search(g, Params::get(), 2, 0, &cache, &future_cache);
        println!("eval: {}", eval);
        assert_eq!(ply.uci(), String::from("d8h4"));
    }

    #[test]
    fn it_evals_bonus_for_quick_mate() {
        let g = make_game("f4 e5 g4");
        let cache = Cache::new();
        let future_cache = Cache::new();
        let (_, eval) = search(g, Params::get(), 4, 0, &cache, &future_cache);
        assert_eq!(eval > 30_000, true);
    }

    #[bench]
    fn bench_search_depth_2_mid_game(b: &mut Bencher) {
        let params = Params::get();
        let cache = Cache::new();
        let future_cache = Cache::new();
        let g = make_game(MID_GAME_PGN);
        b.iter(|| search(g.clone(), params, 2, 0, &cache, &future_cache));
    }

    #[bench]
    fn bench_search_depth_3_mid_game(b: &mut Bencher) {
        let params = Params::get();
        let cache = Cache::new();
        let future_cache = Cache::new();
        let g = make_game(MID_GAME_PGN);
        b.iter(|| search(g.clone(), params, 3, 0, &cache, &future_cache));
    }

    // #[bench]
    // fn bench_search_depth_4_mid_game(b: &mut Bencher) {
    //     let params = Params::get();
    //     let cache = Cache::new();
    //     let future_cache = Cache::new();
    //     let g = make_game(MID_GAME_PGN);
    //     b.iter(|| search(g.clone(), params, 4, 0, &cache, &future_cache));
    // }

    // #[bench]
    // fn bench_search_depth_5_mid_game(b: &mut Bencher) {
    //     let params = Params::get();
    //     let cache = Cache::new();
    //     let future_cache = Cache::new();
    //     let g = make_game(MID_GAME_PGN);
    //     b.iter(|| search(g.clone(), params, 5, 0, &cache, &future_cache));
    // }

    #[bench]
    fn bench_search_depth_4_end_game(b: &mut Bencher) {
        let params = Params::get();
        let cache = Cache::new();
        let future_cache = Cache::new();
        let g = make_game(END_GAME_PGN);
        b.iter(|| search(g.clone(), params, 4, 0, &cache, &future_cache));
    }

    // #[bench]
    // fn bench_search_depth_5_end_game(b: &mut Bencher) {
    //     let params = Params::get();
    //     let g = make_game(END_GAME_PGN);
    //     b.iter(|| search(g.clone(), params, 5, 0));
    // }

    // #[bench]
    // fn bench_search_depth_4(b: &mut Bencher) {
    //     let params = Params::get();
    //     let g = make_game("1. e4 e5 2. Nf3 Nc6 3. Nc3 Nf6 4. Bb5 a6 5. Bxc6 dxc6 6. d4 exd4 7. Nxd4 Bc5 8. Be3 O-O 9. Qd3 Bg4 10. f3 Bh5 11. g4 Bg6 12. O-O-O b5 13. Nf5 Bxe3+ 14. Qxe3 Qc8");
    //     b.iter(|| search(g.clone(), params, 4, 0));
    // }

    // #[bench]
    // fn bench_search_depth_5(b: &mut Bencher) {
    //     let params = Params::get();
    //     let g = make_game("1. e4 e5 2. Nf3 Nc6 3. Nc3 Nf6 4. Bb5 a6 5. Bxc6 dxc6 6. d4 exd4 7. Nxd4 Bc5 8. Be3 O-O 9. Qd3 Bg4 10. f3 Bh5 11. g4 Bg6 12. O-O-O b5 13. Nf5 Bxe3+ 14. Qxe3 Qc8");
    //     b.iter(|| search(g.clone(), params, 5, 0));
    // }
}
