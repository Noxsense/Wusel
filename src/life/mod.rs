//! # Life
//!
//! Life simulation with Wusels (the living parts), objects (passive parts)
//! positions and other directional information and task managers.

pub mod world;

pub mod wusels;

pub mod objects;

/// Default ticks per minute
const MINUTE: u32 = 2; // ticks

/// Default ticks per hour (60 minutes)
const HOUR: u32 = 60 * MINUTE; // ticks

/// Default ticks per day (24 hours)
const DAY: u32 = 24 * HOUR; // ticks

/// Default ticks per week (7 days)
const WEEK: u32 = 7 * DAY; // ticks
