use crate::color::Color;
use crate::game::Game;
use crate::cache::Cache;
use crate::{engine::search, params::Params};
use std::io::Write;

pub fn train() {
    let max_game_cnt = 512;
    loop {
        let start = std::time::SystemTime::now();

        let a_params = Params::get();
        let mut b_params = a_params.clone();
        let (param_name, param_val) = b_params.randomly_mutate();
        println!("");
        println!("---------------------------");
        println!("");
        println!("testing params: {} = {}", param_name, param_val);

        let mut a_wins = 0f32;
        let mut b_wins = 0f32;

        let mut games_played = 0;
        for game_index in 0..max_game_cnt {
            if game_index % 2 == 0 {
                match play_game(a_params, b_params) {
                    Some(v) => match v {
                        Color::White => a_wins += 1.0,
                        _ => b_wins += 1.0,
                    },
                    None => {
                        a_wins += 0.5;
                        b_wins += 0.5;
                    }
                };
            } else {
                match play_game(b_params, a_params) {
                    Some(v) => match v {
                        Color::White => b_wins += 1.0,
                        _ => a_wins += 1.0,
                    },
                    None => {
                        a_wins += 0.5;
                        b_wins += 0.5;
                    }
                };
            }
            games_played += 1;
            let diff = b_wins - a_wins;
            let std_dev = (games_played as f32 / 4.0).sqrt();
            if games_played > 20 {
                if diff.abs() >= std_dev * 4.5 {
                    if diff > 0.0 {
                        match serde_yaml::to_string(&b_params) {
                            Ok(s) => {
                                let mut file = match std::fs::File::create("./params.yml") {
                                    Ok(f) => f,
                                    Err(e) => panic!(e),
                                };
                                match file.write(s.as_ref()) {
                                    Ok(_) => (),
                                    Err(e) => panic!(e),
                                };
                            }
                            Err(e) => panic!(e),
                        };
                        println!("winning params overthrown");
                    }
                    break
                }
            }
            if games_played > 40 && diff <= std_dev * 0.5 {
                break
            }
        }
        println!("winning params: {}, test params: {}", a_wins, b_wins);
        let end = std::time::SystemTime::now();
        println!(
            "{} seconds per game",
            end.duration_since(start).unwrap().as_secs() as f32 / games_played as f32
        );
    }
}

fn play_game(white_params: Params, black_params: Params) -> Option<Color> {
    let g = &mut Game::new();
    let cache = Cache::new();
    let future_cache = Cache::new();
    loop {
        let mat = g.get_player_naive_material() + g.get_enemy_naive_material();

        let depth = if mat < 6 {
            6
        } else if mat < 12 {
            5
        } else if mat < 20 {
            4
        } else if mat < 28 {
            3
        } else {
            2
        };

        let ply = search(
            g.clone(),
            match g.turn {
                Color::White => white_params,
                _ => black_params,
            },
            depth,
            1,
            &cache,
            &future_cache,
        );

        g.do_ply(ply, true);

        match g.is_over {
            true => return g.victor,
            false => (),
        };
    }
}
