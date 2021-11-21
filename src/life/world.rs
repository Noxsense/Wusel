/**
 * module Life.
 * - This module contains actuall all game world and life logics and mechanics.
 *
 * @author Nox
 * @version 2021.0.1
 */
use rand;

use crate::life::areas;
use crate::life::tasks;
use crate::life::wusel;

pub mod task_manager;

/** (Private) Wrapping Wusels and positions together. */
struct WuselOnPosIdx {
    wusel: wusel::Wusel,
    position_index: usize,
}

/** Where the object is stored / placed. */
#[derive(Debug, Clone, PartialEq)]
pub enum Where {
    AtPosition(usize),         // position index
    StoredIn(ObjectIdentifer), // storage ID (object ID of the storage)
    HeldBy(usize),             // held by a wusel (index)
}

/** (Private) Wrapping Objects and where abouts together. */
#[derive(Debug)]
struct ExistingObject {
    object: Box<Object>,
    position: Where,
}

/** The place of existence, time and relations. */
pub struct World {
    width: u32,
    depth: u32,
    positions: Vec<Vec<(char, usize)>>, // all positions [height x width] contain a vector of ids and type/set indicators.

    clock: usize, // time of the world.

    // wusels the main live force in here
    wusels_alltime_count: usize, // coount of all ever created wusels
    wusels_on_pos: Vec<WuselOnPosIdx>, // vector of [ wusels, their positions ]

    relations: std::collections::BTreeMap<(usize, usize), wusel::Relation>, // vector of wusel relations

    // wall or furniture or miscellaneous.
    objects: Vec<ExistingObject>,
    obj_count_furniture: usize, // all ever created furniture objects
    obj_count_misc: usize,      // all ever created miscellaneous objects
    obj_count_food: usize,      // all ever created food objects

    actions: Vec<String>,               // actions to do.
    actions_effects: Vec<tasks::ActionAffect>, // how various actions on various objects may influence
}

impl World {
    /** Create a new world. */
    pub fn new(width: u32, depth: u32) -> Self {
        Self {
            width,
            depth,
            positions: vec![vec![]; width as usize * depth as usize],

            clock: 0,

            wusels_alltime_count: 0,
            wusels_on_pos: vec![],
            relations: std::collections::BTreeMap::new(),

            objects: vec![],
            obj_count_furniture: 0,
            obj_count_misc: 0,
            obj_count_food: 0,

            actions: vec![
                String::from("View"),
                String::from("Take"),
                String::from("Drop"),
                String::from("Consume"),
            ],
            actions_effects: vec![
                (
                    (ObjectType::Food, "Bibimbap", 0),
                    0,
                    "View => Inspired.",
                    vec![],
                ),
                (
                    (ObjectType::Food, "Bibimbap", 3),
                    0,
                    "Consume => Fed.",
                    vec![],
                ),
                ((ObjectType::Food, "Bread", 0), 0, "View => Teased.", vec![]),
                (
                    (ObjectType::Food, "Bread", 0), // object type, subtype, any ID
                    3,                              // action id
                    "Consume => Fed.",              // effect description // placeholder
                    vec![(wusel::Need::WATER, -50), (wusel::Need::FOOD, 200)], // effect
                ),
            ],
        }
    }

    /* World Inventory. */
    const WORLD_INVENTORY: Where = Where::StoredIn((ObjectType::Miscellaneous, "World-Storage", 0));

    pub const TICKS_PER_DAY: usize = 2880; // 24h by 0.5 minutes

    /** Get width of the world. */
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /** Get height of the world. */
    pub fn get_depth(&self) -> u32 {
        self.depth
    }

    pub fn get_area(&self) -> areas::Area {
        areas::Area::new(areas::Position::new(0, 0), self.width, self.depth)
    }

    // self.positions[pos_index].push((Self::CHAR_OBJECT, object_id));

    /** Get the `positions` index for the requesting position (width, height).
     * If the position is not in world, this index is not in [0, positions.len()).*/
    fn position_to_index(&self, pos: areas::Position) -> usize {
        (pos.x + self.width * pos.y) as usize
    }

    /** Get the position tuple from the given index in this world. */
    fn position_from_index(&self, idx: usize) -> areas::Position {
        areas::Position::new(idx as u32 % self.width, idx as u32 / self.width)
    }

    /** Get a random position in this world. */
    pub fn position_random(&self) -> areas::Position {
        self.get_area().position_random()
    }

    /** Get the (valid) neighbours for a position. */
    pub fn position_get_all_neighbours(&self, pos: areas::Position) -> Vec<areas::Position> {
        self.get_area().get_all_neighbours(pos)
    }

    /** Get the next optional neighbour to the given position within the given box. */
    pub fn position_get_neighbour_on(&self, pos: areas::Position, direction: areas::Way) -> Option<areas::Position> {
        self.get_area().get_directed_neighbour(pos, direction)
    }

    /** Check if the position is inside the world bounds. */
    pub fn position_containing(&self, pos: areas::Position) -> bool {
        self.get_area().contains_position(&pos)
    }

    /** Get the distance between two positions represented by indices in this world. */
    #[allow(dead_code)]
    fn positions_indices_distance(&self, a_index: usize, b_index: usize) -> f32 {
        let a = self.position_from_index(a_index);
        let b = self.position_from_index(b_index);
        a.distance_to(&b)
    }

    /** Wusel char. */
    const CHAR_WUSEL: char = '\u{263A}'; // smiley, alternatively or w

    /** Get the character representing an object type. */
    fn objecttype_as_char(t: ObjectType) -> char {
        match t {
            ObjectType::Construction => '#',  // '\u{1f4ba}', // wall
            ObjectType::Furniture => 'm',     // '\u{1f4ba}', // chair
            ObjectType::Miscellaneous => '*', // '\u{26ac}', // small circle
            ObjectType::Food => 'ó',         // '\u{2615}', // hot beverage
        }
    }
    /** Check all positions.
     * Recalculate all positions, if they really consist what they promised. */
    #[allow(dead_code)]
    pub fn positions_recalculate_grid(&mut self) {
        self.positions = vec![vec![]; self.width as usize * self.depth as usize];

        let valid_index = self.positions.len();

        for (wusel_index, w) in self.wusels_on_pos.iter().enumerate() {
            let idx = self.wusels_on_pos[wusel_index].position_index;

            /* Add ID to position. */
            if idx < valid_index {
                self.positions[idx].push((Self::CHAR_WUSEL, w.wusel.get_id()));
            }
        }
    }

    /** Get the positions of all active wusels. */
    #[allow(dead_code)]
    pub fn positions_for_wusels(&self) -> Vec<areas::Position> {
        let mut positions = vec![];
        for w in self.wusels_on_pos.iter() {
            positions.push(self.position_from_index((*w).position_index)); // usize -> Position
        }
        positions
    }

    /** Get all the positions as they are. */
    pub fn positions_for_grid(&self) -> Vec<Vec<(char, usize)>> {
        self.positions.clone()
    }

    /** From an object's ID to a grid (representation) ID. */
    fn objectid_as_gridid(obj_id: &ObjectIdentifer) -> (char, usize) {
        (Self::objecttype_as_char((*obj_id).0), (*obj_id).2)
    }

    /** Find a given thing (given by `ID`), placed on a certain position (given by `position_index`). */
    fn positions_find_index(&self, position_index: usize, id: &(char, usize)) -> Option<usize> {
        self.positions[position_index]
            .iter()
            .position(|obj_id| obj_id == id)
    }

    /** Create a new object to exist in this world.
     * Placed in a world inventory/storage first, can be placed in world.
     * Returns the new object's index for the world's objects. */
    pub fn object_new(
        &mut self,
        subtyped_object: ObjectWithSubType,
        name: String,
        transportable: bool,
        passable: bool,
        consumable_parts: Option<usize>,
        storage_capacity: usize,
    ) -> (ObjectIdentifer, usize) {
        let (obj_type, subtype) = subtyped_object;

        /* Which object's counter to increase. */
        let new_obj_count: usize = match obj_type {
            ObjectType::Construction => {
                // TODO (2021-01-21) ... construction such as walls.
                // self.obj_count_furniture += 1;
                // self.obj_count_furniture // increase and return.
                1
            }
            ObjectType::Furniture => {
                self.obj_count_furniture += 1;
                self.obj_count_furniture // increase and return.
            }
            ObjectType::Food => {
                self.obj_count_food += 1;
                self.obj_count_food // increase and return.
            }
            ObjectType::Miscellaneous => {
                self.obj_count_misc += 1;
                self.obj_count_misc // increase and return.
            }
        };

        /* Add the new object into the world active objects. */
        self.objects.push(ExistingObject {
            object: Box::new(Object {
                name,
                object_id: (obj_type, subtype, new_obj_count),
                transportable,
                passable,
                consumable: consumable_parts,
                storage_capacity,
            }),
            position: Self::WORLD_INVENTORY,
        });

        log::info!("New object created: {:?}", self.objects.last_mut());

        /* Return new ID and appended index. */
        (
            self.objects.last_mut().unwrap().object.object_id,
            self.objects.len() - 1,
        )
    }

    /** Create a new food (an object) to exist in this world.
     * This calls `self.object_new(Food, name, true, 0)`.
     * => Food is transportable, no storage.
     *
     * Placed in a world inventory/storage first, can be placed in world.
     * Returns the new object's index for the world's objects. */
    pub fn food_new(&mut self, name: ObjectSubtype, bites: usize) -> (ObjectIdentifer, usize) {
        self.object_new(
            (ObjectType::Food, name),
            name.to_string(),
            true,
            true,
            Some(bites),
            0,
        )
    }

    /** Duplicate a world object: Use all attributes, but change the ID
     * This will create a new object, currently in world's storage. */
    pub fn object_duplicate(&mut self, base_index: usize) -> Option<(ObjectIdentifer, usize)> {
        /* Duplicate non existing?. */
        if base_index >= self.objects.len() {
            return None;
        }

        Some(self.object_new(
            (
                (*self.objects[base_index].object).object_id.0, // obj type
                (*self.objects[base_index].object).object_id.1, // obj sub-type
            ),
            (*self.objects[base_index].object).name.clone(),
            (*self.objects[base_index].object).transportable,
            (*self.objects[base_index].object).passable,
            (*self.objects[base_index].object).consumable,
            (*self.objects[base_index].object).storage_capacity,
        ))
    }

    /** Find the optional index of an object, given by an ID. */
    fn object_identifier_to_index(&self, object_id: ObjectIdentifer) -> Option<usize> {
        self.objects
            .iter()
            .position(|o| o.object.object_id == object_id)
    }

    /** Get the optional position of an object, given by an index.
     * If the position is held by a storage, get the pos of the storage. */
    fn object_index_get_position(&self, object_index: usize) -> Option<areas::Position> {
        match self.objects[object_index].position {
            Where::AtPosition(pos_index) => Some(self.position_from_index(pos_index)),
            // get nested position.
            Where::HeldBy(wusel_id) => {
                self.wusel_get_position(self.wusel_identifier_to_index(wusel_id))
            }
            Where::StoredIn(storage_obj_id) => self.object_get_position(storage_obj_id),
        }
    }

    /** Get the optional position of an object, given by an ID.
     * If the position is held by a storage, get the pos of the storage. */
    pub fn object_get_position(&self, object_id: ObjectIdentifer) -> Option<areas::Position> {
        if let Some(object_index) = self.object_identifier_to_index(object_id) {
            self.object_index_get_position(object_index)
        } else {
            None
        }
    }

    /** Place an object on a new position. */
    pub fn object_set_position(&mut self, object_id: ObjectIdentifer, pos: areas::Position) {
        if let Some(object_index) = self.object_identifier_to_index(object_id) {
            let position_index = self.position_to_index(pos);
            self.object_set_whereabouts(object_index, Where::AtPosition(position_index));
        }
    }

    /** Place an object on a new position, or store it within an inventory, or let it held by a wusel.
     * The object is given by an (vector) index of all currently active objects.
     * If the object is removed from a world position, this will remove the object from the old
     * position.  */
    fn object_set_whereabouts(&mut self, object_index: usize, whereto: Where) {
        /* Invalid index. => Abort. */
        if object_index >= self.objects.len() {
            return;
        }

        let object = &self.objects[object_index]; // immutable.
        let object_id = object.object.object_id;

        // positions: CHAR and ID.
        let obj_c = Self::objecttype_as_char(object_id.0); // super type
        let obj_i = object_id.2; // index

        if let Where::AtPosition(old_pos_index) = &object.position {
            /* Remove from old position. */
            if let Some(i) = self.positions_find_index(*old_pos_index, &(obj_c, obj_i)) {
                self.positions[*old_pos_index].remove(i);
            }
        }

        /* Update new where. */
        let object = &mut self.objects[object_index]; // now mutable.
        object.position = whereto;

        if let Where::AtPosition(new_pos_index) = self.objects[object_index].position {
            /* Change and update self.positions. */
            self.positions[new_pos_index].push((obj_c, obj_i));
        }
    }

    /** Destroy an object given by a certain all-active-object's index. */
    fn object_destroy(&mut self, object_index: usize) {
        if object_index >= self.objects.len() {
            return;
        }

        let ExistingObject {
            object: obj,
            position: wherefrom,
        } = &self.objects[object_index];

        /* Remove from grid / positions, if available. */
        if let Where::AtPosition(pos_index) = wherefrom {
            if let Some(i) =
                self.positions_find_index(*pos_index, &Self::objectid_as_gridid(&(obj.object_id)))
            {
                self.positions[*pos_index].remove(i);
            }
        }

        /* Finally remove. */
        self.objects.remove(object_index);
    }

    /** Get an index for the wusel with the requesting index.
     * Return LEN, if none is found. */
    pub fn wusel_identifier_to_index(&self, id: usize) -> Option<usize> {
        self.wusels_on_pos.iter().position(|w| w.wusel.get_id() == id)
    }

    /** Add a wusel to the world.
     * ID is the current wusel count.
     * TODO (2020-11-20) what is about dead wusels and decreasing length? */
    pub fn wusel_new(&mut self, name: String, gender: wusel::WuselGender, pos: areas::Position) {
        let id = self.wusels_alltime_count; // almost identifier (for a long time unique)
        let w = wusel::Wusel::new(id, name, gender); // new wusel at (pos)

        /* Add wusel to positions, start at (pos). */
        let pos_index = self.position_to_index(pos);
        if pos_index < self.positions.len() {
            self.positions[pos_index].push((Self::CHAR_WUSEL, w.get_id()));
        }

        self.wusels_on_pos.push(WuselOnPosIdx {
            wusel: w,
            position_index: pos_index,
        }); // wusel on position (by index)
            // self.wusels_positions.push(pos_index); // index.
        self.wusels_alltime_count += 1;
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
        self.wusels_on_pos.len()
    }

    /** Get the position of the indexed wusel. */
    pub fn wusel_get_position(&self, wusel_index: Option<usize>) -> Option<areas::Position> {
        if let Some(wusel_index) = wusel_index {
            if wusel_index < self.wusels_on_pos.len() {
                Some(self.position_from_index(self.wusels_on_pos[wusel_index].position_index))
            } else {
                None // outside the map.
            }
        } else {
            None
        }
    }

    /** Set the position of the indexed wusel to the nearest valid position
     * If the position may land out of the grid, put it to the nearest border. */
    pub fn wusel_set_position(&mut self, wusel_index: usize, pos: areas::Position) {
        if wusel_index < self.wusels_on_pos.len() {
            let id = self.wusels_on_pos[wusel_index].wusel.get_id();

            /* Update the self.positions. */
            let old_pos_index = self.wusels_on_pos[wusel_index].position_index;

            let new_pos = areas::Position::new(u32::min(pos.x, self.width), u32::min(pos.y, self.depth));
            let new_pos_index = self.position_to_index(new_pos);

            /* Set the new position. */
            self.wusels_on_pos[wusel_index].position_index = new_pos_index;

            /* Representation in positions. */
            let wusel_indicator = (Self::CHAR_WUSEL, id);

            /* Remove from old positions[idx]. */
            for i in 0..self.positions[old_pos_index].len() {
                if self.positions[old_pos_index][i] == wusel_indicator {
                    self.positions[old_pos_index].remove(i);
                    break;
                }
            }

            /* Add to new positions[idx]. */
            self.positions[new_pos_index].push(wusel_indicator);
        }
    }

    /** Get the indices of all wusels, which are alive. */
    pub fn wusel_get_all_alive(&self) -> Vec<usize> {
        let mut alive: Vec<usize> = vec![];
        for i in 0..self.wusels_on_pos.len() {
            if self.wusels_on_pos[i].wusel.is_alive() {
                alive.push(i);
            }
        }
        alive
    }

    /** Get the indices of all wusels, which are currently having no tasks to do. */
    pub fn wusel_get_all_unbusy(&self) -> Vec<usize> {
        let mut unbusy: Vec<usize> = vec![];
        for i in 0..self.wusels_on_pos.len() {
            if self.wusels_on_pos[i].wusel.is_tasklist_empty() {
                unbusy.push(i);
            }
        }
        unbusy
    }

    /** Give an available wusel (by index) a new task. */
    pub fn wusel_assign_task(&mut self, wusel_index: usize, taskb: tasks::TaskBuilder) {
        if wusel_index < self.wusels_on_pos.len() {
            /* Task apply wusel[index] as actor. */
            self.wusels_on_pos[wusel_index]
                .wusel
                .add_task(self.clock, taskb);
            log::debug!("task successfully assigned")
        }
    }

    /** Abort an assigned task from an available wusel (by index). */
    pub fn wusel_abort_task(&mut self, wusel_index: usize, task_index: usize) {
        if wusel_index < self.wusels_on_pos.len() {
            /* Remove task. */
            self.wusels_on_pos[wusel_index].wusel.abort_task(task_index);
        }
    }

    /** Show all relations for a wusel, given by index.
     * Prints directly to std::out. */
    pub fn wusel_show_relations(&self, wusel_index: usize) {
        if wusel_index >= self.wusels_on_pos.len() {
            println!("There is no wusel to show.");
            return;
        }

        let wusel_id = self.wusels_on_pos[wusel_index].wusel.get_id();

        print!(
            "Relations with {}: ",
            self.wusels_on_pos[wusel_index].wusel.get_name()
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

            let other_name = self.wusels_on_pos[other_id].wusel.get_name();

            /* Print Relation. */
            print!("[{:?}: {}]", other_name, relation.show());
            has_relations = true;
        }

        if !has_relations {
            print!("Has no relations.");
        }

        println!();
    }

    pub fn wusel_get_name(&self, wusel_id: usize) -> Option<String> {
        self.wusel_identifier_to_index(wusel_id)
            .map(|index| self.wusels_on_pos[index].wusel.get_name())
    }

    pub fn wusel_get_gender(&self, wusel_id: usize) -> Option<wusel::WuselGender> {
        self.wusel_identifier_to_index(wusel_id)
            .map(|index| self.wusels_on_pos[index].wusel.get_gender())
    }

    pub fn wusel_get_tasklist_names(&mut self, wusel_id: usize) -> Vec<String> {
        if let Some(index) = self.wusel_identifier_to_index(wusel_id) {
            self.wusels_on_pos[index].wusel.get_tasklist_names()
        } else {
            vec![]
        }
    }

    pub fn wusel_get_need_full(&mut self, need: wusel::Need) -> u32 {
        wusel::Wusel::default_need_full(&need)
    }

    /** Get the wusel's need. */
    pub fn wusel_get_need(&mut self, wusel_id: usize, need: wusel::Need) -> u32 {
        if let Some(index) = self.wusel_identifier_to_index(wusel_id) {
            self.wusels_on_pos[index].wusel.get_need(need)
        } else {
            0
        }
    }

    /** Set the wusel's need to a new value. */
    pub fn wusel_set_need(&mut self, wusel_id: usize, need: &wusel::Need, new_value: u32) {
        if let Some(index) = self.wusel_identifier_to_index(wusel_id) {
            self.wusels_on_pos[index].wusel.set_need(*need, new_value);
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

        let mut some_busy_wusel: Vec<usize> = vec![];
        let mut new_babies: Vec<(usize, Option<usize>, wusel::WuselGender)> = vec![];
        let mut dying_wusels: Vec<usize> = vec![];

        /* Decay on every object and living. */
        for (i, w) in self.wusels_on_pos.iter_mut().enumerate() {
            /* Watch all tasks, remove tasks, which may be aborted or ran out. */
            w.wusel.auto_clean_tasks();

            /* Peek into the ongoing task, and maybe proceed them.
             * This may lead to remove the done task. */
            if !w.wusel.is_tasklist_empty() {
                some_busy_wusel.push(i);
            } else {
                /* Wusel is currently not busy. => maybe apply an idle/auto task. */
            }

            /* If pregnant: Maybe push out the child => Failure, Early or too late. */
            if w.wusel.is_pregnant() {
                let other_parent: Option<usize> = w.wusel.get_other_parent();
                let pregnancy_days: Option<u8> = w.wusel.get_remaining_pregnancy_days();
                let maybe_now: u8 = rand::random::<u8>() % 100;
                let possibility: u8 = match pregnancy_days {
                    Some(0) => 90,
                    Some(1) => 75,
                    _ => 10,
                };
                if (0u8..possibility).contains(&maybe_now) {
                    log::debug!("Pop the baby!");
                    let gender = wusel::WuselGender::random();
                    new_babies.push((w.wusel.get_id(), other_parent, gender));
                }
            }

            let alive = w.wusel.wusel_tick(new_day);

            /* The wusel just died. Remove if from active wusels later. */
            if !alive {
                dying_wusels.push(i);
            }
        }

        /* Execute ongoing tasks, unmutable wusel context.. */
        for w in some_busy_wusel.iter() {
            if let Some(t) = self.wusels_on_pos[*w].wusel.peek_ongoing_task() {
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
        }
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
       let rel = self.relations.entry(key).or_insert_with(wusel::Relation::new);

       rel.update(relationtype, change);
    }
}

/** Types of an object. */
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObjectType {
    Construction,
    Furniture,
    Miscellaneous,
    Food,
}

type ObjectWithSubType = (ObjectType, ObjectSubtype);

/** Identifier type (tuple) for an object. */
pub type ObjectIdentifer = (ObjectType, ObjectSubtype, usize);

type ObjectSubtype = &'static str; // String doesn't support Copy Trait, what is used for the TaskTag.

/** A world object indicates an object in the world which is not a wusel. */
#[derive(Debug, Clone)]
struct Object {
    name: String,
    object_id: ObjectIdentifer,
    transportable: bool, // can be transported by a wusel, will also apply stotable
    passable: bool,      // if true, wusel can walk over it's occupied place (if at position)
    consumable: Option<usize>, // if None: cannot be consumed; if Some(bites): number of parts, that can be consumed
    storage_capacity: usize,   // items that can be stored 0
}

/** A Recipe is a list of required abilities, consumables or positions
 * to create a certain product after a certain time.
 * Recipe: [ components, Workstation ] + Time => Product. */
#[allow(dead_code)]
struct Recipe {
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

/** Test doing tasks. */
#[cfg(test)]
mod test {

    // use super;

    #[test]
    fn test_consume_bread() {
        // TODO refactor test.

        log::debug!("[TEST] Creating new stuff, let the wusels eat the bread.");
        let mut test_world: super::World = super::World::new(20, 5); // small world.
        log::debug!("Test World created");

        /* Empty test_world tick. */
        test_world.tick();
        log::debug!("Test World ticked");

        test_world.wusel_new(
            "Eater".to_string(),
            super::wusel::WuselGender::Female,
            super::areas::Position::new(1, 0),
        ); // female
        test_world.wusel_new(
            "Starver".to_string(),
            super::wusel::WuselGender::Male,
            super::areas::Position::new(2, 0),
        ); // male
        log::debug!("Test World's wusels created.");

        /* Create food: transportable, no storage. */
        let food1 = test_world.food_new("Bread", 100);

        let (food1_id, food1_index) = food1;

        log::debug!("Test World's food created, index: {}.", food1_index);

        let food2 = test_world.object_duplicate(0).unwrap(); // unsafe, but must be true.

        let (food2_id, food2_index) = food2;
        test_world.object_set_position(food2_id, test_world.position_random());

        log::debug!("Test World's food duplicated, index: {}.", food2_index);

        /* Put a copy into the world. */
        test_world.object_set_position(food1_id, test_world.position_random());

        log::debug!("Test World's food put onto a position.");

        /* Get the food and transport it somewhere else. */
        test_world.wusel_assign_task(1, super::tasks::TaskBuilder::use_object(food1_id, 1)); // take
        test_world.wusel_assign_task(1, super::tasks::TaskBuilder::move_to(test_world.position_random()));
        test_world.wusel_assign_task(1, super::tasks::TaskBuilder::use_object(food1_id, 2)); // drop
        test_world.wusel_assign_task(1, super::tasks::TaskBuilder::move_to(test_world.position_random()));
        test_world.wusel_assign_task(1, super::tasks::TaskBuilder::use_object(food1_id, 1)); // take not exisiting?

        /* Let the other wusel wait, than it's tries to get the food as well, and consume it. */
        test_world.wusel_assign_task(
            0,
            super::tasks::TaskBuilder::move_to(super::areas::Position::new(
                test_world.get_width() - 1,
                test_world.get_depth() - 1,
            )),
        );
        test_world.wusel_assign_task(0, super::tasks::TaskBuilder::use_object(food1_id, 1)); // take as well.
        test_world.wusel_assign_task(0, super::tasks::TaskBuilder::move_to(test_world.position_random()));
        test_world.wusel_assign_task(0, super::tasks::TaskBuilder::use_object(food1_id, 3)); // consume.
        test_world.wusel_assign_task(0, super::tasks::TaskBuilder::move_to(test_world.position_random()));
        test_world.wusel_assign_task(0, super::tasks::TaskBuilder::move_to(test_world.position_random()));
        log::debug!("Test World's task to work at the workbench assigned.");

        // show everyone's stats.
        for i in 0usize..2 {
            // test_world.wusel_show_tasklist(i); // tasks
            for n in super::Need::VALUES.iter() {
                test_world.wusel_set_need(i, n, 100);
            }
        }
        log::debug!("Test World's wusels' needs artificially reduced.");

        /* Show the grid.. */
        let (_w, _h): (usize, usize) = (
            test_world.get_width() as usize,
            test_world.get_depth() as usize,
        );

        println!(
            "{clear}{hide}",
            clear = termion::clear::All,
            hide = termion::cursor::Hide
        ); // clear the test screen

        for _ in 0..300 {
            // render_field(_w, _h, test_world.positions_for_grid());
            println!();
            log::debug!(
                "Test World's current grid, time: {}.",
                test_world.get_time()
            );

            test_world.tick(); // progress time.

            if !test_world.wusel_get_all_unbusy().is_empty() {
                log::debug!("Test world is done, to be busy.");
                break;
            }

            std::thread::sleep(std::time::Duration::from_millis(100)); // wait.
        }
    }

    /** Test doing tasks. */
    #[test]
    fn test_create_bread() {
        // TODO refactor test.

        // Example: Wusel wants to cook.
        // 1. Go to (free) cooking station: (move)
        // 2. Wait for the Station to be free
        // 3. Work on station.
        // 4. Fetch tomatoes to be cut and prepared (needs Tomatoes)
        // 5. Cut (consume) tomatoes, create sauce
        // 6. Heat up sauce. (> use up cold <? Consumable with extra states?)
        // 7. Creates hot tomato sauce. (can get cold or be eaten.)
        //
        // OPTIONAL
        // Or should tools also be "Consumed" after 1M uses?
        // Knife dull and then .. gone
        // Couch is sit broken after several times?

        /* Cook a meal, that needs a working station, tomatoes, a knife and pot.
         * Or knife and Pot as part of the station.
         * Cut a meal, boil the meal => consumes tomatoes, creates tomato soup. */

        // abort if difficulty is too high
        // walk to station.position, wait until free, block
        // get required ingredients
        // do required steps, eg. station changing, prbly a list of subtasks?

        // using objects may influence the needs and skills.
        // eg.
        // * eating uses energy, but fills water and hunger
        // * sleeping fills energy
        // * doing sports uses energy and water and fills sportivité abilities.
    }

    /** Test mutually meeting, which may cause deadlocks.
     * -----
     * 1at: [Read, Meet 2nd].
     * 2nd: [Meet 3rd]
     * 3rd: [Meet 4th]
     * 4th: [Meet 1st]
     * -----
     * 1at: [Read, Meet 2nd] + [Met by 4th]
     * 2nd: [Meet 3rd]
     * 3rd: [Meet 4th] + [Met by 2nd]
     * 4th: [Meet 1st] + [Met by 3rd]
     * -----
     * 1st done with reading and wants to meet 2nd.
     * -----
     * 1at: [Meet 2nd, Met by 4th]
     * 2nd: [Meet 3rd] + [Met by 1st]
     * 3rd: [Meet 4th, Met by 2nd]
     * 4th: [Meet 1st, Met by 3rd]
     * -----
     * Nothing happens, since everyone waits for the other to be done.
     * 2nd, 3rd and 4th stop meeting. (they waited too long)
     * -----
     * 1at: [Meet 2nd, Met by 4th]
     * 2nd: [Met by 1st]
     * 3rd: [Met by 2nd]
     * 4th: [Met by 3rd]
     * -----
     * The active meeter, they were about to be met by is gone, stop being met.
     * 1at: [Meet 2nd, Met by 4th]
     * 2nd: [Met by 1st]
     * 3rd: []
     * 4th: []
     * -----
     * 1st meets 2nd; 4th is not meeting 1st anymore. No tasks left.
     */
    #[test]
    fn test_mutal_meeting() {
        // TODO refactor test.

        println!("[test] Mutual Meeting, causes for circular deadlocks.");
        let mut test_world: super::World = super::World::new(80, 30);

        /* Empty test_world tick. */
        test_world.tick();

        test_world.wusel_new(
            "1st".to_string(),
            super::wusel::WuselGender::Female,
            super::areas::Position { x: 1, y: 0 },
        ); // female
        test_world.wusel_new(
            "2nd".to_string(),
            super::wusel::WuselGender::Female,
            super::areas::Position { x: 3, y: 0 },
        ); // female
        test_world.wusel_new(
            "3rd".to_string(),
            super::wusel::WuselGender::Male,
            super::areas::Position { x: 5, y: 0 },
        ); // male
        test_world.wusel_new(
            "4th".to_string(),
            super::wusel::WuselGender::Male,
            super::areas::Position { x: 9, y: 0 },
        ); // male

        // 4 wusels created.
        assert_eq!(4, test_world.wusel_count());

        /* Create an easy talk, without any preconditions.
         * => no preconditions.
         * => does 'nothing' for ticks steps. */
        let reading: super::tasks::TaskBuilder =
            super::tasks::TaskBuilder::new(String::from("Reading")).set_duration(5 /*ticks*/);

        test_world.tick();

        // first wusel is also doing something else
        test_world.wusel_assign_task(0, reading.clone()); // do reading.

        // scenario: everyone wants too meet the next one.
        test_world.wusel_assign_task(
            0,
            super::tasks::TaskBuilder::meet_with(1, true, false).set_duration(7),
        ); // mutual meeting.
        test_world.wusel_assign_task(
            1,
            super::tasks::TaskBuilder::meet_with(2, true, false).set_duration(7),
        ); // mutual meeting.
        test_world.wusel_assign_task(
            2,
            super::tasks::TaskBuilder::meet_with(3, true, false).set_duration(7),
        ); // mutual meeting.
        test_world.wusel_assign_task(
            3,
            super::tasks::TaskBuilder::meet_with(0, true, false).set_duration(7),
        ); // mutual meeting.

        /* 90 ticks later. */
        for _ in 0..90 {
            test_world.tick();
            // println!("\nTasks at time {}:", test_world.get_time());
            // for w in 0..4 { test_world.wusel_show_tasklist(w); }
        }
    }
}
