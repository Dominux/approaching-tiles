use rand::seq::SliceRandom;

pub fn get_random_symbol() -> String {
    ["🪭", "🦕", "🦔", "🐧", "🍑", "🍆", "🥒", "💎"]
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string()
}
