// backend/src/engine/game.rs

#[derive(Default, Clone, Debug)]
pub struct Game {
    pub score: u32,
}

impl Game {
    /// Tier label based on μ-level and whether the highest μ was mined.
    pub fn rank_from_level(mu_level: u8, is_mined: bool) -> &'static str {
        if !is_mined {
            return "🧾 μOracle";
        }
        match mu_level {
            15 => "🧭 μScout",
            16..=17 => "🔨 μForged",
            18 => "🦁 μLegend",
            19..=20 => "🧙 μMythic",
            21..=u8::MAX => "🦍 μHonorius",
            _ => "❓ Unknown",
        }
    }
}
