use crate::model::world::Position;

/// Wusel.
/// A living object, "that discovers the wolrd."
/// A bundle of needs position in the world.
/// For Living it has an own drive to survive, explore and sozialize.
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct Wusel {
    /// Identificator of one Wusel, should be unique.
    id: WuselId,

    /// Position of the Wusel.
    position: Position,

    /// Age of a Wusel.
    /// when it is born, the age starts, if the wusel is too old, it dies.
    age: u64,

    /// Life Need for Sleep; if too low, the Wusel falls asleep and must rest.
    /// while sleepy, a wusel is also slower.
    wakefulness: u64,

    /// Life Need for Food and Water; if too low, the Wusel starves and dies.
    /// while hungry, the wusel is not very focussed, or distractable.
    nourishment: u64,

    /// After taking Food and Water, the digestion starts (progress).
    /// if the digestions is done, the Wusel needs to get rid of digested materials.
    digestion: u64,

    /// Level of Tidiness.
    /// If The Wusel gets too dirty, the immune system is weakened,
    /// and it may feel uncomfortable.
    tidiness: u64,
}

/// Simple Id to have a unique Wusel.
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct WuselId {
    value: usize, // TODO uuid on the long run.
}

////////////////////////////////////////////////////////////////////////////////

impl WuselId {
    /// generate a new wusel-id
    pub fn generate() -> Self {
        Self {
            value: rand::random::<usize>(),
        }
    }

    pub fn from(value: usize) -> Self {
        Self { value }
    }
}

impl Wusel {
    pub fn new(id: WuselId) -> Self {
        Self {
            // with given wusel id.
            id,

            // root position for new Wusel.
            position: Position::from(0, 0, 0),

            // just born.
            age: 0,

            // not about to fall asleep.
            wakefulness: 10,

            // not about to starve to dead.
            nourishment: 10,

            // digesting nothing.
            digestion: 0,

            // not that dirty.
            tidiness: 10,
        }
    }

    pub fn id(&self) -> WuselId {
        self.id
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn age(&self) -> u64 {
        self.age
    }

    pub fn set_age(&mut self, age: u64) {
        self.age = age;
    }

    pub fn wakefulness(&self) -> u64 {
        self.wakefulness
    }

    pub fn set_wakefulness(&mut self, wakefulness: u64) {
        self.wakefulness = wakefulness;
    }

    pub fn nourishment(&self) -> u64 {
        self.nourishment
    }

    pub fn set_nourishment(&mut self, nourishment: u64) {
        self.nourishment = nourishment;
    }

    pub fn digestion(&self) -> u64 {
        self.digestion
    }

    pub fn set_digestion(&mut self, digestion: u64) {
        self.digestion = digestion;
    }

    pub fn tidiness(&self) -> u64 {
        self.tidiness
    }

    pub fn set_tidiness(&mut self, tidiness: u64) {
        self.tidiness = tidiness;
    }
}
