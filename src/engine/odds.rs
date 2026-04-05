use crate::{constants::CHANCE_CAP, engine::Event};

#[derive(Copy, Clone)]
pub struct Odds {
    jackpot: u8,
    luck_break: u8,
}

impl Odds {
    pub fn new(jackpot: u8, luck_break: u8) -> Self {
        assert!(jackpot <= CHANCE_CAP);
        assert!(luck_break <= CHANCE_CAP);
        Self {
            jackpot,
            luck_break,
        }
    }

    pub fn jackpot(&self) -> u8 {
        self.jackpot
    }

    pub fn luck_break(&self) -> u8 {
        self.luck_break
    }

    pub fn grow(&mut self, jackpot: u8, luck_break: u8, event: Event) {
        self.jackpot = (self.jackpot + jackpot).min(CHANCE_CAP);
        self.luck_break = (self.luck_break + luck_break).min(CHANCE_CAP);
        match event {
            Event::Jackpot => self.jackpot = 0,
            Event::LuckBreak => self.luck_break = 0,
            Event::Normal => (),
        };
    }

    pub fn select_event(&self, roll: u8) -> Event {
        debug_assert!(roll <= 100);

        let jackpot_max = self.jackpot;
        let luck_break_min = 100 - self.luck_break;

        if roll <= jackpot_max {
            Event::Jackpot
        } else if roll >= luck_break_min {
            Event::LuckBreak
        } else {
            Event::Normal
        }
    }
}
