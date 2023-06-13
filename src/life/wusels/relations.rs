/// Pair of Wusels which may have a relation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Relation {
    officially: String,    // officially known state (Friends, Spouse, etc..)
    friendship: i32,       // shared friendship between both.
    romance: i32,          // shared romance between both
    kindred_distance: i32, // blood relation (distance)
}

/// Relation Direction
///
/// Any Relation can be romantically, non-romantically, etc.
pub enum RelationType {
    Romance,
    Friendship,
}

impl RelationType {
    pub fn from_romantically(try_romance: bool) -> Self {
        if try_romance {
            Self::Romance
        } else {
            Self::Friendship
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Friendship => '\u{2639}', // smiley
            Self::Romance => '\u{2661}',    // heart
        }
    }
}

impl Default for Relation {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Relation {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "'{official}' {relation_friendly_char}{friendly} {relation_romance_char}{romance}{kinship}",
            official = self.officially,
            relation_friendly_char = RelationType::Friendship.to_char(),
            friendly = self.friendship,
            relation_romance_char = RelationType::Romance.to_char(),
            romance = self.romance,
            kinship = match self.kindred_distance {
                -1 => "",
                0 => " Self?",
                1 => " Siblings|Parents|Kids",
                _ => "Related",
            }
        )
    }
}

impl Relation {
    pub fn new() -> Self {
        Self {
            officially: String::from("Strangers"),
            friendship: 0,
            romance: 0,
            kindred_distance: -1,
        }
    }

    pub fn update(&mut self, relationtype: RelationType, change: i32) {
        match relationtype {
            RelationType::Friendship => self.update_friendship(change),
            RelationType::Romance => self.update_romance(change),
        }
    }

    pub fn update_romance(&mut self, change: i32) {
        self.romance += change;
    }

    pub fn update_friendship(&mut self, change: i32) {
        self.friendship += change;
    }
}
