pub enum Event {
    Jackpot,
    LuckBreak,
    Normal,
}

pub struct Odds {
    pub good: u8,
    pub bad: u8,
}

pub struct Game {
    pub credits: i32,
    pub highest_score: i32,
    pub odds: Odds,
}
