use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum Notes {
    E1,
    C4,
    E4,
    G6
}

pub const FREQS: HashMap<Notes, f32> = HashMap::from([
    (Notes::E1, 41.2),
    (Notes::C4, 261.6),
    (Notes::E4, 329.6),
    (Notes::G6, 1586.0)
]);
