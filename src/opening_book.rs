pub fn search(state: &str) -> Option<String> {
    match state {
        "" => Some("e2e4".into()),

        "a2a3" => Some("g8f6".into()),
        "a2a4" => Some("d7d5".into()),
        "b2b3" => Some("d7d5".into()),
        "b2b4" => Some("e7e5".into()),
        "c2c3" => Some("d7d5".into()),
        "c2c4" => Some("g8f6".into()),
        "d2d3" => Some("d7d5".into()),
        "d2d4" => Some("g8f6".into()),
        "e2e3" => Some("c7c5".into()),
        "e2e4" => Some("c7c5".into()),
        "f2f3" => Some("e7e5".into()),
        "f2f4" => Some("d7d5".into()),
        "g2g3" => Some("g7g6".into()),
        "g2g4" => Some("d7d5".into()),
        "h2h3" => Some("e7e5".into()),
        "h2h4" => Some("e7e6".into()),
        "b1c3" => Some("d7d5".into()),
        "b1a3" => Some("e7e5".into()),
        "g1f3" => Some("d7d5".into()),
        "g1h3" => Some("e7e5".into()),

        "e2e4 e7e5" => Some("g1f3".into()),
        "e2e4 d7d5" => Some("e4d5".into()),

        _ => None,
    }
}
