//! A need, the Wusel needs to satisfy to survive.
//!

use crate::life;

/// A need, the Wusel needs to satisfy to survive.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq, PartialOrd)]
pub enum Need {
    WATER,
    FOOD,
    SLEEP,
    LOVE,
    FUN,
    WARMTH,
    HEALTH,
}

impl Need {
    pub const VALUES: [Self; 7] = [
        Self::WATER,
        Self::FOOD,
        Self::SLEEP,
        Self::LOVE,
        Self::FUN,
        Self::WARMTH,
        Self::HEALTH,
    ];

    pub fn get_name(&self) -> &str {
        match self {
            Self::WATER => "water",
            Self::FOOD => "food",
            Self::WARMTH => "warmth",
            Self::SLEEP => "sleep",
            Self::HEALTH => "health",
            Self::LOVE => "love",
            Self::FUN => "fun",
        }
    }

    pub fn get_default_decay(&self) -> u32 {
        match self {
            Self::WATER | Self::FOOD | Self::SLEEP | Self::LOVE | Self::FUN => 1,
            Self::WARMTH => 0, // by external sources
            Self::HEALTH => 0, // by external sources
        }
    }

    /// From full to 0:
    /// How many ticks does it need, when it's only normally decreasing.
    /// This is adapted to nromal human life.
    pub fn get_full(&self) -> u32 {
        match self {
            Need::WARMTH => life::HOUR * 8, // 8 hours until freeze to death.
            Need::WATER => life::DAY * 3,   // 3 days until dehydrate.
            Need::FOOD => life::WEEK,       // a week until starve.
            Need::SLEEP => life::WEEK,      // a week until suffer from sleep loss.
            Need::LOVE => life::WEEK * 2,   // 2 weeks until become lonely.
            Need::FUN => life::WEEK * 2,    // 2 weeks until unmotivated and depressive.
            Need::HEALTH => life::WEEK * 2, // 2 weeks until die of illness.
        }
    }

    /// From full to 0:
    /// How many ticks does it need, when it's only normally decreasing.
    /// This is adapted to nromal human life.
    pub fn is_fatal(&self) -> bool {
        match self {
            Self::WATER | Self::FOOD | Self::WARMTH | Self::HEALTH => true,
            Self::SLEEP | Self::LOVE | Self::FUN => false,
        }
    }
}
