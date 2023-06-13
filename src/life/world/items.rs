//! # Items in the World
//!
//! TODO (2023-06-13) compare against objects.rs
//!
//! ## Author
//! Ngoc (Nox) Le <noxsense@gmail.com>

use crate::life::wusels;

/// A Blueprint is a list of required abilities, consumables or positions
///
/// to create a certain product after a certain time.
/// Blueprint: [ components, Workstation ] + Time => Product.
#[derive(Clone, PartialEq, Hash, Eq)]
#[allow(dead_code)]
struct Blueprint {
    id: usize,
    product: usize,
    components: Vec<usize>, // needed components: such as tools (desk) or ingredients (pen, paper).
    steps: usize,           // needed steps.
}

/// Something a Wusel can consume
///
/// Consumption / Usage will 'destroy' this object.
/// Consuming it might modify the needs and skills.
#[derive(Clone, Debug)]
pub struct Consumable {
    name: String,

    // Size representation: whole = 100% = size/size.
    size: u32, // a size representation: consuming this [size]  times, the thing is gone. (fixed)
    available: f32, // 1.0f whole, 0.0f gone. (temporary)

    // Sometimes, a consumable can spoil (> 0)
    spoils_after: u32, // spoils after 0: infinite, or N days. (fixed)
    age: u32,          // the current age of the consumable (temporary)

    // While consuming it, one part (1/size) while change the needs as following.
    need_change: std::collections::HashMap<wusels::needs::Need, i16>,
}

/// Identifier for a Construction
pub type ConstructionId = usize;

/// Type and type attributes of a Construction.
#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub enum ConstructionType {
    Wall(bool, usize), // is_horizontal (grows left->right, otherwise up->down), length
    Door(bool),        // is_open
    Window,
    Stairs(bool), // is_leading_up
    Floor,
}

pub const WALL_LR: bool = true; // is horizontal
pub const WALL_UD: bool = false; // is not horizontal
                                 //
pub const DOOR_OPEN: bool = true;
pub const DOOR_CLOSED: bool = false;

/// A Construction is an enviromental part of the world.
///
/// They offer only just few options to interact with.
/// Mostly they block ways and are there to build and present place for the world.
#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub struct Construction {
    id: ConstructionId,
    construction_type: ConstructionType, // TODO better type.
}

impl Construction {
    pub fn new(id: ConstructionId, construction_type: ConstructionType) -> Self {
        Self {
            id,
            construction_type,
        }
    }

    pub fn id(&self) -> ConstructionId {
        self.id
    }

    pub fn construction_type(&self) -> ConstructionType {
        self.construction_type
    }
}
