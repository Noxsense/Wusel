//! # World
//!
//! This module contains actual all game world and life logics and mechanics.
//!
//! ## Author
//! Ngoc (Nox) Le <noxsense@gmail.com>

use crate::life::objects;
use crate::life::wusels;
use crate::life::wusels::tasks;

use rand;

pub mod areas;
pub mod items;

// engine.
mod task_manager;
mod task_test;
mod unit_tests;

// TODO (2021-11-25) refactor the way something is stored in the world.
// TODO (2021-11-25) refactor how to peek into the world.
// TODO (2021-11-27) handler: life to life manager, positional things by world.
//
//
// TODO (2023-06-13) world.dimensions() => world.area() => world.positions()
// TODO (2023-06-13) world.time() : usize / current time.
// TODO (2023-06-13) world.wusels() // wusel_ids
// TODO (2023-06-13) world.wusel_new() // wusel_ids
// TODO (2023-06-13) world.wusel_set(id, ...) // update data
// TODO (2023-06-13) world.wusel_get(id) // copy of wusel data to view.
// TODO (2023-06-13) world.interactive_items() // objects for wusels. (food, doors, ...)
// TODO (2023-06-13) world.noninteractive_items() // Contruction walls, stairs (also doors)
// TODO (2023-06-13) world.items_set(id) // update item.
// TODO (2023-06-13) world.items_get(id) // data.

///  The place of existence, time and relations.
pub struct World {
    width: u32,
    depth: u32,
    height: u32, // in leveln.

    area: areas::Area,
    position_upper_bound: usize,
    positions: Vec<Vec<PlaceTaker>>,

    clock: usize, // time of the world.

    sequential_wusel_id: wusels::WuselId,

    // all currently living wusel in map.
    wusels: Vec<wusels::Wusel>,
    wusels_index_with_id: Vec<wusels::WuselId>,
    wusels_index_on_position_index: Vec<usize>,

    sequential_object_id: objects::ObjectId,

    // all current object instances in world.
    objects: Vec<objects::Object>,
    objects_index_with_id: Vec<objects::ObjectId>,
    objects_index_with_type: Vec<objects::ObjectType>,
    objects_index_with_whereabouts: Vec<InWorld>,

    // all constructions
    constructions: Vec<items::Construction>,
    constructions_index_on_position_index: Vec<usize>,

    // actions in this world.
    actions: Vec<String>,                      // actions to do.
    actions_effects: Vec<tasks::ActionAffect>, // how various actions on various objects may influence

    // more world information ...
    #[allow(dead_code)]
    dead_wusels: Vec<wusels::Wusel>,

    #[allow(dead_code)]
    relations:
        std::collections::BTreeMap<(wusels::WuselId, wusels::WuselId), wusels::relations::Relation>, // vector of wusel relations
}

/// State (in a sum type) with Positional Data for the world.
#[derive(Clone, Copy, PartialEq, Hash, Eq)]
enum InWorld {
    OnPositionIndex(usize),
    #[allow(dead_code)]
    InStorageId(objects::ObjectId),
    HeldByWuselId(wusels::WuselId),
    Nowhere,
}

/// A type wrapped identifier that represents something in the world.
#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub enum PlaceTaker {
    Construction(items::ConstructionType, items::ConstructionId),
    Wusel(wusels::WuselId),
    Object(objects::ObjectId, objects::ObjectType),
}

// TODO split up engine like, updater, getter, etc.
impl World {
    /// Create a new world.
    pub fn new(width: u32, depth: u32) -> Self {
        let height = 1;
        let position_upper_bound: usize = (width * depth * height) as usize;
        Self {
            width,
            depth,
            height,

            area: areas::Area::new(areas::Position::ROOT, width, depth, height),
            position_upper_bound,
            positions: vec![vec![]; position_upper_bound],

            clock: 0,

            sequential_wusel_id: 0,

            wusels: vec![],
            wusels_index_with_id: vec![],
            wusels_index_on_position_index: vec![],

            sequential_object_id: 0,

            objects: vec![],
            objects_index_with_id: vec![],
            objects_index_with_type: vec![],
            objects_index_with_whereabouts: vec![],

            constructions: vec![],
            constructions_index_on_position_index: vec![],

            dead_wusels: vec![],
            relations: std::collections::BTreeMap::new(),

            actions: vec![],
            actions_effects: vec![],
        }
    }

    /// Get the world's current time.
    pub fn get_time(&self) -> usize {
        self.clock
    }

    /// Increase clock and proceed decay of all things and relations.
    pub fn tick(&mut self) {
        self.clock += 1;

        // A new day is over: Forward the day structure to the world.
        let new_day: bool = self.clock % Self::TICKS_PER_DAY == 0;

        let mut some_busy_wusel: Vec<wusels::WuselId> = vec![];
        let mut new_babies: Vec<(
            wusels::WuselId,
            Option<wusels::WuselId>,
            wusels::WuselGender,
        )> = vec![];
        let mut dying_wusels: Vec<wusels::WuselId> = vec![];

        // Decay on every object and living.
        for (i, wusel) in self.wusels.iter_mut().enumerate() {
            // Watch all tasks, remove tasks, which may be aborted or ran out.
            wusel.auto_clean_tasks();

            // Peek into the ongoing task, and maybe proceed them.
            // This may lead to remove the done task.
            if !wusel.has_tasklist_empty() {
                some_busy_wusel.push(i);
            } else {
                // Wusel is currently not busy. => maybe apply an idle/auto task.
            }

            // If pregnant: Maybe push out the child => Failure, Early or too late.
            if wusel.is_pregnant() {
                let other_parent: Option<wusels::WuselId> = wusel.get_other_parent();
                let pregnancy_days: Option<u8> = wusel.get_remaining_pregnancy_days();
                let maybe_now: u8 = rand::random::<u8>() % 100;
                let possibility: u8 = match pregnancy_days {
                    Some(0) => 90,
                    Some(1) => 75,
                    _ => 10,
                };
                if (0u8..possibility).contains(&maybe_now) {
                    log::debug!("Pop the baby!");
                    let gender = wusels::WuselGender::random();
                    new_babies.push((wusel.get_id(), other_parent, gender));
                    // end pregnancy.
                    wusel.set_pregnancy(None, None);
                }
            }

            let alive = wusel.wusel_tick(new_day);

            // The wusel just died. Remove if from active wusels later.
            if !alive {
                dying_wusels.push(i);
            }
        }

        // Execute ongoing tasks, unmutable wusel context..
        for w in some_busy_wusel.iter() {
            if let Some(t) = self.wusels[*w].peek_ongoing_task() {
                // Decide how to progress the command.
                let u = (*t).clone();
                task_manager::proceed(self, u);
            }
        }

        for _ in self.relations.iter() {
            // Decay of relations over time.
        }

        // Command further name giving and attention from the player.
        for baby in new_babies.iter() {
            log::debug!(
                "New parents {}  and {} ({})",
                baby.0,
                baby.1.unwrap_or(usize::MAX),
                baby.2.to_char(),
            );
            // put babies to the wusel set.
        }
    }

    // const WORLD_INVENTORY: Where = InWorld::InStorageId((objects::ObjectType::Miscellaneous, "World-Storage", 0));

    pub const TICKS_PER_DAY: usize = 2880; // 24h by 0.5 minutes

    /// Get width of the world.
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Get depth of the world.
    pub fn get_depth(&self) -> u32 {
        self.depth
    }

    /// Get height of the world.
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Get full dimension (width, depth, height|levels) of the world.
    pub fn get_dimensions(&self) -> (u32, u32, u32) {
        (self.width, self.depth, self.height)
    }

    /// Get spanned area of the world.
    pub fn get_area(&self) -> areas::Area {
        self.area
    }

    /// Get the `positions` index for the requesting position (width, height).
    /// If the position is not in world, this index is not in [0, positions.len()).
    fn position_to_index(&self, position: areas::Position) -> usize {
        (position.x + self.width * position.y) as usize
    }

    /// Get the position tuple from the given index in this world.
    fn position_from_index(&self, position_index: usize) -> Option<areas::Position> {
        if position_index < self.position_upper_bound {
            Some(areas::Position {
                x: position_index as u32 % self.width,
                y: position_index as u32 / self.width,
                z: 0,
            })
        } else {
            None
        }
    }

    /// Get a random position in this world.
    pub fn position_random(&self) -> areas::Position {
        self.area.position_random()
    }

    /// Get the (valid) neighbours for a position.
    pub fn position_get_all_neighbours(&self, position: areas::Position) -> Vec<areas::Position> {
        self.area.get_all_neighbours_xy(position)
    }

    /// Check if the position is inside the world bounds.
    pub fn has_position(&self, position: areas::Position) -> bool {
        self.area.contains_position(&position)
    }

    /// Get the distance between two positions represented by indices in this world.
    #[allow(dead_code)]
    fn positions_indices_distance(&self, a_index: usize, b_index: usize) -> f32 {
        let a = self.position_from_index(a_index);
        let b = self.position_from_index(b_index);

        if a.is_none() || b.is_none() {
            return f32::INFINITY;
        }

        let a = a.unwrap();
        let b = b.unwrap();

        a.distance_to(&b)
    }

    /// Get all positions with a list/vector of the place takers on this position.
    /// Changing them will not influence the actual world state.
    pub fn positions_for_all_placetakers(&self) -> Vec<Vec<PlaceTaker>> {
        self.positions.clone()
    }

    /// Recalculate all the positions.
    /// On each position (given by the index), make a list / vector of all PlaceTaker
    /// which are on them.
    pub fn recalculate_positions_for_all_placetakers(&mut self) {
        // clear old positions.
        for placetakers in self.positions.iter_mut() {
            placetakers.clear();
        }

        // for constructions
        for (construction_index, &construction) in self.constructions.iter().enumerate() {
            let constructions_position_index =
                self.constructions_index_on_position_index[construction_index];

            let placetaker =
                PlaceTaker::Construction(construction.construction_type(), construction.id());

            // add all positions.
            self.positions[constructions_position_index].push(placetaker);

            if let items::ConstructionType::Wall(horizontal, length) =
                construction.construction_type()
            {
                let mut more_position = constructions_position_index;
                // first position is already put.
                for _i in 1..length {
                    more_position += if horizontal { 1 } else { self.width as usize };
                    self.positions[more_position].push(placetaker);
                }
            }
        }

        for (wusel_index, &wusel_position_index) in
            self.wusels_index_on_position_index.iter().enumerate()
        {
            self.positions[wusel_position_index]
                .push(PlaceTaker::Wusel(self.wusels_index_with_id[wusel_index]));
        }

        for (object_index, object_whereabouts) in
            self.objects_index_with_whereabouts.iter().enumerate()
        {
            if let InWorld::OnPositionIndex(object_position_index) = *object_whereabouts {
                let object_type = self.objects_index_with_type[object_index];
                let object_id = self.objects_index_with_id[object_index];
                self.positions[object_position_index]
                    .push(PlaceTaker::Object(object_id, object_type));
            }
        }
    }

    /// Update the positions.
    ///
    /// Remove the given place taker from the old position and put them onto the new position.
    /// This can also be used for the first position or for removing from the world.
    /// (With no valid old or new position.)
    fn update_positions(
        &mut self,
        placetaker: PlaceTaker,
        old_position_index: usize,
        new_position_index: usize,
    ) {
        // not if both position indices are invalid / higher / "not given", it just does nothing.
        // this also can remove a place taker from the map, or put it there on the first place.

        // remove from old position if given.
        if old_position_index < self.position_upper_bound {
            let opt_placetaker_index: Option<usize> = self.positions[old_position_index]
                .iter()
                .position(|&p| p == placetaker);

            if let Some(placetaker_index) = opt_placetaker_index {
                self.positions[old_position_index].remove(placetaker_index);
            }
        }

        // put on new position if given.
        if new_position_index < self.position_upper_bound {
            self.positions[new_position_index].push(placetaker);
        }
    }

    /// Create a new Construction within the world.
    pub fn construction_new(
        &mut self,
        construction_type: items::ConstructionType,
        position: areas::Position,
    ) {
        let construction = items::Construction::new(0usize, construction_type);

        let position_index = self.position_to_index(position);

        let placetaker = PlaceTaker::Construction(construction_type, construction.id());

        self.constructions.push(construction);

        // start position.
        self.constructions_index_on_position_index
            .push(position_index);

        // all positions it may take.
        self.update_positions(placetaker, 0, position_index);
        if let items::ConstructionType::Wall(horizontal, length) = construction_type {
            let mut more_position = position_index;
            // first position is already put.
            for _i in 1..length {
                more_position += if horizontal { 1 } else { self.width as usize };
                self.update_positions(placetaker, 0, more_position);
            }
        }
    }

    /// Get all construction inidces of a door.
    fn get_all_doors_indices(&self) -> Vec<usize> {
        let mut doors = vec![];
        for (index, construction) in self.constructions.iter().enumerate() {
            if let items::ConstructionType::Door(_is_open) = construction.construction_type() {
                doors.push(index);
            }
        }
        doors
    }

    /// Create a new object to exist in this world.
    ///
    /// Placed in a world inventory/storage first, can be placed in world.
    /// Returns the new object's index for the world's objects.
    pub fn object_new(
        &mut self,
        object_type: objects::ObjectType,
        name: String,
        transportable: bool,
        passable: bool,
        consumable_parts: u16,
        storage_capacity: u16,
    ) -> objects::ObjectId {
        // Add the new object into the world active objects.
        self.objects.push(objects::Object::new(
            name,
            object_type,
            passable,
            false, // stackable
            transportable,
            consumable_parts,
            storage_capacity,
        ));

        let object_id: objects::ObjectId = self.sequential_object_id;

        self.objects_index_with_whereabouts.push(InWorld::Nowhere);
        self.objects_index_with_id.push(object_id);
        self.objects_index_with_type.push(object_type);

        log::info!("New object created: {:?}", self.objects.last_mut());

        self.sequential_object_id += 1;

        // Return new ID and appended index.
        object_id
    }

    /// Create a new food (an object) to exist in this world.
    ///
    /// This calls `self.object_new(Food, name, true, 0)`.
    /// => Food is transportable, no storage.
    ///
    /// Placed in a world inventory/storage first, can be placed in world.
    /// Returns the new object's index for the world's objects.
    pub fn food_new(&mut self, name: objects::ObjectSubtype, bites: u16) -> objects::ObjectId {
        self.object_new(
            objects::ObjectType::Food(name),
            name.to_string(),
            true,
            true,
            bites,
            0,
        )
    }

    /// Duplicate a world object: Use all attributes, but change the ID.
    ///
    /// This will create a new object, currently in world's storage.
    pub fn object_duplicate(&mut self, base_index: usize) -> Option<objects::ObjectId> {
        // Duplicate non existing?.
        if base_index >= self.objects.len() {
            return None;
        }

        let fresh_object = objects::Object::clone_as_new(&(self.objects[base_index]));
        let fresh_object_id = fresh_object.get_object_id();
        let fresh_object_type = fresh_object.get_object_type();

        self.objects.push(fresh_object);
        self.objects_index_with_whereabouts.push(InWorld::Nowhere);
        self.objects_index_with_id.push(fresh_object_id);
        self.objects_index_with_type.push(fresh_object_type);

        self.sequential_object_id += 1;

        Some(fresh_object_id)
    }

    fn get_objects_index_by_id(&self, object_id: objects::ObjectId) -> Option<usize> {
        self.objects_index_with_id
            .iter()
            .position(|id| *id == object_id)
    }

    fn get_object_whereabouts_by_id(&self, object_id: objects::ObjectId) -> Option<&InWorld> {
        if let Some(object_index) = self.get_objects_index_by_id(object_id) {
            self.objects_index_with_whereabouts.get(object_index)
        } else {
            None
        }
    }

    fn get_object_type_by_id(&self, object_id: objects::ObjectId) -> Option<objects::ObjectType> {
        if let Some(object_index) = self.get_objects_index_by_id(object_id) {
            self.objects_index_with_type.get(object_index).copied()
        } else {
            None
        }
    }

    /// Find the optional index of an object, given by an ID.
    fn object_id_to_index(&self, object_id: objects::ObjectId) -> Option<usize> {
        self.objects
            .iter()
            .position(|o| o.get_object_id() == object_id)
    }

    /// Get the optional position of an object, given by an index.
    /// If the position is held by a storage, get the position of the storage.
    fn objects_index_get_position(&self, object_index: usize) -> Option<areas::Position> {
        match self.objects_index_with_whereabouts.get(object_index) {
            Some(InWorld::OnPositionIndex(position_index)) => {
                self.position_from_index(*position_index)
            }
            Some(InWorld::HeldByWuselId(wusel_id)) => {
                // get nested position of holder.
                self.get_wusels_index_by_id(*wusel_id)
                    .map(|holder_index| self.wusels_index_on_position_index[holder_index])
                    .map(|wusel_position_index| self.position_from_index(wusel_position_index))
                    .map(|opt_opt_position| opt_opt_position.unwrap())
            }
            Some(InWorld::InStorageId(storage_object_id)) => {
                // get nested position (of storage).
                self.object_get_position(*storage_object_id)
            }
            _ => None,
        }
    }

    /// Get the optional position of an object, given by an ID.
    /// If the position is held by a storage, get the position of the storage.
    pub fn object_get_position(&self, object_id: objects::ObjectId) -> Option<areas::Position> {
        if let Some(object_index) = self.object_id_to_index(object_id) {
            self.objects_index_get_position(object_index)
        } else {
            None
        }
    }

    /// Get the positions of all InWorld::OnPositionIndex objects.
    #[allow(dead_code)]
    pub fn positions_for_objects(&self) -> Vec<areas::Position> {
        // unique positions.
        self.objects_index_with_whereabouts
            .iter()
            .filter(|whereabout| matches!(whereabout, InWorld::OnPositionIndex(_)))
            .map(|on_position_index| {
                if let InWorld::OnPositionIndex(position_index) = on_position_index {
                    self.position_from_index(*position_index)
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }

    /// Place an object on a new position (in world).
    /// If the object was held or stored before, it is now not anymore.
    pub fn object_set_position(&mut self, object_id: objects::ObjectId, position: areas::Position) {
        if let Some(object_index) = self.object_id_to_index(object_id) {
            let object_type = *self.objects_index_with_type.get(object_index).unwrap();

            let placetaker = PlaceTaker::Object(object_id, object_type);

            let old_position_index = match self.objects_index_with_whereabouts[object_index] {
                InWorld::OnPositionIndex(old_position_index) => old_position_index,
                _ => self.position_upper_bound, // none (out of world).
            };
            let new_position_index = self.position_to_index(position);

            self.object_set_whereabouts(object_index, InWorld::OnPositionIndex(new_position_index));

            self.update_positions(placetaker, old_position_index, new_position_index);
        }
    }

    /// Place an object on a new position, or store it within an inventory, or let it held by a wusel.
    /// The object is given by an (vector) index of all currently active objects.
    /// If the object is removed from a world position, this will remove the object from the old position.
    fn object_set_whereabouts(&mut self, object_index: usize, whereto: InWorld) {
        // Invalid index. => Abort.
        if object_index >= self.objects.len() {
            return;
        }

        // just update.
        self.objects_index_with_whereabouts[object_index] = whereto;
    }

    /// Destroy an object given by a certain all-active-object's index.
    fn object_destroy(&mut self, object_index: usize) {
        if object_index >= self.objects.len() {
            return;
        }

        self.objects.remove(object_index);
        self.objects_index_with_whereabouts.remove(object_index);
        self.objects_index_with_id.remove(object_index);
    }

    /// Add a wusel to the world.
    ///
    ///ID is the current wusel count.
    // TODO (2020-11-20) what is about dead wusels and decreasing length?
    pub fn wusel_new(
        &mut self,
        name: String,
        gender: wusels::WuselGender,
        position: areas::Position,
    ) {
        let new_wusel_id = self.sequential_wusel_id; // almost id (for a long time unique)
        let new_wusel = wusels::Wusel::new(new_wusel_id, name, gender); // new wusel at (position)

        // Add wusel to positions, start at (position).
        let position_index = self.position_to_index(position);

        // XXX put new wusel on position.
        self.wusels.push(new_wusel);
        self.wusels_index_with_id.push(new_wusel_id); // fast access id.
        self.wusels_index_on_position_index.push(position_index); // access position.

        // self.wusels_positions.push(position_index); // index.
        self.sequential_wusel_id += 1;
    }

    /// Create a new random wusel.
    pub fn wusel_new_random(&mut self, wusel_name: String) {
        let wusel_gender = wusels::WuselGender::random();
        let wusel_position = self.position_random();
        self.wusel_new(wusel_name, wusel_gender, wusel_position);
    }

    /// Count how many wusels are currently active.
    pub fn wusel_count(&self) -> usize {
        self.wusels.len()
    }

    /// Check if a wusel index is actually given within the world::wusels.
    fn check_valid_wusel_index(&self, wusel_index: usize) -> bool {
        wusel_index < self.wusels.len()
    }

    /// Return the wusel index that holds the wusle with the requesting identifier.
    fn get_wusels_index_by_id(&self, wusel_id: wusels::WuselId) -> Option<usize> {
        self.wusels_index_with_id
            .iter()
            .position(|id| *id == wusel_id)
    }

    /// Return an index for positions that is held by the wusel given by their identifier.
    fn get_wusel_position_index_by_id(&self, wusel_id: wusels::WuselId) -> Option<&usize> {
        if let Some(wusel_index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels_index_on_position_index.get(wusel_index)
        } else {
            None
        }
    }

    /// Get an optional Position for the wusel given by their identifier.
    pub fn wusel_get_position(&self, wusel_id: wusels::WuselId) -> Option<areas::Position> {
        self.get_wusel_position_index_by_id(wusel_id)
            .map(|&position_index| self.position_from_index(position_index))
            .map(|opt_position| opt_position.unwrap())
    }

    /// Set the position of the indexed wusel to the nearest valid position
    /// If the position may land out of the grid, put it to the nearest border.
    pub fn wusel_set_position(&mut self, wusel_id: wusels::WuselId, position: areas::Position) {
        if let Some(&wusel_index) = self.get_wusel_position_index_by_id(wusel_id) {
            self.wusel_set_position_by_index(wusel_index, position);
        }
    }

    /// Set the position of the indexed wusel to the nearest valid position
    /// If the position may land out of the grid, put it to the nearest border.
    fn wusel_set_position_by_index(&mut self, wusel_index: usize, position: areas::Position) {
        if self.check_valid_wusel_index(wusel_index) {
            let placetaker = PlaceTaker::Wusel(self.wusels_index_with_id[wusel_index]);
            let old_position_index = self.wusels_index_on_position_index[wusel_index];
            let new_position_index = self.position_to_index(position);

            self.wusels_index_on_position_index[wusel_index] = new_position_index;

            self.update_positions(placetaker, old_position_index, new_position_index);
        }
    }

    /// Get the positions of all active wusels.
    #[allow(dead_code)]
    pub fn positions_for_wusels(&self) -> Vec<areas::Position> {
        // unique positions.
        self.wusels_index_on_position_index
            .iter()
            .map(|&position_index| self.position_from_index(position_index))
            .flatten()
            .collect()
    }

    /// Get the indices of all wusels, which are alive.
    pub fn wusel_get_all_alive(&self) -> Vec<usize> {
        // TODO (2021-12-11) why as indices, this could be leaked or obsolete on later steps.

        let mut alive: Vec<usize> = vec![];
        for i in 0..self.wusels.len() {
            if self.wusels[i].is_alive() {
                alive.push(i);
            }
        }
        alive
    }

    /// Get the indices of all wusels, which are currently having no tasks to do.
    pub fn wusel_get_all_unbusy(&self) -> Vec<usize> {
        // TODO (2021-12-11) why as indices, this could be leaked or obsolete on later steps.

        let mut unbusy: Vec<usize> = vec![];
        for i in 0..self.wusels.len() {
            if self.wusels[i].has_tasklist_empty() {
                unbusy.push(i);
            }
        }
        unbusy
    }

    /// Check if the wusel of the world is alive.
    ///
    /// This wraps [wusel::Wusel::is_alive](wusel::Wusel::is_alive) for a world wusel.
    pub fn wusel_is_alive(&self, wusel_id: wusels::WuselId) -> Option<bool> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].is_alive())
    }

    /// Get the age of the wusel in days.
    ///
    /// This wraps [wusel::Wusel::get_lived_days](wusel::Wusel::get_lived_days) for a world wusel.
    pub fn wusel_get_lived_days(&self, wusel_id: wusels::WuselId) -> Option<u32> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].get_lived_days())
    }

    /// Set the life stage of the wusel. This also indirectly may override the age in days.
    ///
    /// This wraps [wusel::Wusel::set_life_state](wusel::Wusel::set_life_state) for a world wusel.
    pub fn wusel_set_life_state(&mut self, wusel_id: wusels::WuselId, life_state: wusels::Life) {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index].set_life_state(life_state);
        }
    }

    /// Get the name of the wusel.
    ///
    /// This wraps [wusel::Wusel::get_name](wusel::Wusel::get_name) for a world wusel.
    pub fn wusel_get_name(&self, wusel_id: wusels::WuselId) -> Option<String> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].get_name())
    }

    /// Set the name of a Wusel.
    ///
    /// This wraps [wusel::Wusel::set_name](wusel::Wusel::set_name) for a world wusel.
    pub fn wusel_set_name(&mut self, wusel_id: wusels::WuselId, new_name: String) {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index].set_name(new_name);
        }
    }

    /// Get the gender of the wusel.
    ///
    /// This wraps [wusel::Wusel::get_gender](wusel::Wusel::get_gender) for a world wusel.
    pub fn wusel_get_gender(&self, wusel_id: wusels::WuselId) -> Option<wusels::WuselGender> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].get_gender())
    }

    /// Set the gender of a Wusel.
    ///
    /// This wraps [wusel::Wusel::set_gender](wusel::Wusel::set_gender) for a world wusel.
    pub fn wusel_set_gender(&mut self, wusel_id: wusels::WuselId, new_gender: wusels::WuselGender) {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index].set_gender(new_gender);
        }
    }

    /// Get the requested need's level of the wusel.
    ///
    /// This wraps [wusel::Wusel::get_need](wusel::Wusel::get_need) for a world wusel.
    pub fn wusel_get_need(&mut self, wusel_id: wusels::WuselId, need: wusels::needs::Need) -> u32 {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].get_need(need))
            .unwrap_or(0u32)
    }

    /// Set the requesting need's level of the wusel.
    ///
    /// This wraps [wusel::Wusel::set_need](wusel::Wusel::set_need) for a world wusel.
    pub fn wusel_set_need(
        &mut self,
        wusel_id: wusels::WuselId,
        need: &wusels::needs::Need,
        new_value: u32,
    ) {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index].set_need(*need, new_value);
        }
    }

    /// Modify the requesting needs's level of the wusel by a given value.
    ///
    /// This wraps [wusel::Wusel::set_need_relative](wusel::Wusel::set_need_relative)
    /// for a world wusel.
    pub fn wusel_set_need_relative(
        &mut self,
        wusel_id: wusels::WuselId,
        need: &wusels::needs::Need,
        relative: i16,
    ) {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index].set_need_relative(*need, relative);
        }
    }

    /// Get the requesting ability's value of the wusel.
    ///
    /// This wraps [wusel::Wusel::get_ability](wusel::Wusel::get_ability) for a world wusel.
    pub fn wusel_get_ability(
        &self,
        wusel_id: wusels::WuselId,
        ability: wusels::abilities::Ability,
    ) -> Option<u32> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].get_ability(ability))
    }

    /// Set the requesting ability's value of the wusel.
    ///
    /// This wraps [wusel::Wusel::set_ability](wusel::Wusel::set_ability) for a world wusel.
    pub fn wusel_set_ability(
        &mut self,
        wusel_id: wusels::WuselId,
        ability: wusels::abilities::Ability,
        new_value: u32,
    ) {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index].set_ability(ability, new_value);
        }
    }

    /// Increase the requesting ability's value of the wusel.
    ///
    /// This wraps [wusel::Wusel::improve](wusel::Wusel::improve) for a world wusel.
    pub fn wusel_improve(
        &mut self,
        wusel_id: wusels::WuselId,
        ability: wusels::abilities::Ability,
    ) {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index].improve(ability);
        }
    }

    /// Check if the wusel's tasklist is empty.
    ///
    /// This wraps [wusel::Wusel::has_tasklist_empty](wusel::Wusel::has_tasklist_empty) for a world
    /// wusel.
    pub fn wusel_has_tasklist_empty(&self, wusel_id: wusels::WuselId) -> Option<bool> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].has_tasklist_empty())
    }

    /// Get the count of the wusel's tasklist.
    ///
    /// This wraps [wusel::Wusel::get_tasklist_len](wusel::Wusel::get_tasklist_len) for a world
    /// wusel.
    pub fn wusel_get_tasklist_len(&self, wusel_id: wusels::WuselId) -> Option<usize> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].get_tasklist_len())
    }

    /// Get the wusel's takslist as name representation.
    ///
    /// This wraps [wusel::Wusel::get_tasklist_names](wusel::Wusel::get_tasklist_names)
    /// for a world wusel.
    pub fn wusel_get_tasklist_names(&mut self, wusel_id: wusels::WuselId) -> Vec<String> {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index].get_tasklist_names()
        } else {
            vec![]
        }
    }

    /// Give an available wusel (by index) a new task.
    ///
    /// This wraps [wusel::Wusel::assign_to_task](wusel::Wusel::assign_to_task) for a world wusel.
    pub fn wusel_assign_to_task(&mut self, wusel_index: usize, taskb: tasks::TaskBuilder) {
        // TODO (2021-12-11) why given with wusel index?

        if let Some(wusel) = self.wusels.get_mut(wusel_index) {
            // Task apply wusel[index] as actor.
            wusel.assign_to_task(self.clock, taskb);
            log::debug!("task successfully assigned")
        }
    }

    /// Abort the wusel's task.
    ///
    /// This wraps [wusel::Wusel::abort_task](wusel::Wusel::abort_task) for a world wusel.
    pub fn wusel_abort_task(&mut self, wusel_id: wusels::WuselId, task_index: usize) {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index].abort_task(task_index);
        }
    }

    /// Get the task the wusel is currently doing.
    ///
    /// This can be none if the Wusel is doing nothing,
    /// or if that requesting wusel could not be found as well.
    ///
    /// This wraps [wusel::Wusel::peek_ongoing_task](wusel::Wusel::peek_ongoing_task)
    /// for a world wusel.
    pub fn wusel_peek_ongoing_task(&self, wusel_id: wusels::WuselId) -> Option<&tasks::Task> {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index].peek_ongoing_task()
        } else {
            None
        }
    }

    /// Check if the wusel is pregnant.
    ///
    /// This wraps [wusel::Wusel::is_pregnant](wusel::Wusel::is_pregnant) for a world wusel.
    pub fn wusel_is_pregnant(&self, wusel_id: wusels::WuselId) -> Option<bool> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].is_pregnant())
    }

    /// Set the wusel pregnant, with the optional other parent and optional remaining days.
    ///
    /// This wraps [wusel::Wusel::set_pregnancy](wusel::Wusel::set_pregnancy) for a world wusel.
    pub fn wusel_set_pregnancy(
        &mut self,
        wusel_id: wusels::WuselId,
        other_parent: Option<wusels::WuselId>,
        remaining_days: Option<u8>,
    ) {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index].set_pregnancy(other_parent, remaining_days);
        }
    }

    /// Get the other parent of the wusel's unborn child.
    ///
    /// This wraps [wusel::Wusel::get_other_parent](wusel::Wusel::get_other_parent)
    /// for a world wusel.
    pub fn wusel_get_other_parent(&self, wusel_id: wusels::WuselId) -> Option<wusels::WuselId> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].get_other_parent())
            .unwrap_or(None)
    }

    /// Get the remaining days of the wusel's pregnancy.
    ///
    /// This wraps [wusel::Wusel::get_remaining_pregnancy_days](wusel::Wusel::get_remaining_pregnancy_days) for a world wusel.
    pub fn wusel_get_remaining_pregnancy_days(&self, wusel_id: wusels::WuselId) -> Option<u8> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].get_remaining_pregnancy_days())
            .unwrap_or(None)
    }

    /// Show all relations for the wusel, given by index.
    /// Prints directly to std::out.
    pub fn wusel_show_relations(&self, wusel_index: usize) {
        // TODO (2021-12-11) refactor function declaration (not with index; this should not be on top out of the class.
        // TODO (2021-12-11) do not wusel_index on pub functions.
        // TODO (2021-12-11) refactor. function

        if wusel_index >= self.wusels.len() {
            println!("There is no wusel to show.");
            return;
        }

        let wusel_id = self.wusels[wusel_index].get_id();

        print!("Relations with {}: ", self.wusels[wusel_index].get_name());

        let mut has_relations: bool = false;

        for (who, relation) in self.relations.iter() {
            let other_id: wusels::WuselId;

            // Get the other wusel.
            // Skip where this wusel is even not part in the relation.
            if wusel_id == who.0 {
                other_id = who.1;
            } else if wusel_id == who.1 {
                other_id = who.0;
            } else {
                continue;
            } // not in relation

            let other_name = self.wusels[other_id].get_name();

            // Print Relation.
            print!("[{:?}: {relation}]", other_name);
            has_relations = true;
        }

        if !has_relations {
            print!("Has no relations.");
        }

        println!();
    }

    /// Update the relation of two wusels, given by their ID.
    pub fn wusel_update_relations(
        &mut self,
        wusel0_id: wusels::WuselId,
        wusel1_id: wusels::WuselId,
        nice: bool,
        relationtype: wusels::relations::RelationType,
    ) {
        // TODO (2021-12-11) refactor.

        // Decide for a relation key: (Greater ID, Smaller ID).

        let key = if wusel0_id <= wusel1_id {
            (wusel0_id, wusel1_id)
        } else {
            (wusel1_id, wusel0_id)
        };

        let change = if nice { 1 } else { -1 };

        // Get the relation if available.
        // update a key, guarding against the key possibly not being set.
        let rel = self
            .relations
            .entry(key)
            .or_insert_with(wusels::relations::Relation::new);

        rel.update(relationtype, change);
    }
}
