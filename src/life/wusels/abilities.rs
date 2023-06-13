/// An ability, the Wusel can learn to improve their lifestyle.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Ability {
    COOKING,
    COMMUNICATION,
    FITNESS,
    FINESSE,
}

impl Ability {
    pub const VALUES: [Self; 4] = [
        Self::COOKING,
        Self::COMMUNICATION,
        Self::FITNESS,
        Self::FINESSE,
    ];

    pub fn get_name(&self) -> &str {
        match self {
            Self::COOKING => "cooking",
            Self::COMMUNICATION => "communication",
            Self::FITNESS => "fitness",
            Self::FINESSE => "finesse",
        }
    }
}
