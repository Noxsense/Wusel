/** Types of an object. */
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObjectType {
    Construction,
    Furniture,
    Miscellaneous,
    Food,
}

pub type ObjectWithSubType
    = (ObjectType, ObjectSubtype);

/** Identifier type (tuple) for an object. */
pub type ObjectId
    = (ObjectType, ObjectSubtype, usize);

pub type ObjectSubtype
    = &'static str; // String doesn't support Copy Trait, what is used for the TaskTag.

/** A world object indicates an object in the world which is not a wusel. */
#[derive(Debug, Clone)]
pub struct Object {
    name: String,
    object_id: ObjectId,
    transportable: bool, // can be transported by a wusel, will also apply stotable
    passable: bool,      // if true, wusel can walk over it's occupied place (if at position)
    consumable: Option<usize>, // if None: cannot be consumed; if Some(bites): number of parts, that can be consumed
    storage_capacity: usize,   // items that can be stored 0
}

impl Object {
    // /* Which object's counter to increase. */
    // let new_obj_count: usize = match obj_type {
    //     objects::ObjectType::Construction => {
    //         // TODO (2021-01-21) ... construction such as walls.
    //         // self.obj_count_furniture += 1;
    //         // self.obj_count_furniture // increase and return.
    //         1
    //     }
    //     objects::ObjectType::Furniture => {
    //         self.obj_count_furniture += 1;
    //         self.obj_count_furniture // increase and return.
    //     }
    //     objects::ObjectType::Food => {
    //         self.obj_count_food += 1;
    //         self.obj_count_food // increase and return.
    //     }
    //     objects::ObjectType::Miscellaneous => {
    //         self.obj_count_misc += 1;
    //         self.obj_count_misc // increase and return.
    //     }
    // };

    fn get_items_created() -> usize {
        0
    }

    pub fn new(
        name: String,
        obj_type: ObjectType,
        subtype: ObjectSubtype,
        transportable: bool,
        passable: bool,
        consumable_parts: Option<usize>,
        storage_capacity: usize,
    ) -> Self {
        Self {
            name,
            object_id: (obj_type, subtype, Self::get_items_created()),
            transportable,
            passable,
            consumable: consumable_parts,
            storage_capacity,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_object_id(&self) -> ObjectId {
        self.object_id
    }

    pub fn is_transportable(&self) -> bool {
        self.transportable
    }

    pub fn set_transportable(&mut self, transportable: bool) {
        self.transportable = transportable;
    }

    pub fn is_passable(&self) -> bool {
        self.passable
    }

    pub fn set_passable(&mut self, passable: bool) {
        self.passable = passable;
    }

    pub fn get_consumable(&self) -> Option<usize> {
        self.consumable
    }

    pub fn set_consumable(&mut self, consumable: Option<usize>) {
        self.consumable = consumable;
    }

    pub fn get_storage_capacity(&self) -> usize {
        self.storage_capacity
    }

    pub fn set_storage_capacity(&mut self, storage_capacity: usize) {
        self.storage_capacity = storage_capacity;
    }
}
