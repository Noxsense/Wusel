/**
 * module Life.
 * - This module contains actuall all game world and life logics and mechanics.
 * @author Nox
 * @version 2021.0.1
 */
use rand;

/** (Private) Wrapping Wusels and positions together. */
struct WuselOnPosIdx {
    wusel: Wusel,
    position_index: usize,
}

/** (Private) Wrapping Objects and where abouts together. */
#[derive(Debug)]
struct ExistingObject {
    object: Box<Object>,
    position: Where,
}

/** Simple position in world. */
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    /** Simple constructor. */
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    /** Get the distance between two positions. */
    pub fn distance_to(self: &Self, other: &Self) -> f32 {
        (((self.x as i64 - other.x as i64).pow(2) + (self.y as i64 - other.y as i64).pow(2)) as f32)
            .sqrt()
    }
}

/** Simple position in world. */
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Area {
    anchor: Position,
    width: u32,
    depth: u32,
    iterator_index: u32,
}

impl Area {
    pub fn new(anchor: Position, width: u32, depth: u32) -> Self {
        Self {
            anchor,
            width,
            depth,
            iterator_index: 0,
        }
    }

    /** Create an area, that is spanned by the given positions. */
    pub fn span(a: &Position, b: &Position) -> Self {
        let (min_x, max_x) = (<u32>::min(a.x, b.x), <u32>::max(a.x, b.x));
        let (min_y, max_y) = (<u32>::min(a.y, b.y), <u32>::max(a.y, b.y));

        return Area {
            anchor: Position::new(min_x, min_y),
            /* If only one position is spanned: width/depth: 1. */
            width: <u32>::max(1, max_x - min_x),
            depth: <u32>::max(1, max_y - min_y),
            iterator_index: 0,
        };
    }

    /** Check, if the position is in the area. */
    pub fn contains_position(self: &Self, pos: &Position) -> bool {
        (self.anchor.x <= pos.x && pos.x < (self.anchor.x + self.width))
            && (self.anchor.y <= pos.y && pos.y < (self.anchor.y + self.depth))
    }

    /** Get a random position within this area. */
    pub fn position_random(self: &Self) -> Position {
        Position::new(
            self.anchor.x + (rand::random::<u32>() % (self.anchor.x + self.width)),
            self.anchor.y + (rand::random::<u32>() % (self.anchor.y + self.depth)),
        )
    }

    /** Get all valid neighbours of a position within the area. */
    pub fn get_all_neighbours(self: &Self, pos: Position) -> Vec<Position> {
        // TODO (maka a storage, to not calculate it every time. )
        let mut neighbours: Vec<Position> = vec![];

        /* Get all the valid neighbours. */
        for d in Way::NEIGHBOURING.iter() {
            if let Some(n) = self.get_directed_neighbour(pos, *d) {
                neighbours.push(n);
            }
        }
        return neighbours;
    }

    /** Get a requested neighbour of a given position within this area. */
    pub fn get_directed_neighbour(self: &Self, pos: Position, direction: Way) -> Option<Position> {
        let change = direction.as_direction_tuple();

        let box_width = self.anchor.x + self.width;
        let box_depth = self.anchor.y + self.depth;

        /* On west border => No west neighbours. (None) */
        if pos.x < 1 && change.0 < 0 {
            return None;
        }

        /* On east border => No east neighbours. (None) */
        if pos.x >= box_width && change.0 > 0 {
            return None;
        }

        /* On south border => No south neighbours. (None) */
        if pos.y < 1 && change.1 < 0 {
            return None;
        }

        /* On north border => No north neighbours. (None) */
        if pos.y >= box_depth && change.1 > 0 {
            return None;
        }

        return Some(Position::new(
            (pos.x as i64 + change.0 as i64) as u32,
            (pos.y as i64 + change.1 as i64) as u32,
        ));
    }

    /** Get the optional position, which is on the given index. */
    pub fn position_from_index(self: &Self, index: u32) -> Option<Position> {
        if index < self.width * self.depth {
            Some(Position::new(
                index % self.width + self.anchor.x,
                index / self.width + self.anchor.y,
            ))
        } else {
            None
        }
    }
}

impl Iterator for Area {
    type Item = Position;

    /** Iterator over the positions of the field. */
    fn next(self: &mut Self) -> Option<Self::Item> {
        let index = self.iterator_index;
        self.iterator_index += 1;
        self.position_from_index(index)
    }
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

    relations: std::collections::BTreeMap<(usize, usize), Relation>, // vector of wusel relations

    // wall or furniture or miscellaneous.
    objects: Vec<ExistingObject>,
    obj_count_furniture: usize, // all ever created furniture objects
    obj_count_misc: usize,      // all ever created miscellaneous objects
    obj_count_food: usize,      // all ever created food objects

    actions: Vec<String>, // actions to do.
    actions_effects: Vec<(ObjectIdentifer, usize, &'static str, Vec<(Need, i16)>)>, // how various actions on various objects may influence
}

impl World {
    /** Create a new world. */
    pub fn new(width: u32, depth: u32) -> Self {
        return Self {
            width: width,
            depth: depth,
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
                    vec![(Need::WATER, -50), (Need::FOOD, 200)], // effect
                ),
            ],
        };
    }

    /* World Inventory. */
    const WORLD_INVENTORY: Where = Where::StoredIn((ObjectType::Miscellaneous, "World-Storage", 0));

    pub const TICKS_PER_DAY: usize = 2880; // 24h by 0.5 minutes

    /** Get width of the world. */
    pub fn get_width(self: &Self) -> u32 {
        self.width
    }

    /** Get height of the world. */
    pub fn get_depth(self: &Self) -> u32 {
        self.depth
    }

    pub fn get_area(self: &Self) -> Area {
        Area::new(Position::new(0, 0), self.width, self.depth)
    }

    // self.positions[pos_index].push((Self::CHAR_OBJECT, object_id));

    /** Get the `positions` index for the requesting position (width, height).
     * If the position is not in world, this index is not in [0, positions.len()).*/
    fn position_to_index(self: &Self, pos: Position) -> usize {
        (pos.x + self.width * pos.y) as usize
    }

    /** Get the position tuple from the given index in this world. */
    fn position_from_index(self: &Self, idx: usize) -> Position {
        Position::new(idx as u32 % self.width, idx as u32 / self.width)
    }

    /** Get a random position in this world. */
    pub fn position_random(self: &Self) -> Position {
        self.get_area().position_random()
    }

    /** Get the (valid) neighbours for a position. */
    pub fn position_get_all_neighbours(self: &Self, pos: Position) -> Vec<Position> {
        self.get_area().get_all_neighbours(pos)
    }

    /** Get the next optional neighbour to the given position within the given box. */
    pub fn position_get_neighbour_on(
        self: &Self,
        pos: Position,
        direction: Way,
    ) -> Option<Position> {
        self.get_area().get_directed_neighbour(pos, direction)
    }

    /** Check if the position is inside the world bounds. */
    pub fn position_containing(self: &Self, pos: Position) -> bool {
        self.get_area().contains_position(&pos)
    }

    /** Get the distance between two positions represented by indices in this world. */
    fn positions_indices_distance(self: &Self, a_index: usize, b_index: usize) -> f32 {
        let a = self.position_from_index(a_index);
        let b = self.position_from_index(b_index);
        return a.distance_to(&b);
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
    pub fn positions_recalculate_grid(self: &mut Self) {
        self.positions = vec![vec![]; self.width as usize * self.depth as usize];

        let valid_index = self.positions.len();

        let mut wusel_index = 0usize;
        for w in self.wusels_on_pos.iter() {
            let idx = self.wusels_on_pos[wusel_index].position_index;
            wusel_index += 1;

            /* Add ID to position. */
            if idx < valid_index {
                self.positions[idx].push((Self::CHAR_WUSEL, w.wusel.id));
            }
        }
    }

    /** Get the positions of all active wusels. */
    #[allow(dead_code)]
    pub fn positions_for_wusels(self: &Self) -> Vec<Position> {
        let mut positions = vec![];
        for w in self.wusels_on_pos.iter() {
            positions.push(self.position_from_index((*w).position_index)); // usize -> Position
        }
        return positions;
    }

    /** Get all the positions as they are. */
    pub fn positions_for_grid(self: &Self) -> Vec<Vec<(char, usize)>> {
        self.positions.clone()
    }

    /** From an object's ID to a grid (representation) ID. */
    fn objectid_as_gridid(obj_id: &ObjectIdentifer) -> (char, usize) {
        (Self::objecttype_as_char((*obj_id).0), (*obj_id).2)
    }

    /** Find a given thing (given by `ID`), placed on a certain position (given by `position_index`). */
    fn positions_find_index(
        self: &Self,
        position_index: usize,
        id: &(char, usize),
    ) -> Option<usize> {
        self.positions[position_index]
            .iter()
            .position(|obj_id| obj_id == id)
    }

    /** Create a new object to exist in this world.
     * Placed in a world inventory/storage first, can be placed in world.
     * Returns the new object's index for the world's objects. */
    pub fn object_new(
        self: &mut Self,
        obj_type: ObjectType,
        subtype: ObjectSubtype,
        name: String,
        transportable: bool,
        passable: bool,
        consumable_parts: Option<usize>,
        storage_capacity: usize,
    ) -> (ObjectIdentifer, usize) {
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
                name: name,
                object_id: (obj_type, subtype, new_obj_count),
                transportable: transportable,
                passable: passable,
                consumable: consumable_parts,
                storage_capacity: storage_capacity,
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
    pub fn food_new(
        self: &mut Self,
        name: ObjectSubtype,
        bites: usize,
    ) -> (ObjectIdentifer, usize) {
        self.object_new(
            ObjectType::Food,
            name,
            name.to_string(),
            true,
            true,
            Some(bites),
            0,
        )
    }

    /** Duplicate a world object: Use all attributes, but change the ID
     * This will create a new object, currently in world's storage. */
    pub fn object_duplicate(
        self: &mut Self,
        base_index: usize,
    ) -> Option<(ObjectIdentifer, usize)> {
        /* Duplicate non existing?. */
        if base_index >= self.objects.len() {
            return None;
        }

        Some(self.object_new(
            (&*self.objects[base_index].object).object_id.0,
            (&*self.objects[base_index].object).object_id.1,
            (&*self.objects[base_index].object).name.clone(),
            (&*self.objects[base_index].object).transportable,
            (&*self.objects[base_index].object).passable,
            (&*self.objects[base_index].object).consumable,
            (&*self.objects[base_index].object).storage_capacity,
        ))
    }

    /** Find the optional index of an object, given by an ID. */
    fn object_identifier_to_index(self: &Self, object_id: ObjectIdentifer) -> Option<usize> {
        self.objects
            .iter()
            .position(|o| o.object.object_id == object_id)
    }

    /** Get the optional position of an object, given by an index.
     * If the position is held by a storage, get the pos of the storage. */
    fn object_index_get_position(self: &Self, object_index: usize) -> Option<Position> {
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
    pub fn object_get_position(self: &Self, object_id: ObjectIdentifer) -> Option<Position> {
        if let Some(object_index) = self.object_identifier_to_index(object_id) {
            self.object_index_get_position(object_index)
        } else {
            None
        }
    }

    /** Place an object on a new position. */
    pub fn object_set_position(self: &mut Self, object_id: ObjectIdentifer, pos: Position) {
        if let Some(object_index) = self.object_identifier_to_index(object_id) {
            let position_index = self.position_to_index(pos);
            self.object_set_whereabouts(object_index, Where::AtPosition(position_index));
        }
    }

    /** Place an object on a new position, or store it within an inventory, or let it held by a wusel.
     * The object is given by an (vector) index of all currently active objects.
     * If the object is removed from a world position, this will remove the object from the old
     * position.  */
    fn object_set_whereabouts(self: &mut Self, object_index: usize, whereto: Where) {
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
    fn object_destroy(self: &mut Self, object_index: usize) {
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
    fn wusel_identifier_to_index(self: &Self, id: usize) -> Option<usize> {
        self.wusels_on_pos.iter().position(|w| w.wusel.id == id)
    }

    /** Add a wusel to the world.
     * ID is the current wusel count.
     * TODO (2020-11-20) what is about dead wusels and decreasing length? */
    pub fn wusel_new(self: &mut Self, name: String, gender: WuselGender, pos: Position) {
        let id = self.wusels_alltime_count; // almost identifier (for a long time unique)
        let w = Wusel::new(id, name, gender); // new wusel at (pos)

        /* Add wusel to positions, start at (pos). */
        let pos_index = self.position_to_index(pos);
        if pos_index < self.positions.len() {
            self.positions[pos_index].push((Self::CHAR_WUSEL, w.id));
        }

        self.wusels_on_pos.push(WuselOnPosIdx {
            wusel: w,
            position_index: pos_index,
        }); // wusel on position (by index)
            // self.wusels_positions.push(pos_index); // index.
        self.wusels_alltime_count += 1;
    }

    /** Count how many wusels are currently active. */
    pub fn wusel_count(self: &Self) -> usize {
        self.wusels_on_pos.len()
    }

    /** Get the position of the indexed wusel. */
    pub fn wusel_get_position(self: &Self, wusel_index: Option<usize>) -> Option<Position> {
        if let Some(wusel_index) = wusel_index {
            if wusel_index < self.wusels_on_pos.len() {
                Some(self.position_from_index(self.wusels_on_pos[wusel_index].position_index))
            } else {
                None // outside the map.
            }
        } else {
            return None;
        }
    }

    /** Set the position of the indexed wusel to the nearest valid position
     * If the position may land out of the grid, put it to the nearest border. */
    pub fn wusel_set_position(self: &mut Self, wusel_index: usize, pos: Position) {
        if wusel_index < self.wusels_on_pos.len() {
            let id = self.wusels_on_pos[wusel_index].wusel.id;

            /* Update the self.positions. */
            let old_pos_index = self.wusels_on_pos[wusel_index].position_index;

            let new_pos = Position::new(u32::min(pos.x, self.width), u32::min(pos.y, self.depth));
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
    pub fn wusel_get_all_alive(self: &Self) -> Vec<usize> {
        let mut alive: Vec<usize> = vec![];
        for i in 0..self.wusels_on_pos.len() {
            if self.wusels_on_pos[i].wusel.is_alive() {
                alive.push(i);
            }
        }
        return alive;
    }

    /** Get the indices of all wusels, which are currently having no tasks to do. */
    pub fn wusel_get_all_unbusy(self: &Self) -> Vec<usize> {
        let mut unbusy: Vec<usize> = vec![];
        for i in 0..self.wusels_on_pos.len() {
            if self.wusels_on_pos[i].wusel.tasklist.len() < 1 {
                unbusy.push(i);
            }
        }
        return unbusy;
    }

    /** Give an available wusel (by index) a new task. */
    pub fn wusel_assign_task(self: &mut Self, wusel_index: usize, taskb: TaskBuilder) {
        if wusel_index < self.wusels_on_pos.len() {
            /* Task apply wusel[index] as actor. */
            self.wusels_on_pos[wusel_index]
                .wusel
                .add_task(self.clock, taskb);
            log::debug!("task successfully assigned")
        }
    }

    /** Abort an assigned task from an available wusel (by index). */
    pub fn wusel_abort_task(self: &mut Self, wusel_index: usize, task_index: usize) {
        if wusel_index < self.wusels_on_pos.len() {
            /* Remove task. */
            self.wusels_on_pos[wusel_index].wusel.abort_task(task_index);
        }
    }

    /** Print overview of (selected) wusel to std::out.*/
    pub fn wusel_show_overview(self: &Self, wusel_index: usize) {
        /* No wusel is there to show. */
        if wusel_index >= self.wusels_on_pos.len() {
            println!("There is no wusel to show.");
            return;
        }
        println!("{}", self.wusels_on_pos[wusel_index].wusel.show_overview());
    }

    /** Show all relations for a wusel, given by index.
     * Prints directly to std::out. */
    pub fn wusel_show_relations(self: &Self, wusel_index: usize) {
        if wusel_index >= self.wusels_on_pos.len() {
            println!("There is no wusel to show.");
            return;
        }

        let wusel_id = self.wusels_on_pos[wusel_index].wusel.id;

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

        println!("");
    }

    pub fn wusel_get_tasklist(self: &mut Self, wusel_id: usize) -> Vec<String> {
        if let Some(index) = self.wusel_identifier_to_index(wusel_id) {
            self.wusels_on_pos[index].wusel.get_tasklist()
        } else {
            vec![]
        }
    }

    pub fn wusel_get_need_full(self: &mut Self, need: Need) -> u32 {
        Wusel::default_need_full(&need)
    }

    /** Get the wusel's need. */
    pub fn wusel_get_need(self: &mut Self, wusel_id: usize, need: Need) -> u32 {
        if let Some(index) = self.wusel_identifier_to_index(wusel_id) {
            self.wusels_on_pos[index].wusel.get_need(need)
        } else {
            0
        }
    }

    /** Set the wusel's need to a new value. */
    pub fn wusel_set_need(self: &mut Self, wusel_id: usize, need: &Need, new_value: u32) {
        if let Some(index) = self.wusel_identifier_to_index(wusel_id) {
            self.wusels_on_pos[index].wusel.set_need(*need, new_value);
        }
    }

    /** Get the world's current time. */
    pub fn get_time(self: &Self) -> usize {
        self.clock
    }

    /** Increase clock and proceed decay of all things and relations. */
    pub fn tick(self: &mut Self) {
        self.clock += 1;

        /* A new day is over: Forward the day structure to the world. */
        let new_day: bool = self.clock % Self::TICKS_PER_DAY == 0;

        let mut some_busy_wusel: Vec<usize> = vec![];
        let mut new_babies: Vec<(usize, usize, bool)> = vec![];
        let mut dying_wusels: Vec<usize> = vec![];

        /* Decay on every object and living. */
        let mut i: usize = 0;
        for w in self.wusels_on_pos.iter_mut() {
            /* Watch all tasks, remove tasks, which may be aborted or ran out. */
            w.wusel.auto_clean_tasks();

            /* Peek into the ongoing task, and maybe proceed them.
             * This may lead to remove the done task. */
            if w.wusel.tasklist.len() > 0 {
                some_busy_wusel.push(i);
            } else {
                /* Wusel is currently not busy. => maybe apply an idle/auto task. */
            }

            /* If pregnant: Maybe push out the child => Failure, Early or too late. */
            if let Some((father, pregnancy_days)) = w.wusel.pregnancy {
                let maybe_now: u8 = rand::random::<u8>() % 100;
                let possibility: u8 = match pregnancy_days {
                    0 => 90,
                    1 => 75,
                    _ => 10,
                };
                if (0u8..possibility).contains(&maybe_now) {
                    log::debug!("Pop the baby!");
                    let gender = rand::random::<bool>();
                    new_babies.push((w.wusel.id, father, gender));
                }
            }

            let alive = w.wusel.wusel_tick(new_day);

            /* The wusel just died. Remove if from active wusels later. */
            if !alive {
                dying_wusels.push(i);
            }

            i += 1;
        }

        /* Execute ongoing tasks, unmutable wusel context.. */
        for w in some_busy_wusel.iter() {
            if let Some(t) = self.wusels_on_pos[*w].wusel.peek_ongoing_task() {
                /* Decide how to progress the command. */
                let u = (*t).clone();
                self.proceed(u);
            }
        }

        for _ in self.relations.iter() { /* Decay of relations over time. */ }

        /* Command further name giving and attention from the player. */
        for baby in new_babies.iter() {
            log::debug!(
                "New parents {}  and {}: It is a {} ",
                baby.0,
                baby.1,
                if baby.2 { "Girl" } else { " Boy" }
            );
        }
    }

    /** Proceed the task in this world. */
    fn proceed(self: &mut World, task: Task) {
        /* World proceeds task. */

        let actor_id = task.active_actor_id;
        let actor_index = self.wusel_identifier_to_index(actor_id);

        if actor_index == None {
            return; // abort, because actor unavailable
        }

        let actor_index = actor_index.unwrap();

        let start_time = match task.started {
            true => task.start_time,
            false => {
                /* Notify the start of the task (for the wusel). */
                self.wusels_on_pos[actor_id]
                    .wusel
                    .start_ongoing_task(self.clock);

                self.clock // starting now
            }
        };

        /* Decide what to do, and if the task case done a step. */
        let succeeded = match task.passive_part {
            TaskTag::WaitLike => {
                log::debug!("{}", task.name);
                true
            }
            TaskTag::BeMetFrom(other_id) => {
                let other_index = self.wusel_identifier_to_index(other_id);

                /* Other wusel needs also to exist or still wants to meet.
                 * Otherwise pop. */

                /* Meeting party is valid, check their ongoing task. */
                if let Some(other_index) = other_index {
                    match self.wusels_on_pos[other_index].wusel.peek_ongoing_task() {
                        /* => Proceed, since the other party is doing nothing, so no meeting. */
                        None => true,

                        /* Other party is busy. */
                        Some(t) => match t.passive_part {
                            /* => Do not end (proceed), since the other party is still meeting with this actor. */
                            TaskTag::MeetWith(id, _nice, _love) if id == actor_id => false,

                            /* => proceed task, other party is busy with sth else. */
                            _ => true,
                        },
                    }
                } else {
                    /* => proceed, since the other party was invalid. */
                    true
                }
            }
            TaskTag::MeetWith(other_id, nice, romantically) => {
                let other_index = self.wusel_identifier_to_index(other_id);

                /* Other wusel needs also to exist. */
                if other_index == None {
                    self.wusels_on_pos[actor_index].wusel.pop_ongoing_task();
                    return; // task can not be done, without target.
                }

                let other_index = other_index.unwrap();

                /* Check all preconditions, maybe solve one and maybe do the actually meeting.
                 * 0, when they met, like the C-ish "OK".
                 * 1, when the actor walked.
                 * 2, when the actual knocking was just applied.
                 * 3, when the knocking was done, but the passive is still busy. */
                let meeting_result =
                    self.let_two_wusels_meet(actor_index, other_index, nice, romantically);

                /* On Final Success with own step,
                 * also let the BeMetFrom() succeed. */

                match meeting_result {
                    // waiting, but don't wait too long.
                    Self::MEET_RESULT_WAITED => {
                        if self.clock - start_time >= Task::PATIENCE_TO_MEET {
                            self.wusels_on_pos[actor_index].wusel.pop_ongoing_task();
                        }
                        false // => do not notify succession
                    }

                    /* They met and the task is over. */
                    Self::MEET_RESULT_OK => true, // => notify process
                    _ => false, // => no process (FOLLOWED, KNOCKED or an unexpected)
                }
            }
            TaskTag::MoveToPos(pos) => {
                /* Let the wusel walk; check if they stopped. */
                let stopped: bool = self.let_wusel_walk_to_position(actor_index, pos);

                stopped // true == stop == success.
            }
            TaskTag::UseObject(object_id, action_id) => {
                // TODO: get index for the given object ID.
                let object_index = self
                    .objects
                    .iter()
                    .position(|wo| wo.object.object_id == object_id);

                // TODO: get index for the given action ID.
                let action_index = if action_id >= self.actions.len() {
                    None
                } else {
                    Some(action_id)
                };

                if object_index == None || action_index == None {
                    log::warn!(
                        "Object[{:?}] or Action[{}] could not be found.",
                        object_id,
                        action_id
                    );
                    true // proceed to next action.
                } else {
                    let object_index = object_index.unwrap(); // TODO
                    let action_index = action_index.unwrap(); // TODO
                    self.let_wusel_use_object(actor_index, object_index, action_index)
                }
            }
        };

        /* Notify the task succeeded to do a step. */
        if succeeded {
            self.wusels_on_pos[actor_index]
                .wusel
                .notify_ongoing_succeeded();
        }
    }

    const MEET_RESULT_ERROR: i8 = -1; //  meeting error.
    const MEET_RESULT_OK: i8 = 0; //  When they met, like the C-ish "OK".
    const MEET_RESULT_FOLLOWED: i8 = 1; //  When the actor walked, they might not have met yet.
    const MEET_RESULT_KNOCKED: i8 = 2; //  When the actual knocking was just applied, they know both of the meeting, but that may come next.
    const MEET_RESULT_WAITED: i8 = 3; //  When the knocking was done, but the passive is still busy, they actually have not met like intended.

    /** Arrange the meeting of two wusels.
     * They must both exist.
     * They must be close to each other (neighbour fields or shared desk/bench...).
     * If not, the active wusels walk to the passive wusel.
     * The passive wusel must be free or ready to receive the active wusel's approaches.
     * If not, let the active wait for longer and add the request to the passive wusel.
     * The outcome may be influenced by random and the communication abilities the active member.
     *
     * The output is a number, presenting what the active wusel has done.
     * 0: When they met, like the C-ish "OK".
     * 1: When the actor walked, they might not have met yet.
     * 2: When the actual knocking was just applied, they know both of the meeting, but that may come next.
     * 3: When the knocking was done, but the passive is still busy, they actually have not met like intended.
     *
     * #Return, if they actually met (true), or only preconditions needed to be satisfied (false). */
    fn let_two_wusels_meet(
        self: &mut Self,
        active_index: usize,
        passive_index: usize,
        intention_good: bool,
        romantically: bool,
    ) -> i8 {
        log::debug!(
            "Meet with {}, nice: {}.",
            self.wusels_on_pos[passive_index].wusel.show(),
            intention_good
        );

        /* If not close to the other wusel, use this step to get closer,
         * return as not yet ready. */
        let pos_o = self.wusel_get_position(Some(passive_index));

        if pos_o == None {
            return Self::MEET_RESULT_ERROR; // No position.
        }

        let pos_o = pos_o.unwrap();

        log::debug!("Meet at {:?}", pos_o);

        /* If the actor is close enough, do the next steps. */
        let following = self.let_wusel_walk_to_position_if_not_close(active_index, pos_o, 2.0);

        /* Just followed. */
        if !following {
            return Self::MEET_RESULT_FOLLOWED;
        }

        let active_id = self.wusels_on_pos[active_index].wusel.id;
        let passive_id = self.wusels_on_pos[passive_index].wusel.id;

        /* Get the passive wusel's current task.
         * If it is being met by the active, succeed a step with the meeting,
         * otherwise see below. */
        let passives_ongoing_tasktag: Option<TaskTag> =
            if let Some(t) = self.wusels_on_pos[passive_index].wusel.peek_ongoing_task() {
                Some(t.passive_part.clone())
            } else {
                None
            };

        let active_is_met = &TaskTag::BeMetFrom(active_id);

        let handshake_okay = match &passives_ongoing_tasktag {
            Some(tag) if *tag == *active_is_met => true,
            _ => false,
        };

        if handshake_okay {
            let performance: bool; // how well is the communication

            performance = true;
            // random influence of 10%
            // current value and intention
            // communication ability

            /* Update the relation between active and passive. */
            self.wusel_update_relations(
                active_id,
                passive_id,
                intention_good && performance,
                romantically && performance,
            );

            return Self::MEET_RESULT_OK; // they actually met.
        }

        /* Check, if the passive is already waiting (in tasklist). */
        let passive_is_waiting = self.wusels_on_pos[passive_index]
            .wusel
            .has_task_with(active_is_met);

        /* Check if they both want an (actively) Meeting each other. */
        let mutuall_meeting_as_actives = match &passives_ongoing_tasktag {
            Some(TaskTag::MeetWith(id, _, _)) if *id == active_id => true,
            _ => false,
        };

        /* They are blocking each other by waiting.
         * A: "I want to talk with you, but wait until you're done with your task."
         * B: "I also want to talk with you, but I wait for you!" */
        if mutuall_meeting_as_actives {
            /* If one of them are already waiting for the other, it's settled.
             * Just get the waiting (to be met) to active task. */

            /* This active meeter was earlier.
             * This passive meeter was earlier.
             * Otherwise some invalid index. */

            let already_waiting_index = match 0 {
                _p if passive_is_waiting => passive_index,
                _a if self.wusels_on_pos[active_index]
                    .wusel
                    .has_task_with(&TaskTag::BeMetFrom(passive_id)) =>
                {
                    active_index
                }
                _ => self.wusels_on_pos.len(),
            };

            /* Move already waiting task to active tasks. */
            if already_waiting_index < self.wusels_on_pos.len() {
                /* What happens:
                 * A: [Talk B, Task A2, Task A3]
                 * B: [Talk A, Task B3, Listen A] // B already knows.
                 * ----
                 * A: [Talk B, Task A2, Task A3]
                 * B: [Listen A, Talk A, Task B2, Task B3] // let B listen first.
                 */

                let mut i = self.wusels_on_pos[already_waiting_index]
                    .wusel
                    .tasklist
                    .len();
                while i > 0 {
                    i -= 1;
                    if self.wusels_on_pos[already_waiting_index].wusel.tasklist[i].passive_part
                        == *active_is_met
                    {
                        let met_task = self.wusels_on_pos[already_waiting_index]
                            .wusel
                            .tasklist
                            .remove(i);
                        self.wusels_on_pos[already_waiting_index]
                            .wusel
                            .tasklist
                            .push(met_task); // append to back (ongoing)
                        break;
                    }
                }
                return Self::MEET_RESULT_KNOCKED; // even if it might be knocked before.
            }

            /* Non of them requested anything before.
             * Decide it on communication skill.
             * On tie, let this active be the first one.
             * (No waiting-to-be-met needs to be deleted.) */

            let skill = Ability::COMMUNICATION;
            let c0 = self.wusels_on_pos[active_index].wusel.get_ability(&skill);
            let c1 = self.wusels_on_pos[passive_index].wusel.get_ability(&skill);

            let (more_active, more_passive) = match c0 {
                better if better > c1 => (active_index, passive_index),
                worse if worse < c1 => (passive_index, active_index),
                _tie if active_index < passive_index => (active_index, passive_index),
                _ => (passive_index, active_index),
            };

            self.wusel_assign_task(
                more_passive,
                TaskBuilder::be_met_from(more_active)
                    .set_name(format!("Be met by {}", more_active)),
            );

            return Self::MEET_RESULT_KNOCKED;
        }

        /* Else, just notify them, if not yet done,
         * I am there and wait for them to be ready. */
        if !passive_is_waiting {
            /* Tell passive to be ready for active. */
            self.wusel_assign_task(passive_index, TaskBuilder::be_met_from(active_id));
            return Self::MEET_RESULT_KNOCKED;
        }

        /* If the passive target is not yet ready to talk, wait.  */
        return Self::MEET_RESULT_WAITED;
    }

    const TASK_HOLD: bool = false;
    const TASK_PROCEED: bool = true;

    /** Let a wusel use an object.
     *
     * If the object is held by the wusel themselves, use it directly.
     * If the object is placed in the world, go to the object.
     * If the object is held by an accessable inventory, find the inventory and get the object (hold it).
     * If the object is held by another wusel, it cannot be done.
     *
     * Using the object may change the needs and abilities of the wusel (given by an preset).
     * Using the object may also consume the object.
     *
     * Returns if an interaction happend (true) or not (false).
     *
     * Examples.
     * - Wusel `consumes` some bread they hold (bread held).
     * - Wusel `consumes` some bread on the desk (bread placed).
     * - Wusel `takes` a shower (shower placed).
     * - Wusel cannot `consume` some bread held by another wusel (shower placed).
     */
    fn let_wusel_use_object(
        self: &mut Self,
        wusel_index: usize,
        object_index: usize,
        action_index: usize,
    ) -> bool {
        /* Invalid wusel index. */
        if wusel_index >= self.wusels_on_pos.len() {
            return false;
        }

        let wusel_id = self.wusels_on_pos[wusel_index].wusel.id;

        /* Invalid object index. */
        if object_index >= self.objects.len() {
            log::warn!("No such object.");
            return false;
        }

        /* Check where the object is.
         * If AtPosition(pos) => go to position (pos).
         * If StoredIn(storage) => get from storage.
         * If HeldBy(holder_id) => holder_id ==~ wusel_id => ok, else abort. */
        let obj_pos = self.object_index_get_position(object_index);

        /* If not close to object, go to it. */
        let close_enough = if let Some(obj_pos) = obj_pos {
            log::debug!("Go to object's position.");
            self.let_wusel_walk_to_position_if_not_close(
                wusel_index,
                obj_pos, // current object position.
                1.2,     // max distance.
            )
        } else {
            false
        };

        let obj_pos = obj_pos.unwrap();
        let obj_pos_index = self.position_to_index(obj_pos);

        if !close_enough {
            return false;
        }

        let obj_where = &self.objects[object_index].position;
        let obj_id = self.objects[object_index].object.object_id;

        /* Invalid action index. */
        if action_index >= self.actions.len() {
            log::warn!("No such action.");
            return false;
        }

        log::debug!(
            "Used object ({:?} on {:?}).",
            self.actions[action_index],
            self.objects[object_index]
        );

        /* Get the effect of interacting with the object. */
        let effect = self.actions_effects.iter().find(
            |((obj_type, obj_subtype, _), act_id, _effect_str, _effect_vec)| {
                *obj_type == obj_id.0 && *obj_subtype == obj_id.1 && *act_id == action_index
            },
        );

        if let Some(effect) = effect {
            log::debug!("Using the object has the following effect: {:?}", effect);
            let (_, _, _, effect_vec) = effect;
            for e in effect_vec {
                log::debug!("- Apply effect: {:?}", e);
                self.wusels_on_pos[wusel_index].wusel.mod_need(e.0, e.1);
            }
        }

        /* Do the actual action. */
        return match self.actions[action_index].as_ref() {
            "View" => {
                log::info!("Just view.");
                // TODO can u view sth, if it's held by another wusel?
                Self::TASK_PROCEED
            }
            "Take" => {
                if let Where::AtPosition(_) | Where::StoredIn(_) = obj_where {
                    log::info!("Get it, if possible.");
                    self.object_set_whereabouts(object_index, Where::HeldBy(wusel_id));
                    return Self::TASK_PROCEED;
                }
                log::warn!("Item is already hold, just look and stop.");
                Self::TASK_PROCEED // if already held, cannot be held, but just stop to do so.
            }
            "Drop" => {
                if let Where::HeldBy(holder_id) = *obj_where {
                    if holder_id == wusel_id {
                        log::info!("Drop it, if held by wusel themself.");
                        self.object_set_whereabouts(object_index, Where::AtPosition(obj_pos_index)); // == wusel_pos, as pos of all containers
                        log::debug!("Object placed, somewhere.");
                        return Self::TASK_PROCEED;
                    }
                }
                Self::TASK_PROCEED // if not held, it cannot be dropped, but the wusel will be done, to drop the object.
            }
            "Consume" => {
                let consumable = self.objects[object_index].object.consumable;
                if consumable != None {
                    let left_over = consumable.unwrap();
                    log::debug!("Consume a part of the consumable object.");

                    if left_over <= 1usize {
                        self.object_destroy(object_index); // delete from world.
                        log::debug!("Consumable Object fully consumed.");
                        return Self::TASK_PROCEED;
                    }
                    self.objects[object_index].object.consumable = Some(left_over - 1);

                    // return Self::TASK_PROCEED;
                    return Self::TASK_HOLD; // ddbug.
                }
                log::warn!("Tried to consume something  unconsumable");
                Self::TASK_HOLD // if not held, it cannot be dropped, but the wusel will be done, to drop the object.
            }
            _ => {
                log::info!("Undefined action?");
                Self::TASK_HOLD
            }
        };
    }

    /** Let the wusel walk to a position, if they are not close.
     * Return true, if they are close enough. */
    fn let_wusel_walk_to_position_if_not_close(
        self: &mut Self,
        wusel_index: usize,
        goal: Position,
        max_distance: f32,
    ) -> bool {
        let wpos = self.wusel_get_position(Some(wusel_index));

        if wpos == None {
            return false; // wusel itself has no position.
        }

        let wpos = wpos.unwrap();

        if wpos.distance_to(&goal) > max_distance {
            self.let_wusel_walk_to_position(wusel_index, goal);
            false // just walked
        } else {
            true // reached goal.
        }
    }

    /** Let the wusel walk to a position.
     * If the path is already calculated, let it walk the pre-calculated path.
     * If not, calculate a new path.
     * If an obstacle occurs, recalculate the path and continue walking.
     * If the goal, is reached, the walk is done.
     *
     * #Return, if wusel has stopped walking / is on goal (true), otherwise true, if they are still walking. */
    fn let_wusel_walk_to_position(self: &mut Self, wusel_index: usize, goal: Position) -> bool {
        let pos = self.wusel_get_position(Some(wusel_index));

        if pos == None {
            return true; // couldn't move => stopped walking.
        }

        let pos = pos.unwrap();

        /* Check if the goal is already reached. */
        if pos.x == goal.x && pos.y == goal.y {
            log::info!("Reached Goal ({},{}).", goal.x, goal.y);
            return true; // stopped walking.
        }

        log::info!("Move to Goal {:?}.", goal);

        /* Check, if the pre-calculated path is blocked. */
        if false { /* Abort the pre-calculated, but blocked path. */ }

        /* Check, if the path is (still) pre-calculated. */
        if true {
            /* Walk the path. */
            // XXX easy placeholder walking, ignoring all obstacles.

            /* Get the current positions neighbours. */
            let neighbours = self.position_get_all_neighbours(pos);

            if neighbours.len() < 1 {
                log::info!("Wusel cannot move, it's enclosed, wait forever");
                return true;
            }

            let goal: Position = Position::new(goal.x, goal.y);
            let mut closest: Position = neighbours[0];
            let mut closest_distance: f32 = f32::MAX;

            /* Find closest neighbour to goal. */
            for p in neighbours.iter() {
                let distance = goal.distance_to(&p);

                if distance < closest_distance {
                    closest = *p;
                    closest_distance = distance;
                }
            }

            /* move to closest position. */
            self.wusel_set_position(wusel_index, closest);
            return false; // still walking.
        } else {
            /* Calculate the path and go it next time. */
            log::info!("Calculate the path to {:?}", goal);
            return false; // still walking.
        }
    }

    /** Update the relation of two wusels, given by their ID. */
    pub fn wusel_update_relations(
        self: &mut Self,
        wusel0_id: usize,
        wusel1_id: usize,
        nice: bool,
        romantic: bool,
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
        let rel = self.relations.entry(key).or_insert_with(Relation::new);

        (*rel).friendship += change;

        if romantic {
            (*rel).romance += change;
        }
    }
}

/** Way in the world. */
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Way {
    NW,
    N,
    NE,
    W,
    E,
    SW,
    S,
    SE,
}

impl Way {
    pub const NEIGHBOURING: [Self; 8] = [
        Self::NW,
        Self::N,
        Self::NE, // north
        Self::W,
        Self::E, // same longitude
        Self::SW,
        Self::S,
        Self::SE, // south
    ];
    /** Get the offsets to walk, to get to the way point. */
    pub fn as_direction_tuple(self: &Self) -> (i8, i8) {
        match self {
            /* Go north. */
            Way::NW => return (-1, 1),
            Way::N => return (0, 1),
            Way::NE => return (1, 1),

            /* Stay on longitude. */
            Way::W => return (-1, 0),
            Way::E => return (1, 0),

            /* Go south. */
            Way::SW => return (-1, -1),
            Way::S => return (0, -1),
            Way::SE => return (1, -1),
        }
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

/** Identifier type (tuple) for an object. */
type ObjectIdentifer = (ObjectType, ObjectSubtype, usize);
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

/** Where the object is stored / placed. */
#[derive(Debug, Clone, PartialEq)]
pub enum Where {
    AtPosition(usize),         // position index
    StoredIn(ObjectIdentifer), // storage ID (object ID of the storage)
    HeldBy(usize),             // held by a wusel (index)
}

/** A Recipe is a list of required abilities, consumables or positions
 * to create a certain product after a certain time.
 * Recipe: [ components, Workstation ] + Time => Product. */
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
    need_change: std::collections::HashMap<Need, i16>,
}

/** Pair of Wusels which may have a relation. */
#[derive(Clone, Debug)]
pub struct Relation {
    officially: String, // officially known state (Friends, Spouse, etc..)

    friendship: i32, // shared friendship between both.

    romance: i32, // shared romance between both

    kindred_distance: i32, // blood relation (distance)
}

impl Relation {
    /** Create a new empty relationship for just met strangers. */
    fn new() -> Self {
        Self {
            officially: String::from("Strangers"),
            friendship: 0,
            romance: 0,
            kindred_distance: -1,
        }
    }

    pub const RELATION_FRIEND: char = '\u{2639}'; // smiley
    pub const RELATION_ROMANCE: char = '\u{2661}'; // heart

    /** Print this relation to a String. */
    pub fn show(self: &Self) -> String {
        format!(
            "'{official}' {rel_f}{friendly} {rel_r}{romance}{kinship}",
            official = self.officially,
            rel_f = Self::RELATION_FRIEND,
            friendly = self.friendship,
            rel_r = Self::RELATION_ROMANCE,
            romance = self.romance,
            kinship = match self.kindred_distance {
                -1 => "",
                0 => " Self?",
                1 => " Siblings|Parents|Kids",
                _ => "Related",
            }
        )
    }
}

/** Life state of a Wusel.
 * All but alive leads to a not living state, though a ghost may wander and interact. */
#[derive(Copy, Clone, PartialEq)]
pub enum Life {
    ALIVE,
    #[allow(unused)]
    DEAD,
    #[allow(unused)]
    GHOST,
}

/** A need, the Wusel needs to satisfy to survive. */
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Need {
    WATER,
    FOOD,
    SLEEP,
    LOVE,
    FUN,
    WARMTH,
    HEALTH,
}

impl Need {
    /** Custom iterable values. */
    pub const VALUES: [Self; 7] = [
        Self::WATER,
        Self::FOOD,
        Self::SLEEP,
        Self::LOVE,
        Self::FUN,
        Self::WARMTH,
        Self::HEALTH,
    ];

    const DEFAULT_NEED_DECAY_PER_MINUTE: [u32; 7] = [
        1, 1, 1, 1, 1, 0, /*warmth*/
        0, /*health*/ // by outer sources
    ];

    pub fn name(self: &Self) -> &str {
        return match self {
            Self::WATER => "water",
            Self::FOOD => "food",
            Self::WARMTH => "warmth",
            Self::SLEEP => "sleep",
            Self::HEALTH => "health",
            Self::LOVE => "love",
            Self::FUN => "fun",
        };
    }

    pub fn get_default_decay(self: &Self) -> u32 {
        for i in 0..Self::VALUES.len() {
            if self == &Self::VALUES[i] {
                return Self::DEFAULT_NEED_DECAY_PER_MINUTE[i];
            }
        }
        return 0; // default: no decay.
    }
}

/** An ability, the Wusel can learn to improve their lifestyle. */
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Ability {
    COOKING,
    COMMUNICATION,
    FITNESS,
    FINESSE,
}

impl Ability {
    fn name(self: &Self) -> &str {
        return match self {
            Self::COOKING => "cooking",
            Self::COMMUNICATION => "communication",
            Self::FITNESS => "fitness",
            Self::FINESSE => "finesse",
        };
    }
}

/** TaskBuilder, to create a Task for a Wusel.
 * Name, Target, duration and conditions are set with the builder. */
#[derive(Debug, Clone)]
pub struct TaskBuilder {
    name: String,
    duration: usize,
    passive_part: TaskTag,
}

impl TaskBuilder {
    /** Create a new Task Builder. */
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            duration: 0,
            passive_part: TaskTag::WaitLike,
        }
    }

    /** Create a new Task Builder, preset for moving. */
    pub fn move_to(pos: Position) -> Self {
        Self {
            name: "Moving".to_string(),
            duration: 1,
            passive_part: TaskTag::MoveToPos(pos),
        }
    }

    /** Create a new Task Builder, preset for meeting. */
    pub fn meet_with(passive: usize, friendly: bool, romantically: bool) -> Self {
        Self {
            name: "Meeting".to_string(),
            duration: 1,
            passive_part: TaskTag::MeetWith(passive, friendly, romantically),
        }
    }

    /** Create a new Task Builder, preset for working on a workbench. */
    pub fn use_object(object_id: ObjectIdentifer, action_id: usize) -> Self {
        Self {
            name: format!("Use[{}] Object[{:?}]", action_id, object_id),
            duration: 1,
            passive_part: TaskTag::UseObject(object_id, action_id),
        }
    }

    /** Create a new Task Builder, preset for being met. */
    pub fn be_met_from(active: usize) -> Self {
        Self {
            name: "Being Met".to_string(),
            duration: 1,
            passive_part: TaskTag::BeMetFrom(active),
        }
    }

    /** Get the name of the future task or all then created tasks. */
    #[allow(dead_code)]
    pub fn get_name(self: &Self) -> String {
        self.name.clone()
    }

    /** Get the duration of the future task or all then created tasks. */
    #[allow(dead_code)]
    pub fn get_duration(self: &Self) -> usize {
        self.duration
    }

    /** Rename the task builder in the Task Builder. */
    pub fn set_name(mut self, name: String) -> Self {
        self.name = name;
        return self;
    }

    /** Set the duration in the Task Builder. */
    pub fn set_duration(mut self, time: usize) -> Self {
        self.duration = time;
        return self;
    }

    /** Set the duration in the passive part. */
    #[allow(dead_code)]
    pub fn set_passive_part(mut self, passive: TaskTag) -> Self {
        self.passive_part = passive;
        return self;
    }

    /** Create a new Task from the builder for the requesting [actor](Wusel). */
    fn assign(self, start_time: usize, actor: &Wusel) -> Task {
        Task {
            name: self.name,
            started: false,
            start_time: start_time,
            duration: self.duration,
            done_steps: 0,

            active_actor_id: actor.id,
            passive_part: self.passive_part,
        }
    }
}

/** Task, a Wusel can do. */
#[derive(Clone)]
pub struct Task {
    name: String,
    started: bool,
    start_time: usize,
    duration: usize,
    done_steps: usize,

    active_actor_id: usize, // wusel ID.
    passive_part: TaskTag,  // position | object-to-be | object | wusel | nothing.
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskTag {
    WaitLike,
    MoveToPos(Position),

    UseObject(ObjectIdentifer, usize), // object_id, and action_id

    MeetWith(usize, bool, bool), // commute with another wusel (ID)
    BeMetFrom(usize),            // be met by another wusel (ID)
}

impl Task {
    const PATIENCE_TO_MEET: usize = 20; // TODO

    /** Get the approximately rest time (in ticks), this task needs. */
    fn get_rest_time(self: &Self) -> usize {
        self.duration - self.done_steps
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WuselGender {
    Female,
    Male,
}

/** Wusel.
 * Bundle of information on a certain position and abilities. */
pub struct Wusel {
    id: usize,

    /* Name */
    name: String,

    gender: WuselGender, // WuselGender::Female => able to bear children, male => able to inject children
    pregnancy: Option<(usize, u8)>, // optional pregnancy with father's ID and remaining days.

    life: Life,      // alive | dead | ghost
    lived_days: u32, // last lived day.

    needs: Vec<(Need, u32)>,

    /* Abilities. */
    abilities: Vec<(Ability, u32)>, // ability levels.

    /* List of tasks. */
    tasklist: Vec<Task>,
}

impl Wusel {
    /** From full to 0, how many ticks does it need, when it's only normally decreasing. */
    const WUSEL_FULL_NEEDS: [(Need, u32); 7] = [
        (Need::WATER, (24 * 60 * 2) * 3),   // 3 days until dehydrate.
        (Need::FOOD, (24 * 60 * 2) * 7),    // a week until starve.
        (Need::WARMTH, (8 * 60 * 2)),       // 8h until freeze to death.
        (Need::SLEEP, (24 * 60 * 2) * 7),   // a week until suffer from sleep loss.
        (Need::HEALTH, (24 * 60 * 2) * 14), // 2 weeks until die of illness.
        (Need::LOVE, (24 * 60 * 2) * 14),   // 2 weeks until become lonely.
        (Need::FUN, (24 * 60 * 2) * 14),    // 2 weeks until unmotivated and depressive.
    ];

    /** Create a new Wusel with name. */
    fn new(id: usize, name: String, gender: WuselGender) -> Self {
        let mut new = Self {
            id,
            name,

            gender,
            pregnancy: None,

            life: Life::ALIVE,
            lived_days: 0,

            needs: vec![],
            abilities: vec![],
            tasklist: vec![],
        };

        /* Initiate all known needs to FULL. */
        for (n, full) in &Self::WUSEL_FULL_NEEDS {
            new.needs.push((*n, *full));
        }

        return new;
    }

    fn get_id(self: &Self) -> usize {
        return self.id;
    }

    /** Tick one unit.
     * Reduce the satisfaction of each needs by default values.
     * Maybe let it age one day.
     * @return if the wusel is still alive in the end. */
    fn wusel_tick(self: &mut Self, add_day: bool) -> bool {
        /* If in action, need changes may also apply, eg. eating. */
        // self.act(); // proceed on task, if tasklist is providing one.

        /* Decrease every value by DEFAULT_NEED_DECAY_PER_MINUTE * minutes. */
        for i in 0..self.needs.len() {
            let (n, v) = self.needs[i];
            let decay = n.get_default_decay();

            // XXX when SICK: decay health
            // XXX when IN COLD: decay warmth

            self.needs[i] = (n, if v < decay { 0 } else { v - decay });
        }

        /* Add a new day. */
        if add_day {
            self.add_new_day()
        }

        return self.is_alive();
    }

    /** Count a new day to the lived lived. */
    fn add_new_day(self: &mut Self) {
        if self.is_alive() {
            /* Age one day. */
            self.lived_days += 1;

            /* Decay all abilities by one point. */
            for i in 0..self.abilities.len() {
                let (abi, val) = self.abilities[i];
                self.abilities[i] = (abi, val - 1);
            }

            /* If pregnant, reduce time until arrival. */
            self.pregnancy = match self.pregnancy {
                Some((father, days)) if days > 0 => Some((father, days - 1)),
                _ => self.pregnancy,
            }
        }
    }

    /** Check, if this Wusel is alive. */
    fn is_alive(self: &Self) -> bool {
        return match self.life {
            Life::ALIVE => true, // all but alive are not alive.
            _ => false,
        };
    }

    /** Get name of the Wusel. */
    fn get_name(self: &Self) -> String {
        self.name.clone()
    }

    /** Show the name, gender and age. */
    fn show(self: &Self) -> String {
        /* The name */
        let mut string = self.name.clone();
        string.push(' ');

        /* Gender */
        string.push_str(if self.gender == WuselGender::Female {
            "\u{2640}"
        } else {
            "\u{2642}"
        });

        /* Birth tick. */
        string.push_str(" (");

        /* Show life and age. */
        match self.life {
            Life::ALIVE => println!(""),
            Life::DEAD => println!("dead, "),
            Life::GHOST => println!("ghost, "),
        }
        string.push_str(&self.lived_days.to_string());
        string.push_str("d)"); // days

        return string;
    }

    /** Show collected data. */
    fn show_overview(self: &Self) -> String {
        let mut s = format!("==={:=<40}\n", "");

        s += &format!("  {}\n", self.show());

        /* Show needs. */
        s += &format!("---{:-<40}\n", " NEEDS: ");

        /* Show abilities. */
        s += &format!("---{:-<40}\n", " ABILITIES: ");
        s += &self.show_abilities();

        /* Show relations. */
        // TODO (2020-11-16) show relations.
        s += &format!("{:_<43}\n", "");

        return s;
    }

    /** Print the tasklist (as queue). */
    fn get_tasklist(self: &Self) -> Vec<String> {
        return self
            .tasklist
            .iter()
            .map(|task| task.name.to_string())
            .collect();
    }

    /** Print the Wusel's abilities. */
    fn show_abilities(self: &Self) -> String {
        let mut s = String::new();
        for (ability, value) in &self.abilities {
            s += &format!(
                "{a:>15} {v:5} {bar:*<v$}",
                a = ability.name(),
                v = *value as usize,
                bar = ""
            );
        }
        return s;
    }

    /** Get the default need value. */
    fn default_need_full(need: &Need) -> u32 {
        for (n, v) in Self::WUSEL_FULL_NEEDS.iter() {
            if n == need {
                return *v;
            }
        }
        return 0; // else return 0, if not an default need.
    }

    /** Get the value for a need.
     * This may append the needs with a new default value, if the need is not
     * yet inserted. */
    fn get_need(self: &mut Self, need: Need) -> u32 {
        /* Find the need and return the value. */
        let size: usize = self.needs.len();
        for i in 0..(size) {
            let (n, v) = self.needs[i];
            if n == need {
                return v;
            } // return assigned value
        }
        /* If not found: Append with default Need value. */
        let default: u32 = 0;
        self.needs.push((need, default));
        return default;
    }

    /** Set the value for a need.
     * This may append the needs with the new given value. */
    fn set_need(self: &mut Self, need: Need, new_value: u32) {
        /* Find the need and change the value. */
        let size: usize = self.needs.len();
        for i in 0..(size) {
            let (n, _) = self.needs[i];
            if n == need {
                self.needs[i] = (n, new_value); // update the value.
                return; // done
            }
        }
        /* If not found: Append with default Need value. */
        self.needs.push((need, new_value));
    }

    /** Change the value for a need relatively.
     * This may create a new value, with default input changed by the change value.
     * @return the new value.*/
    fn mod_need(self: &mut Self, need: Need, change_value: i16) -> u32 {
        let current = self.get_need(need) as i64; // get current value (or default)

        let mut changed = current + (change_value as i64);
        if changed < 0 {
            changed = 0; // don't go below 0.
        }

        self.set_need(need, changed as u32); // change the value.

        return self.get_need(need); // return final need's value.
    }

    /** Improve the given ability by one point. */
    fn improve(self: &mut Self, ability: &Ability) {
        /* Improve the given ability. */
        for i in 0..(self.abilities.len()) {
            let (a, v) = self.abilities[i];
            if *ability == a {
                self.abilities[i] = (a, v + 1);
                return;
            }
        }
        /* If the given ability is not yet learned, add it to the abilities. */
        self.abilities.push((*ability, 1));
    }

    /** Get the value for a requested ability. */
    pub fn get_ability(self: &Self, ability: &Ability) -> u32 {
        for (a, v) in self.abilities.iter() {
            if a == ability {
                return *v;
            }
        }
        0
    }

    /** Append a new task to the task list. */
    fn add_task(self: &mut Self, init_time: usize, task_builder: TaskBuilder) {
        /* Task apply self as actor. */
        let task = task_builder.assign(init_time, self);
        self.tasklist.insert(0, task); // revert queue
    }

    /** Abort a task in the task list. */
    fn abort_task(self: &mut Self, index: usize) {
        if index < self.tasklist.len() {
            self.tasklist.remove(index);
        }
        /* Otherwise no task is aborted. */
    }

    /** Clean task list.
     * Remove ongoing tasks if there are no steps left. */
    fn auto_clean_tasks(self: &mut Self) {
        /* Remove ongoing task, if it is done. */
        while let Some(ongoing) = self.peek_ongoing_task() {
            if ongoing.get_rest_time() < 1 {
                self.tasklist.pop();
            } else {
                break; // ongoing task not yet done.
            }
        }
    }

    /** Check, if this wusel has a task with the requested passive tag. */
    fn has_task_with(self: &Self, task_tag: &TaskTag) -> bool {
        for t in self.tasklist.iter() {
            if t.passive_part == *task_tag {
                return true;
            }
        }
        return false;
    }

    /** Peek the ongoing task. */
    fn peek_ongoing_task(self: &Self) -> Option<&Task> {
        self.tasklist.last()
    }

    /** Start the ongoing task.
     * This may set the started flag to true, if not yet set and maybe
     * updates the starting time. */
    fn start_ongoing_task(self: &mut Self, start_time: usize) {
        if let Some(t) = self.tasklist.last_mut() {
            if !t.started {
                t.started = true;
                t.start_time = start_time;
            }
        }
    }

    /** Notify the ongoing task, that its done steps are increased
     * This increases the optional ongoing tasks [done_steps](Task.done_steps). */
    fn notify_ongoing_succeeded(self: &mut Self) {
        if let Some(ongoing) = self.tasklist.last_mut() {
            ongoing.done_steps += 1;
        }
    }

    /** Pop the ongoing task (queue reversed). */
    fn pop_ongoing_task(self: &mut Self) -> Option<Task> {
        self.tasklist.pop()
    }

    /** Check, if the wusel is pregnant. */
    #[allow(dead_code)]
    pub fn is_pregnant(self: &Self) -> bool {
        return self.pregnancy != None;
    }

    /** Get the remaining days of an possible Pregnancy. */
    #[allow(dead_code)]
    pub fn get_remaining_pregnancy_days(self: &Self) -> Option<u8> {
        if let Some((_father, days)) = self.pregnancy {
            Some(days)
        } else {
            None
        }
    }
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
            super::WuselGender::Female,
            super::Position::new(1, 0),
        ); // female
        test_world.wusel_new(
            "Starver".to_string(),
            super::WuselGender::Male,
            super::Position::new(2, 0),
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
        test_world.wusel_assign_task(1, super::TaskBuilder::use_object(food1_id, 1)); // take
        test_world.wusel_assign_task(1, super::TaskBuilder::move_to(test_world.position_random()));
        test_world.wusel_assign_task(1, super::TaskBuilder::use_object(food1_id, 2)); // drop
        test_world.wusel_assign_task(1, super::TaskBuilder::move_to(test_world.position_random()));
        test_world.wusel_assign_task(1, super::TaskBuilder::use_object(food1_id, 1)); // take not exisiting?

        /* Let the other wusel wait, than it's tries to get the food as well, and consume it. */
        test_world.wusel_assign_task(
            0,
            super::TaskBuilder::move_to(super::Position::new(
                test_world.get_width() - 1,
                test_world.get_depth() - 1,
            )),
        );
        test_world.wusel_assign_task(0, super::TaskBuilder::use_object(food1_id, 1)); // take as well.
        test_world.wusel_assign_task(0, super::TaskBuilder::move_to(test_world.position_random()));
        test_world.wusel_assign_task(0, super::TaskBuilder::use_object(food1_id, 3)); // consume.
        test_world.wusel_assign_task(0, super::TaskBuilder::move_to(test_world.position_random()));
        test_world.wusel_assign_task(0, super::TaskBuilder::move_to(test_world.position_random()));
        log::debug!("Test World's task to work at the workbench assigned.");

        // show everyone's stats.
        for i in 0usize..2 {
            // test_world.wusel_show_tasklist(i); // tasks
            for n in super::Need::VALUES.iter() {
                test_world.wusel_set_need(i, n, 100);
            }
            test_world.wusel_show_overview(i); // needs
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

            // show everyone's stats.
            for i in 0usize..2 {
                test_world.wusel_show_overview(i); // needs
                                                   // test_world.wusel_show_tasklist(i); // tasks
            }

            if test_world.wusel_get_all_unbusy().len() > 1 {
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
            super::WuselGender::Female,
            super::Position { x: 1, y: 0 },
        ); // female
        test_world.wusel_new(
            "2nd".to_string(),
            super::WuselGender::Female,
            super::Position { x: 3, y: 0 },
        ); // female
        test_world.wusel_new(
            "3rd".to_string(),
            super::WuselGender::Male,
            super::Position { x: 5, y: 0 },
        ); // male
        test_world.wusel_new(
            "4th".to_string(),
            super::WuselGender::Male,
            super::Position { x: 9, y: 0 },
        ); // male

        // 4 wusels created.
        assert_eq!(4, test_world.wusel_count());

        /* Create an easy talk, without any preconditions.
         * => no preconditions.
         * => does 'nothing' for ticks steps. */
        let reading: super::TaskBuilder =
            super::TaskBuilder::new(String::from("Reading")).set_duration(5 /*ticks*/);

        test_world.tick();

        // first wusel is also doing something else
        test_world.wusel_assign_task(0, reading.clone()); // do reading.

        // scenario: everyone wants too meet the next one.
        test_world.wusel_assign_task(
            0,
            super::TaskBuilder::meet_with(1, true, false).set_duration(7),
        ); // mutual meeting.
        test_world.wusel_assign_task(
            1,
            super::TaskBuilder::meet_with(2, true, false).set_duration(7),
        ); // mutual meeting.
        test_world.wusel_assign_task(
            2,
            super::TaskBuilder::meet_with(3, true, false).set_duration(7),
        ); // mutual meeting.
        test_world.wusel_assign_task(
            3,
            super::TaskBuilder::meet_with(0, true, false).set_duration(7),
        ); // mutual meeting.

        /* 90 ticks later. */
        for _ in 0..90 {
            test_world.tick();
            // println!("\nTasks at time {}:", test_world.get_time());
            // for w in 0..4 { test_world.wusel_show_tasklist(w); }
        }
    }
}
