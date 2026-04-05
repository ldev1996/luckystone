use crate::{constants::CHANCE_CAP, engine::Event};

/// Represents the dynamic event probabilities used by the game.
///
/// `Odds` stores the current chance of two special events:
///
/// - `Jackpot`
/// - `LuckBreak`
///
/// These probabilities grow gradually over time until the corresponding
/// event occurs, at which point that event's probability is reset to zero.
///
/// The remaining probability space results in a `Normal` event.
#[derive(Copy, Clone)]
pub struct Odds {
    jackpot: u8,
    luck_break: u8,
}

/// Creates a new probability state.
///
/// Both probabilities must be less than or equal to [`CHANCE_CAP`].
/// This value represents the maximum allowed probability percentage
/// for each event.
impl Odds {
    pub fn new(jackpot: u8, luck_break: u8) -> Self {
        assert!(jackpot <= CHANCE_CAP);
        assert!(luck_break <= CHANCE_CAP);
        Self {
            jackpot,
            luck_break,
        }
    }

    /// Returns the current probability of a `Jackpot` event.
    pub fn jackpot(&self) -> u8 {
        self.jackpot
    }

    /// Returns the current probability of a `Luck Break` event.
    pub fn luck_break(&self) -> u8 {
        self.luck_break
    }

    /// Increases the event probabilities and resets the one that occurred.
    ///
    /// Both probabilities grow by the provided values but never exceed
    /// [`CHANCE_CAP`]. If the current turn triggered a special event,
    /// its corresponding probability is reset to zero.
    pub fn grow(&mut self, jackpot: u8, luck_break: u8, event: Event) {
        self.jackpot = (self.jackpot + jackpot).min(CHANCE_CAP);
        self.luck_break = (self.luck_break + luck_break).min(CHANCE_CAP);
        match event {
            Event::Jackpot => self.jackpot = 0,
            Event::LuckBreak => self.luck_break = 0,
            Event::Normal => (),
        };
    }

    /// Determines which event occurs for a given random roll.
    ///
    /// The roll must be within `1..=100`.
    ///
    /// Event selection follows this logic:
    ///
    /// - `roll <= jackpot` → `Jackpot`
    /// - `roll >= 100 - luck_break` → `LuckBreak`
    /// - otherwise → `Normal`
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
