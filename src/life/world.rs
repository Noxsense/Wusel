/** module Life.
 * - This module contains actuall all game world and life logics and mechanics.
 * @author Nox
 * @version 2021.0.1 */

use crate::life::areas;
use crate::life::tasks;
use crate::life::wusel;
use crate::life::objects;

use rand;

pub mod task_manager;
mod task_test;
mod unit_tests;

// TODO (2021-11-25) refactor the way something is stored in the world.
// TODO (2021-11-25) refactor how to peek into the world.
// TODO (2021-11-27) handler: life to life manager, positional things by world.

/** The place of existence, time and relations. */
pub struct World {
    width: u32,
    depth: u32,

    area: areas::Area,
    position_upper_bound: usize,
    positions: Vec<Vec<PlaceTaker>>,

    clock: usize, // time of the world.

    sequential_wusel_id: usize,

    // all currently living wusel in map.
    wusels: Vec<wusel::Wusel>,
    wusels_index_with_id: Vec<usize>,
    wusels_index_on_position_index: Vec<usize>,

    sequential_object_id: usize,

    // all current object instances in world.
    objects: Vec<objects::Object>,
    objects_index_with_id: Vec<objects::ObjectId>,
    objects_index_with_whereabouts: Vec<InWorld>,

    // actions in this world.
    actions: Vec<String>,                      // actions to do.
    actions_effects: Vec<tasks::ActionAffect>, // how various actions on various objects may influence

    // more world information ...

    dead_wusels: Vec<wusel::Wusel>,
    relations: std::collections::BTreeMap<(usize, usize), wusel::Relation>, // vector of wusel relations

}

impl World {

    /** Create a new world. */
    pub fn new(width: u32, depth: u32) -> Self {
        let position_upper_bound: usize = (width * 1 * depth) as usize;
        Self {
            width,
            depth,

            area: areas::Area::new(areas::Position::new(0, 0), width, depth),
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
            objects_index_with_whereabouts: vec![],

            dead_wusels: vec![],
            relations: std::collections::BTreeMap::new(),

            actions: vec![],
            actions_effects: vec![],
        }
    }

    /** Get the world's current time. */
    pub fn get_time(&self) -> usize {
        self.clock
    }

    /** Increase clock and proceed decay of all things and relations. */
    pub fn tick(&mut self) {
        self.clock += 1;

        /* A new day is over: Forward the day structure to the world. */
        let new_day: bool = self.clock % Self::TICKS_PER_DAY == 0;

        let mut some_busy_wusel: Vec<wusel::WuselId> = vec![];
        let mut new_babies: Vec<(wusel::WuselId, Option<wusel::WuselId>, wusel::WuselGender)> = vec![];
        let mut dying_wusels: Vec<wusel::WuselId> = vec![];

        /* Decay on every object and living. */
        for (i, wusel) in self.wusels.iter_mut().enumerate() {
            /* Watch all tasks, remove tasks, which may be aborted or ran out. */
            wusel.auto_clean_tasks();

            /* Peek into the ongoing task, and maybe proceed them.
             * This may lead to remove the done task. */
            if !wusel.has_tasklist_empty() {
                some_busy_wusel.push(i);
            } else {
                /* Wusel is currently not busy. => maybe apply an idle/auto task. */
            }

            /* If pregnant: Maybe push out the child => Failure, Early or too late. */
            if wusel.is_pregnant() {
                let other_parent: Option<usize> = wusel.get_other_parent();
                let pregnancy_days: Option<u8> = wusel.get_remaining_pregnancy_days();
                let maybe_now: u8 = rand::random::<u8>() % 100;
                let possibility: u8 = match pregnancy_days {
                    Some(0) => 90,
                    Some(1) => 75,
                    _ => 10,
                };
                if (0u8..possibility).contains(&maybe_now) {
                    log::debug!("Pop the baby!");
                    let gender = wusel::WuselGender::random();
                    new_babies.push((wusel.get_id(), other_parent, gender));
                    // end pregnancy.
                    wusel.set_pregnancy(None, None);
                }
            }

            let alive = wusel.wusel_tick(new_day);

            /* The wusel just died. Remove if from active wusels later. */
            if !alive {
                dying_wusels.push(i);
            }
        }

        /* Execute ongoing tasks, unmutable wusel context.. */
        for w in some_busy_wusel.iter() {
            if let Some(t) = self.wusels[*w].peek_ongoing_task() {
                /* Decide how to progress the command. */
                let u = (*t).clone();
                task_manager::proceed(self, u);
            }
        }

        for _ in self.relations.iter() { /* Decay of relations over time. */ }

        /* Command further name giving and attention from the player. */
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

    /* World Inventory. */
    // const WORLD_INVENTORY: Where = InWorld::InStorageId((objects::ObjectType::Miscellaneous, "World-Storage", 0));

    pub const TICKS_PER_DAY: usize = 2880; // 24h by 0.5 minutes

    /** Get width of the world. */
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /** Get height of the world. */
    pub fn get_depth(&self) -> u32 {
        self.depth
    }

    pub fn get_dimensions(&self) -> (u32, u32, u32) {
        (self.width, self.depth, 0)
    }

    pub fn get_area(&self) -> areas::Area {
        areas::Area::new(areas::Position::new(0, 0), self.width, self.depth)
    }

    /** Get the `positions` index for the requesting position (width, height).
     * If the position is not in world, this index is not in [0, positions.len()).*/
    fn position_to_index(&self, position: areas::Position) -> usize {
        (position.x + self.width * position.y) as usize
    }

    /** Get the position tuple from the given index in this world. */
    fn position_from_index(&self, position_index: usize) -> Option<areas::Position> {
        if position_index < self.position_upper_bound {
            Some(
                areas::Position::new(
                    position_index as u32 % self.width,
                    position_index as u32 / self.width
                    )
                )
        } else {
            None
        }
    }

    /** Get a random position in this world. */
    pub fn position_random(&self) -> areas::Position {
        self.area.position_random()
    }

    /** Get the (valid) neighbours for a position. */
    pub fn position_get_all_neighbours(&self, position: areas::Position) -> Vec<areas::Position> {
        self.area.get_all_neighbours(position)
    }

    /** Get the next optional neighbour to the given position within the given box. */
    pub fn position_get_neighbour_on(
        &self,
        position: areas::Position,
        direction: areas::Way,
    ) -> Option<areas::Position> {
        self.area.get_directed_neighbour(position, direction)
    }

    /** Check if the position is inside the world bounds. */
    pub fn has_position(&self, position: areas::Position) -> bool {
        self.area.contains_position(&position)
    }

    /** Get the distance between two positions represented by indices in this world. */
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

    pub fn positions_for_all_placetakers(&self) -> Vec<Vec<PlaceTaker>> {
        self.positions.clone()
    }

    pub fn recalculate_positions_for_all_placetakers(&mut self) {
        // clear old positions.
        for placetakers in self.positions.iter_mut() {
            placetakers.clear();
        }

        // for constructions
        let constructions_index_on_position_index: Vec<usize> = vec![];
        for (construction_index, &constructions_position_index) in constructions_index_on_position_index.iter().enumerate() {

            self.positions[constructions_position_index].push(PlaceTaker::Construction(construction_index));

        }

        for (wusel_index, &wusel_position_index) in self.wusels_index_on_position_index.iter().enumerate() {

            self.positions[wusel_position_index].push(PlaceTaker::Wusel(self.wusels_index_with_id[wusel_index]));

        }

        for (object_index, object_whereabouts) in self.objects_index_with_whereabouts.iter().enumerate() {
            if let InWorld::OnPositionIndex(object_position_index) = *object_whereabouts {

                self.positions[object_position_index].push(PlaceTaker::Object(self.objects_index_with_id[object_index]));

            }
        }
    }

    fn update_positions(&mut self, placetaker: PlaceTaker, old_position_index: usize, new_position_index: usize) {
        // not if both position indices are invalid / higher / "not given", it just does nothing.
        // this also can remove a place taker from the map, or put it there on the first place.

        // remove from old position if given.
        if old_position_index < self.position_upper_bound {
            let opt_placetaker_index: Option<usize>
                = self.positions[old_position_index].iter()
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

    /** Wusel char. */
    const CHAR_WUSEL: char = '\u{263A}'; // smiley, alternatively or w

    /** Get the character representing an object type. */
    fn objecttype_as_char(t: objects::ObjectType) -> char {
        match t {
            objects::ObjectType::Construction => '#',  // '\u{1f4ba}', // wall
            objects::ObjectType::Furniture => 'm',     // '\u{1f4ba}', // chair
            objects::ObjectType::Miscellaneous => '*', // '\u{26ac}', // small circle
            objects::ObjectType::Food => 'ó',         // '\u{2615}', // hot beverage
        }
    }

    /** Create a new object to exist in this world.
     * Placed in a world inventory/storage first, can be placed in world.
     * Returns the new object's index for the world's objects. */
    pub fn object_new(
        &mut self,
        subtyped_object: objects::ObjectWithSubType,
        name: String,
        transportable: bool,
        passable: bool,
        consumable_parts: Option<usize>,
        storage_capacity: usize,
        ) -> objects::ObjectId {
        let (object_type, subtype) = subtyped_object;

        // /* Add the new object into the world active objects. */
        self.objects.push(
            objects::Object::new(
                name,
                object_type, subtype,
                transportable,
                passable,
                consumable_parts,
                storage_capacity,
                ));

        let object_id: objects::ObjectId = (
            self.objects.last_mut().unwrap().get_object_id().0,
            self.objects.last_mut().unwrap().get_object_id().1,
            self.sequential_object_id,
        );

        self.objects_index_with_whereabouts.push(InWorld::Nowhere);
        self.objects_index_with_id.push(object_id);

        log::info!("New object created: {:?}", self.objects.last_mut());

        self.sequential_object_id += 1;

        /* Return new ID and appended index. */
        object_id
    }

    /** Create a new food (an object) to exist in this world.
     * This calls `self.object_new(Food, name, true, 0)`.
     * => Food is transportable, no storage.
     *
     * Placed in a world inventory/storage first, can be placed in world.
     * Returns the new object's index for the world's objects. */
    pub fn food_new(&mut self, name: objects::ObjectSubtype, bites: usize) -> objects::ObjectId {
        self.object_new(
            (objects::ObjectType::Food, name),
            name.to_string(),
            true,
            true,
            Some(bites),
            0,
        )
    }

    /** Duplicate a world object: Use all attributes, but change the ID
     * This will create a new object, currently in world's storage. */
    pub fn object_duplicate(&mut self, base_index: usize) -> Option<objects::ObjectId> {
        /* Duplicate non existing?. */
        if base_index >= self.objects.len() {
            return None;
        }

        Some(self.object_new(
            (
                (self.objects[base_index]).get_object_id().0, // object type
                (self.objects[base_index]).get_object_id().1, // object sub-type
            ),
            (self.objects[base_index]).get_name(),
            (self.objects[base_index]).is_transportable(),
            (self.objects[base_index]).is_passable(),
            (self.objects[base_index]).get_consumable(),
            (self.objects[base_index]).get_storage_capacity(),
        ))
    }

    fn get_objects_index_by_id(&self, object_id: objects::ObjectId) -> Option<usize> {
        self.objects_index_with_id.iter().position(|id| *id == object_id)
    }

    fn get_object_whereabouts_by_id(&self, object_id: objects::ObjectId) -> Option<&InWorld> {
        if let Some(object_index) = self.get_objects_index_by_id(object_id) {
            self.objects_index_with_whereabouts.get(object_index)
        } else {
            None
        }
    }

    /** Find the optional index of an object, given by an ID. */
    fn object_id_to_index(&self, object_id: objects::ObjectId) -> Option<usize> {
        self.objects
            .iter()
            .position(|o| o.get_object_id() == object_id)
    }

    /** Get the optional position of an object, given by an index.
     * If the position is held by a storage, get the position of the storage. */
    fn objects_index_get_position(&self, object_index: usize) -> Option<areas::Position> {
        match self.objects_index_with_whereabouts.get(object_index) {
            Some(InWorld::OnPositionIndex(position_index)) => {
                self.position_from_index(*position_index)
            },
            Some(InWorld::HeldByWuselId(wusel_id)) => {
                // get nested position of holder.
                self.get_wusels_index_by_id(*wusel_id)
                    .map(|holder_index| self.wusels_index_on_position_index[holder_index])
                    .map(|wusel_position_index| self.position_from_index(wusel_position_index))
                    .map(|opt_opt_position| opt_opt_position.unwrap())

            },
            Some(InWorld::InStorageId(storage_object_id)) => {
                // get nested position (of storage).
                self.object_get_position(*storage_object_id)
            },
            _ => None
        }
    }

    /** Get the optional position of an object, given by an ID.
     * If the position is held by a storage, get the position of the storage. */
    pub fn object_get_position(&self, object_id: objects::ObjectId) -> Option<areas::Position> {
        if let Some(object_index) = self.object_id_to_index(object_id) {
            self.objects_index_get_position(object_index)
        } else {
            None
        }
    }

    /** Get the positions of all InWorld::OnPositionIndex objects. */
    #[allow(dead_code)]
    pub fn positions_for_objects(&self) -> Vec<areas::Position> {
        // unique positions.
        self.objects_index_with_whereabouts
            .iter()
            .filter(|whereabout| matches!(whereabout, InWorld::OnPositionIndex(_)))
            .map(|on_position_index| if let InWorld::OnPositionIndex(position_index) = on_position_index { self.position_from_index(*position_index) } else { None })
            .flatten()
            .collect()
    }


    /** Place an object on a new position (in world).
     * If the object was held or stored before, it is now not anymore. */
    pub fn object_set_position(&mut self, object_id: objects::ObjectId, position: areas::Position) {
        if let Some(object_index) = self.object_id_to_index(object_id) {
            let placetaker = PlaceTaker::Object(object_id);
            let old_position_index
                = match self.objects_index_with_whereabouts[object_index] {
                    InWorld::OnPositionIndex(old_position_index) => old_position_index,
                    _ => self.position_upper_bound, // none (out of world).
                };
            let new_position_index = self.position_to_index(position);

            self.object_set_whereabouts(object_index, InWorld::OnPositionIndex(new_position_index));

            self.update_positions(placetaker, old_position_index, new_position_index);
        }
    }

    /** Place an object on a new position, or store it within an inventory, or let it held by a wusel.
     * The object is given by an (vector) index of all currently active objects.
     * If the object is removed from a world position, this will remove the object from the old
     * position.  */
    fn object_set_whereabouts(&mut self, object_index: usize, whereto: InWorld) {
        /* Invalid index. => Abort. */
        if object_index >= self.objects.len() {
            return;
        }

        // just update.
        self.objects_index_with_whereabouts[object_index] = whereto;
    }

    /** Destroy an object given by a certain all-active-object's index. */
    fn object_destroy(&mut self, object_index: usize) {
        if object_index >= self.objects.len() {
            return;
        }

        self.objects.remove(object_index);
        self.objects_index_with_whereabouts.remove(object_index);
        self.objects_index_with_id.remove(object_index);
    }

    /** Add a wusel to the world.
     * ID is the current wusel count.
     * TODO (2020-11-20) what is about dead wusels and decreasing length? */
    pub fn wusel_new(&mut self, name: String, gender: wusel::WuselGender, position: areas::Position) {
        let new_wusel_id = self.sequential_wusel_id; // almost id (for a long time unique)
        let new_wusel = wusel::Wusel::new(new_wusel_id, name, gender); // new wusel at (position)

        /* Add wusel to positions, start at (position). */
        let position_index = self.position_to_index(position);

        // XXX put new wusel on position.
        self.wusels.push(new_wusel);
        self.wusels_index_with_id.push(new_wusel_id); // fast access id.
        self.wusels_index_on_position_index.push(position_index); // access position.

        // self.wusels_positions.push(position_index); // index.
        self.sequential_wusel_id += 1;
    }

    pub fn wusel_new_random(&mut self, wusel_name: String) {
        let wusel_gender = wusel::WuselGender::random();
        let wusel_position = areas::Position {
            x: rand::random::<u32>() % self.get_width(),
            y: rand::random::<u32>() % self.get_depth(),
        };
        self.wusel_new(wusel_name, wusel_gender, wusel_position);
    }

    /** Count how many wusels are currently active. */
    pub fn wusel_count(&self) -> usize {
        self.wusels.len()
    }

    fn check_valid_wusel_index(&self, wusel_index: usize) -> bool {
        wusel_index < self.wusels.len()
    }

    fn get_wusels_index_by_id(&self, wusel_id: usize) -> Option<usize> {
        self.wusels_index_with_id.iter().position(|id| *id == wusel_id)
    }

    fn get_wusel_position_index_by_id(&self, wusel_id: usize) -> Option<&usize> {
        if let Some(wusel_index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels_index_on_position_index.get(wusel_index)
        } else {
            None
        }
    }

    pub fn wusel_get_position(&self, wusel_id: usize) -> Option<areas::Position> {
        self.get_wusel_position_index_by_id(wusel_id)
            .map(|&position_index| self.position_from_index(position_index))
            .map(|opt_position| opt_position.unwrap())
    }


    /** Set the position of the indexed wusel to the nearest valid position
     * If the position may land out of the grid, put it to the nearest border. */
    pub fn wusel_set_position(&mut self, wusel_id: usize, position: areas::Position) {
        if let Some(&wusel_index) = self.get_wusel_position_index_by_id(wusel_id) {
            self.wusel_set_position_by_index(wusel_index, position);
        }
    }

    /** Set the position of the indexed wusel to the nearest valid position
     * If the position may land out of the grid, put it to the nearest border. */
    fn wusel_set_position_by_index(&mut self, wusel_index: usize, position: areas::Position) {
        if self.check_valid_wusel_index(wusel_index) {
            let placetaker = PlaceTaker::Wusel(self.wusels_index_with_id[wusel_index]);
            let old_position_index = self.wusels_index_on_position_index[wusel_index].clone();
            let new_position_index = self.position_to_index(position);

            self.wusels_index_on_position_index[wusel_index] = new_position_index;

            self.update_positions(placetaker, old_position_index, new_position_index);
        }
    }

    /** Get the positions of all active wusels. */
    #[allow(dead_code)]
    pub fn positions_for_wusels(&self) -> Vec<areas::Position> {
        // unique positions.
        self.wusels_index_on_position_index
            .iter()
            .map(|&position_index| self.position_from_index(position_index))
            .flatten()
            .collect()
    }

    /** Get the indices of all wusels, which are alive. */
    pub fn wusel_get_all_alive(&self) -> Vec<usize> {
        let mut alive: Vec<usize> = vec![];
        for i in 0..self.wusels.len() {
            if self.wusels[i].is_alive() {
                alive.push(i);
            }
        }
        alive
    }

    /** Get the indices of all wusels, which are currently having no tasks to do. */
    pub fn wusel_get_all_unbusy(&self) -> Vec<usize> {
        let mut unbusy: Vec<usize> = vec![];
        for i in 0..self.wusels.len() {
            if self.wusels[i].has_tasklist_empty() {
                unbusy.push(i);
            }
        }
        unbusy
    }

    pub fn wusel_is_alive(&self, wusel_id: wusel::WuselId) -> Option<bool> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].is_alive())
    }

    pub fn wusel_get_lived_days(&self, wusel_id: wusel::WuselId) -> Option<u32> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].get_lived_days())
    }

    pub fn wusel_set_life_state(&mut self, wusel_id: wusel::WuselId, life_state: wusel::Life) {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index].set_life_state(life_state);
        }
    }

    pub fn wusel_get_name(&self, wusel_id: wusel::WuselId) -> Option<String> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].get_name())
    }

    pub fn wusel_set_name(&mut self, wusel_id: wusel::WuselId, new_name: String) {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index].set_name(new_name);
        }
    }

    pub fn wusel_get_gender(&self, wusel_id: wusel::WuselId) -> Option<wusel::WuselGender> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].get_gender())
    }

    pub fn wusel_set_gender(&mut self, wusel_id: wusel::WuselId, new_gender: wusel::WuselGender) {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index].set_gender(new_gender);
        }
    }

    pub fn wusel_get_need(&mut self, wusel_id: wusel::WuselId, need: wusel::Need) -> u32 {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].get_need(need))
            .unwrap_or(0u32)
    }

    pub fn wusel_set_need(&mut self, wusel_id: wusel::WuselId, need: &wusel::Need, new_value: u32) {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index].set_need(*need, new_value);
        }
    }

    pub fn wusel_set_need_relative(
        &mut self,
        wusel_id: wusel::WuselId,
        need: &wusel::Need,
        relative: i16,
    ) {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index]
                .set_need_relative(*need, relative);
        }
    }

    pub fn wusel_get_ability(
        &self,
        wusel_id: wusel::WuselId,
        ability: wusel::Ability,
    ) -> Option<u32> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].get_ability(ability))
    }

    pub fn wusel_set_ability(
        &mut self,
        wusel_id: wusel::WuselId,
        ability: wusel::Ability,
        new_value: u32,
    ) {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index]
                .set_ability(ability, new_value);
        }
    }

    pub fn wusel_improve(&mut self, wusel_id: wusel::WuselId, ability: wusel::Ability) {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index].improve(ability);
        }
    }

    pub fn wusel_has_tasklist_empty(&self, wusel_id: wusel::WuselId) -> Option<bool> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].has_tasklist_empty())
    }

    pub fn wusel_get_tasklist_len(&self, wusel_id: wusel::WuselId) -> Option<usize> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].get_tasklist_len())
    }

    pub fn wusel_get_tasklist_names(&mut self, wusel_id: wusel::WuselId) -> Vec<String> {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index].get_tasklist_names()
        } else {
            vec![]
        }
    }

    /** Give an available wusel (by index) a new task. */
    pub fn wusel_assign_to_task(&mut self, wusel_index: usize, taskb: tasks::TaskBuilder) {
        if let Some(wusel) = self.wusels.get_mut(wusel_index) {
            /* Task apply wusel[index] as actor. */
            wusel.assign_to_task(self.clock, taskb);
            log::debug!("task successfully assigned")
        }
    }

    pub fn wusel_abort_task(&mut self, wusel_id: wusel::WuselId, task_index: usize) {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index].abort_task(task_index);
        }
    }

    pub fn wusel_peek_ongoing_task(&self, wusel_id: wusel::WuselId) -> Option<&tasks::Task> {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index].peek_ongoing_task()
        } else {
            None
        }
    }

    pub fn wusel_is_pregnant(&self, wusel_id: wusel::WuselId) -> Option<bool> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].is_pregnant())
    }
    pub fn wusel_set_pregnancy(
        &mut self,
        wusel_id: wusel::WuselId,
        other_parent: Option<usize>,
        remaining_days: Option<u8>,
    ) {
        if let Some(index) = self.get_wusels_index_by_id(wusel_id) {
            self.wusels[index]
                .set_pregnancy(other_parent, remaining_days);
        }
    }
    pub fn wusel_get_other_parent(&self, wusel_id: wusel::WuselId) -> Option<usize> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| self.wusels[index].get_other_parent())
            .unwrap_or(None)
    }
    pub fn wusel_get_remaining_pregnancy_days(&self, wusel_id: wusel::WuselId) -> Option<u8> {
        self.get_wusels_index_by_id(wusel_id)
            .map(|index| {
                self.wusels[index].get_remaining_pregnancy_days()
            })
            .unwrap_or(None)
    }

    /** Show all relations for a wusel, given by index.
     * Prints directly to std::out. */
    pub fn wusel_show_relations(&self, wusel_index: usize) {
        if wusel_index >= self.wusels.len() {
            println!("There is no wusel to show.");
            return;
        }

        let wusel_id = self.wusels[wusel_index].get_id();

        print!(
            "Relations with {}: ",
            self.wusels[wusel_index].get_name()
        );

        let mut has_relations: bool = false;

        for (who, relation) in self.relations.iter() {
            let other_id: usize;

            /* Get the other wusel.
             * Skip where this wusel is even not part in the relation. */
            if wusel_id == who.0 {
                other_id = who.1;
            } else if wusel_id == who.1 {
                other_id = who.0;
            } else {
                continue;
            } // not in relation

            let other_name = self.wusels[other_id].get_name();

            /* Print Relation. */
            print!("[{:?}: {}]", other_name, relation.show());
            has_relations = true;
        }

        if !has_relations {
            print!("Has no relations.");
        }

        println!();
    }

    /** Update the relation of two wusels, given by their ID. */
    pub fn wusel_update_relations(
        &mut self,
        wusel0_id: usize,
        wusel1_id: usize,
        nice: bool,
        relationtype: wusel::RelationType,
    ) {
        /* Decide for a relation key: (Greater ID, Smaller ID). */

        let key = if wusel0_id <= wusel1_id {
            (wusel0_id, wusel1_id)
        } else {
            (wusel1_id, wusel0_id)
        };

        let change = if nice { 1 } else { -1 };

        /* Get the relation if available.
         * update a key, guarding against the key possibly not being set. */
        let rel = self
            .relations
            .entry(key)
            .or_insert_with(wusel::Relation::new);

        rel.update(relationtype, change);
    }
}

#[derive(Clone, Copy, PartialEq)]
enum InWorld {
    OnPositionIndex(usize),
    InStorageId(objects::ObjectId),
    HeldByWuselId(usize),
    Nowhere,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PlaceTaker {
    Construction(usize),
    Wusel(usize),
    Object(objects::ObjectId),
}

/** A Blueprint is a list of required abilities, consumables or positions
 * to create a certain product after a certain time.
 * Blueprint: [ components, Workstation ] + Time => Product. */
#[allow(dead_code)]
struct Blueprint {
    id: usize,
    product: usize,
    components: Vec<usize>, // needed components: such as tools (desk) or ingredients (pen, paper).
    steps: usize,           // needed steps.
}

/** Something a Wusel can consume (= destroying by usage).
 * Consuming it might modify the needs and skills. */
#[derive(Clone, Debug)]
pub struct Consumable {
    name: String,

    /* Size representation: whole = 100% = size/size. */
    size: u32, // a size representation: consuming this [size]  times, the thing is gone. (fixed)
    available: f32, // 1.0f whole, 0.0f gone. (temporary)

    /* Sometimes, a consumable can spoil (> 0) */
    spoils_after: u32, // spoils after 0: infinite, or N days. (fixed)
    age: u32,          // the current age of the consumable (temporary)

    /* While consuming it, one part (1/size) while change the needs as following. */
    need_change: std::collections::HashMap<wusel::Need, i16>,
}
