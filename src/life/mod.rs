pub mod areas;
pub mod objects;
pub mod tasks;
pub mod world;
pub mod wusel;

const MINUTE: u32 = 2; // ticks
const HOUR: u32 = 60 * MINUTE; // ticks
const DAY: u32 = 24 * HOUR; // ticks
const WEEK: u32 = 7 * DAY; // ticks
