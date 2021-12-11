//! # Objects
//!
//! An object is a non-living but intractable part of the world.
//! They can store other objects (or be put into the storages).
//! Also they can be consumed and used up or created, or just put into the world.
//!
//! ## Author
//! Ngoc (Nox) Le <noxsense@gmail.com>

/// Types of an object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectType {
    Furniture(ObjectSubtype),
    Miscellaneous(ObjectSubtype),
    Food(ObjectSubtype),
}

/// Subtype or Subcategory of an Object
pub type ObjectSubtype
    = &'static str; // String doesn't support Copy Trait, what is used for the TaskTag.

/// Identifier type (tuple) for an object.
pub type ObjectId
    = usize;

/// A world object indicates an object in the world which is not a wusel.
#[derive(Debug, Clone)]
pub struct Object {
    name: String,
    id: usize,
    object_type: ObjectType,
    object_attributes: u8,
    consumable_bites: u16,
    storage_capacity: u16,   // items that can be stored 0

    consumable_bites_left: u16,
    storage_capacity_left: u16,
}

impl Object {

    pub const OBJECT_IS_BLOCKING: u8  = 0b001; // not passable, will block the way.
    pub const OBJECT_IS_STACKABLE: u8 = 0b010; // can be under or on top of another object.
    pub const OBJECT_IS_PORTABLE: u8  = 0b100; // can be carried or stored.

    fn get_items_created() -> usize {
        0
    }

    pub fn new(
        name: String,
        object_type: ObjectType,
        is_solid: bool,
        is_stackable:bool,
        is_portable: bool,
        consumable_bites: u16,
        storage_capacity: u16,
    ) -> Self {
        Self {
            name,
            object_type,
            id: Self::get_items_created(),
            object_attributes: Self::to_object_attributes(is_solid, is_stackable, is_portable),
            consumable_bites,
            storage_capacity,
            consumable_bites_left: consumable_bites,
            storage_capacity_left: storage_capacity,
        }
    }

    /// Initiate new object (but as new) like the given. New Object: Not consumed and empty.
    pub fn clone_as_new(other: &Self) -> Self {
        Self {
            name: other.name.clone(),
            object_type: other.object_type,
            id: Self::get_items_created(), // new
            object_attributes: other.object_attributes,
            consumable_bites: other.consumable_bites,
            storage_capacity: other.storage_capacity,
            consumable_bites_left: other.consumable_bites,
            storage_capacity_left: other.storage_capacity,
        }
    }

    pub fn get_object_id(&self) -> usize {
        self.id
    }

    pub fn get_object_type(&self) -> ObjectType {
        self.object_type
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn to_object_attributes(
        is_blocking: bool,
        is_stackable:bool,
        is_portable: bool
    ) -> u8 {
        (if is_blocking { Self::OBJECT_IS_BLOCKING } else { 0u8 })
        + if is_stackable { Self::OBJECT_IS_STACKABLE } else { 0u8 }
        + if is_portable { Self::OBJECT_IS_PORTABLE } else { 0u8 }
    }

    pub fn is_blocking_stackable_portable(&self) -> (bool, bool, bool) {
        (self.is_blocking(), self.is_stackable(), self.is_portable())
    }

    pub fn is_blocking(&self) -> bool {
        (self.object_attributes & Self::OBJECT_IS_BLOCKING) != 0
    }

    pub fn is_stackable(&self) -> bool {
        (self.object_attributes & Self::OBJECT_IS_STACKABLE) != 0
    }

    pub fn is_portable(&self) -> bool {
        (self.object_attributes & Self::OBJECT_IS_PORTABLE) != 0
    }

    pub fn get_consumable(&self) -> u16 {
        self.consumable_bites
    }

    pub fn get_consumable_left(&self) -> u16 {
        self.consumable_bites_left
    }

    pub fn set_consumable_left(&mut self, consumable_bites_left: u16) {
        self.consumable_bites_left = consumable_bites_left;
    }

    pub fn get_storage_capacity(&self) -> u16 {
        self.storage_capacity
    }

    pub fn get_storage_capacity_left(&self) -> u16 {
        self.storage_capacity_left
    }

    pub fn set_storage_capacity_left(&mut self, storage_capacity_left: u16) {
        self.storage_capacity_left = storage_capacity_left;
    }
}
