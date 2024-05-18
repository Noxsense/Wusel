use crate::model::creature::*;

/// World.
/// View of the World and life.
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct World {
    // TODO: placeholder.
    /// how many simulated time units are played withim one real time unit. (normal: 1)
    time: u64,
    wusels: Vec<Wusel>,
}

/// Position.
/// A point in the world with three coordinates.
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct Position {
    /// width (bird's eye: horizontal axis)
    x: u64,

    /// depth (bird's eye: vertical axis)
    y: u64,

    /// height (bird's eye: upper may cover below, up to certsin level)
    z: u64,
}

////////////////////////////////////////////////////////////////////////////////

impl Position {
    pub fn from(x: u64, y: u64, z: u64) -> Self {
        Self { x, y, z }
    }
}

impl World {
    pub fn new() -> Self {
        Self {
            time: 0,
            wusels: vec![],
        }
    }

    pub fn from(time: u64, wusels: Vec<Wusel>) -> Self {
        Self { time, wusels }
    }

    pub fn with_time(&self, time: u64) -> Self {
        let wusels = vec![];
        Self { time, wusels }
    }

    pub fn time(&self) -> u64 {
        self.time
    }

    pub fn wusels_iter(&self) -> std::slice::Iter<Wusel> {
        self.wusels.iter()
    }

    pub fn wusel_by_id(&self, wusel_id: WuselId) -> Option<Wusel> {
        self.wusels
            .clone()
            .into_iter()
            .find(|wusel| wusel.id() == wusel_id)
    }
}
