extern crate rand;

use std::io;

/** The main method of the wusel world. */
fn main() -> Result<(), io::Error> {
    liv::test_wusel_eats();

    let mut world: liv::World = liv::World::new(80, 30);
    println!("Created a new world: w:{w}, h:{h}",
             w = world.get_width(),
             h = world.get_height());

    /* Empty world tick. */
    world.tick();

    world.new_wusel("1st".to_string(), true); // female
    world.new_wusel("2nd".to_string(), true); // female
    world.new_wusel("3rd".to_string(), false); // male
    world.new_wusel("4th".to_string(), false); // male

    let reading: liv::TaskBuilder = liv::TaskBuilder::new(
        String::from("Reading"))
        // .set_passive_part(String::from("Any Book"))
        .set_passive_part(liv::TaskTag::WaitLike)
        .set_duration(5 /*ticks*/);

    let walking: liv::TaskBuilder = liv::TaskBuilder::new(
        String::from("Walking"))
        // .set_passive_part(String::from("Any Book"))
        .set_passive_part(liv::TaskTag::MoveToPos(10, 10))
        .set_duration(1 /*ticks*/);

    println!("\n----\nCreated a new common Task: {} ({} ticks).", reading.get_name(), reading.get_duration());
    println!("\n----\nCreated a new common Task: {} ({} ticks).", walking.get_name(), walking.get_duration());

    println!("\n\n");

    world.tick();
    println!("\n\n");

    // wusel.improve(liv::Ability::COOKING);
    // wusel.improve(liv::Ability::COMMUNICATION);
    // wusel.improve(liv::Ability::FITNESS);
    // wusel.improve(liv::Ability::FINESSE);

    println!("World Clock: {}", world.get_clock());
    world.show_wusel_overview();
    println!("\n\n");

    world.select_wusel(1);

    println!("World Clock: {}", world.get_clock());
    world.show_wusel_overview();
    println!("\n\n");

    world.assign_task_to_wusel(0, reading.clone());
    world.tick();
    println!("\n\n");

    world.assign_task_to_wusel(1, reading.clone());
    world.assign_task_to_wusel(1, walking.clone());
    world.tick();
    println!("\n\n");

    world.abort_task_from_wusel(0, 0);
    world.tick();
    println!("\n\n");

    world.assign_task_to_wusel(0, reading.clone());

    println!("World Clock: {}", world.get_clock());
    world.show_wusel_overview();

    /* Spend time until almost frozen. */
    for _ in 0..900 { world.tick(); }

    println!("World Clock: {}", world.get_clock());
    world.show_wusel_overview();

    print!("\n");

    // fn draw_the_grid() {
    {
        // world.recalculate_all_positions();

        let positions = world.get_positions();
        let h = world.get_height();
        let w = world.get_width();

        println!("{side}{side:->len$}", side = "+", len = (w+1) as usize); // top side.

        let mut y = h - 1;
        loop {
            print!("|"); // left border.
            for x in 0u32 .. w {
                // draw position. (x,y)
                let on_pos = &positions[(y*w + x) as usize];
                print!("{}", if on_pos.len() < 1 { '`' } else {
                    on_pos[0].0 // type indicator.
                });
            }
            print!("|\n"); // right border and new line.
            if y == 0 { break; } else { y -= 1; }
        }

        println!("{side}{side:->len$}", side = "+", len = (w+1) as usize); // bot side.
    }

    Ok(())
}




mod liv {
    /** The place of existence, time and relations. */
    pub struct World {
        height: u32,
        width: u32,
        positions: Vec<Vec<(char, usize)>>, // all positions [height x width] contain a vector of ids and type/set indicators.

        clock: usize, // time of the world.

        wusels: Vec<Wusel>, // vector of wusels?
        wusel_selected: usize, // currently selected wusel
    }

    impl World {
        /** Create a new world. */
        pub fn new(width: u32, height: u32) -> Self {
            return Self{
                height: height, width: width,
                positions: vec![vec![]; width as usize * height as usize],
                clock: 0,
                wusels: vec![],
                wusel_selected: 0,
            };
        }

        pub const TICKS_PER_DAY: usize = 2880; // 24h by 0.5 minutes

        /** Get width of the world. */
        pub fn get_width(&self) -> u32 {
            self.width
        }

        /** Get height of the world. */
        pub fn get_height(&self) -> u32 {
            self.height
        }

        pub fn get_clock(&self) -> usize {
            self.clock
        }

        const CHAR_WUSEL: char = 'w';

        /** Add a wusel to the world.
         * ID is the current wusel count.
         * TODO (2020-11-20) what is about dead wusels and decreasing length? */
        pub fn new_wusel(self: &mut Self, name: String, female: bool) {
            println!("Create a new wusel at time {}, with the name {} {}",
                     self.clock,
                     name, match female { true => "\u{2640}", _ => "\u{2642}"});

            let id = self.wusels.len();
            let w = Wusel::new(id, name, female);

            /* Add wusel to positions, start at 0. */
            let pos_idx = self.pos_to_idx(w.get_position());
            if pos_idx < self.positions.len() {
                self.positions[pos_idx].push((Self::CHAR_WUSEL, w.id));
            }

            self.wusels.push(w);
        }

        /** Select a wusel by index/living count. */
        pub fn select_wusel(self: &mut Self, selection: usize) {
            self.wusel_selected = usize::min(selection, self.wusels.len());
        }

        /** Give an available wusel (by index) a new task. */
        pub fn assign_task_to_wusel(self: &mut Self, wusel_index: usize, taskb: TaskBuilder) {
            if wusel_index < self.wusels.len() {
                /* Task apply wusel[index] as actor. */
                self.wusels[wusel_index].add_task(taskb);
            }
        }

        /** Abort an assinged task from an available wusel (by index). */
        pub fn abort_task_from_wusel(self: &mut Self, wusel_index: usize, task_index: usize) {
            if wusel_index < self.wusels.len() {
                /* Remove task. */
                self.wusels[wusel_index].abort_task(task_index);
            }
        }

        /** Print overview of (selected) wusel to std::out.*/
        pub fn show_wusel_overview(self: &Self) {
            /* No wusel is there to show. */
            if self.wusels.len() < 1 {
                println!("There is no wusel to show.");
                return;
            }
            self.wusels[self.wusel_selected].show_overview();
        }

        /** Get an index for the wusel with the requesting index.
         * Return LEN, if none is found. */
        fn wusel_identifier_to_index(self: &Self, id: usize) -> usize {
            for i in 0 .. self.wusels.len() {
                if self.wusels[i].id == id {
                    return i;
                } // return matching id.
            }
            return self.wusels.len();
        }

        /** Get the position of the indexed wusel. */
        pub fn get_wusel_position(self: &Self, wusel_index: usize) -> (u32, u32) {
            if wusel_index < self.wusels.len() {
                self.wusels[wusel_index].position
            } else {
                (self.width, self.height) // outside the map.
            }
        }

        /** Set the position of the indexed wusel to the nearest valid position
         * If the position may land out of the grid, put it to the nearest border. */
        pub fn set_wusel_position(self: &mut Self, wusel_index: usize, pos: (u32, u32)) {
            if wusel_index < self.wusels.len() {
                let id = self.wusels[wusel_index].id;

                let old_pos = self.wusels[wusel_index].position;
                let new_pos = (u32::min(pos.0, self.width), u32::min(pos.1, self.height));

                /* Set the new position. */
                self.wusels[wusel_index].position = new_pos;

                /* Update the self.positions. */
                let old_pos_idx = self.pos_to_idx(old_pos);
                let new_pos_idx = self.pos_to_idx(new_pos);

                let wusel_indicator = (Self::CHAR_WUSEL, id);

                /* Remove from old positions[idx]. */
                for i in 0..self.positions[old_pos_idx].len() {
                    if self.positions[old_pos_idx][i] == wusel_indicator {
                        self.positions[old_pos_idx].remove(i);
                        break;
                    }
                }

                /* Add to new positions[idx]. */
                self.positions[new_pos_idx].push(wusel_indicator);

            }
        }

        /** Get the positions of all active wusels. */
        #[allow(dead_code)]
        pub fn get_all_wusels_positions(&self) -> Vec<(u32, u32)> {
            let mut positions = vec![];
            for w in self.wusels.iter() {
                positions.push((w.position.0, w.position.1));
            }
            return positions;
        }

        /** Check all positions.
         * Recalculate all positions, if they really consist what they promis. */
        #[allow(dead_code)]
        pub fn recalculate_all_positions(&mut self) {
            self.positions = vec![vec![]; self.width as usize * self.height as usize];

            let valid_idx = self.positions.len();

            for w in self.wusels.iter() {
                let pos = w.position;
                let idx = (pos.0 + self.width * pos.1) as usize;

                /* Add id to position. */
                if idx < valid_idx {
                    self.positions[idx].push((Self::CHAR_WUSEL, w.id));
                }

                println!("[DEBUG] idx({} \u{2190} {:?}): add ({},{})", idx, pos, Self::CHAR_WUSEL, w.id);
            }
        }

        /** Get the `positions` index for the requesting position.
         * If the position is not in world, this index is not in [0,positions.len()).*/
        fn pos_to_idx(&self, pos: (u32, u32)) -> usize {
            (pos.0 + self.width * pos.1) as usize
        }

        /** Get all the positions as they are. */
        pub fn get_positions(&self) -> Vec<Vec<(char, usize)>> {
            self.positions.clone()
        }

        /** Increase clock and proceed decay of all things and relations. */
        pub fn tick(self: &mut Self) {

            self.clock += 1;

            /* A new day is over: Forward the day structure to the world. */
            let new_day: bool = self.clock % Self::TICKS_PER_DAY == 0;

            let mut ongoing_tasks: Vec<Task> = vec![];

            /* Decay on every object and living. */
            for w in self.wusels.iter_mut() {

                /* Peek ongoing tasks of (all wusels) and try to proceed.
                 * While peeking, this may remove done tasks and maybe return Nothing. */
                w.auto_clean_tasks();

                if let Some(task) = w.pop_ongoing_task() {
                    ongoing_tasks.push(task);
                } else {
                    /* Wusel is currently unbusy. => maybe apply an idle/auto task. */
                }

                w.tick();
                if new_day {
                    w.add_new_day()
                } // add new day to live.
            }

            /* Execute ongoing tasks. */
            while ongoing_tasks.len() > 0 {
                if let Some(t) = ongoing_tasks.pop() {
                    self.proceed(t);
                }
            }
        }

        /** Proceed the task in this world.
         * @return true, if they are still ongoing. */
        fn proceed(self: &mut World, mut task: Task) -> bool {

            println!("\nWorld proceeds task: {}", task.name);
            println!(" - steps: {}/{}", task.done_steps, task.duration);

            let wusel_size = self.wusels.len();
            let actor_index = self.wusel_identifier_to_index(task.active_actor_id);

            if actor_index >= wusel_size {
                println!(" - actor not available.");
                return false; // abort.
            }

            println!(" - actor: ID: {} => IDX: {}",
                     task.active_actor_id, actor_index);

            // let mut actor: &mut Wusel;
            let still_running: bool;

            match task.passive_part {
                TaskTag::WaitLike => {
                    println!("Wait?");
                },
                TaskTag::MoveToPos(x,y) => {
                    println!("Move Goal: ({}, {}).", x, y);
                    // XXX easy placeholder walking, ignoring all obstacles.

                    let apos = self.get_wusel_position(actor_index);
                    let (actor_x, actor_y) = apos;

                    if actor_x == x && actor_y == y {
                        println!("Goal ({},{}) reached.", x, y);
                        return false;
                    }

                    let neighbours = Self::get_neighbour_positions(
                        self.width, self.height,
                        apos);

                    if neighbours.len() < 1 {
                        println!("Wusel cannot move, it's enclosed, wait forever");
                        return true;
                    }

                    let goal: (u32, u32) = (x, y);
                    let mut closest: (u32, u32) = neighbours[0];
                    let mut closest_distance: f32 = f32::MAX;

                    for p in neighbours.iter() {
                        let distance = Self::get_distance_between(goal, *p);

                        if distance < closest_distance {
                            closest = *p;
                            closest_distance = distance;
                        }
                    }

                    /* move to closest position. */
                    self.set_wusel_position(actor_index, closest);

                    task.duration += 1; // XXX stop from ending early.

                    // XXX calculate the path for a certain reach/depth, go first step.
                    // XXX where to save the precalculated path?
                },
                TaskTag::GetFromPos(x,y) => {
                    println!("Pick from Goal: ({}, {}).", x, y);
                    // == MoveToPos(x,y) && pick up on field?
                },
                TaskTag::PutAtPos(x,y) => {
                    println!("Drop at Goal: ({}, {}).", x, y);
                    // == MoveToPos(x,y) && drop up on field?
                },
                TaskTag::MeetWith(other, nice) => {
                    println!("Commute with ID: {}, nice: {}.", other, nice);

                    let other_index = self.wusel_identifier_to_index(other);

                    if other_index >= self.wusels.len() {
                        println!("Cannot meet with ID {}! Abort.", other);
                        return false;
                    }

                    // == MoveToPos(other.x, other.y) && drop up on field?
                },
            }

            // XXX (2020-11-16) implement pre-conditon testsing. remove place holder.
            let dissatisfied: u8 = rand::random::<u8>() % 11;

            /* Check, if precondition are satified.
             * Maybe proceed to satisfy those condition. */
            for i in 0u8..10 {
                println!("Check Task Conditon {}", i, );
                if i == dissatisfied {
                    println!(" - Condition not satisfied, satisfy first.");

                    // XXX satisfy preconditon.
                    // proceed(precondition_subtask);

                    self.wusels[actor_index].tasklist.push(task); // put back to ongoing.
                    return true; // still running.
                }
            } // if precondition not satisfied, the code will return before here.

            /* If not done yet, put back to tasklist (the ongoing task.) */

            task.done_steps += 1;
            still_running = task.done_steps < task.duration;

            if still_running {
                /* Insert back, where current task are (reversed queue) */
                self.wusels[actor_index].tasklist.push(task);
            }

            return still_running;
        }

        /** Get the (valid) neighbours for a position. */
        pub fn get_neighbour_positions(box_width: u32, box_height: u32, pos: (u32,u32)) -> Vec<(u32, u32)> {
            let mut neighbours: Vec<(u32,u32)> = vec![];

            /* Get all the valid neighbours. */
            for d in Way::NEIGHBOURING.iter() {
                if let Some(n) = Self::get_neighbour_on(box_width, box_height, pos, *d) {
                    neighbours.push(n);
                }
            }

            return neighbours;
        }

        /** Get the next optional neighbour to the given position within the given box. */
        pub fn get_neighbour_on(box_width: u32, box_height: u32, pos: (u32,u32), direction: Way) -> Option<(u32, u32)> {
            let change = direction.as_direction_tuple();

            /* On west border => No west neighbours. (None) */
            if pos.0 < 1 && change.0 < 0 {
                return None;
            }

            /* On east border => No east neighbours. (None) */
            if pos.0 >= box_width && change.0 > 0 {
                return None;
            }

            /* On south border => No south neighbours. (None) */
            if pos.1 < 1 && change.1 < 0 {
                return None;
            }

            /* On north border => No north neighbours. (None) */
            if pos.1 >= box_height && change.1 > 0 {
                return None;
            }

            return Some(
                ((pos.0 as i64 + change.0 as i64) as u32,
                (pos.1 as i64 + change.1 as i64) as u32));
        }

        /** Get the distance between two positions. */
        pub fn get_distance_between(a: (u32, u32), b: (u32, u32)) -> f32 {
            (((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)) as f32).sqrt()
        }
    }

    /** Way in the world. */
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Way {
        NW, N, NE,
        W,      E,
        SW, S, SE,
    }

    impl Way {
        pub const NEIGHBOURING: [Self; 8] = [
            Self::NW,Self::N, Self::NE, // north
            Self::W,          Self::E, // same longitude
            Self::SW,Self::S, Self::SE, // south
        ];
        /** Get the offsets to walk, to get to the way point. */
        pub fn as_direction_tuple(self: &Self) -> (i8,i8) {
            match self {
                /* Go north. */
                Way::NW => return (-1,  1),
                Way::N =>  return ( 0,  1),
                Way::NE => return ( 1,  1),

                /* Stay on longitude. */
                Way::W =>  return (-1,  0),
                Way::E =>  return ( 1,  0),

                /* Go south. */
                Way::SW => return (-1, -1),
                Way::S =>  return ( 0, -1),
                Way::SE => return ( 1, -1),
            }
        }
    }

    /** Something a Wusel can consume. */
    pub struct Food {
        name: String,

        position: (u32, u32),

        available: f32, // 1.0f whole, 0.0f gone.
        bites: u32, // eats an x-th part per minute of this food. => partition_per_bite

        /* Per bite. */
        needed_energy: u32, // needed energy to eat; eg. eating a cup of Water or eating raw meat
        satisfied_food: u32, // satisfies hunger need.
        satisfied_water: u32, // satisfies water need.

        spoils_after: u32, // spoils after 0: infinite, or N days.
        age: u32,
    }

    impl Food {
        fn new(name: String,
               bites: u32,
               energy_per_bite: u32,
               food_per_bite: u32,
               water_per_bite: u32,
               spoils: u32,
               ) -> Self {
            Self {
                name: name,

                position: (0, 0), // start at root

                available: 1.0f32, // start fully.
                bites: bites, // => 0.5 per minute

                needed_energy: energy_per_bite,
                satisfied_food: food_per_bite,
                satisfied_water: water_per_bite,

                spoils_after: spoils, // after days, 0: does not spoil.
                age: 0,
            }
        }

        /** Render a nice String to show the food, it's availability and spoilage. */
        fn show(self: &Self) -> String {
            let maybe_spoiled = if self.is_spoiled() {
                " (spoiled)"
            } else {
                ""
            };
            let avail: i32 = (self.available * 1000f32) as i32;
            return match avail {
                0 => format!("Eaten {} (fully gone)", self.name),
                1000 => format!("{}{}", self.name, maybe_spoiled),
                _ => format!("{} {}{}", self.available, self.name, maybe_spoiled),
            }
        }

        /** Check if the food is already spoiled. */
        fn is_spoiled(self: &Self) -> bool {
            self.age >= self.spoils_after
        }

        /** Get the size of one bite. */
        fn bite_part(self: &Self) -> f32 {
            1.0 / self.bites as f32
        }

        /** Take a bite: Decrease availability by one bite.
         * @return true, if the bite was successful. */
        fn take_a_bite(self: &mut Self) -> bool {
            if self.available > 0.0 {
                self.available -= self.bite_part(); // remove a part.
                true
            } else {
                false
            }
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

    /** A need, the wusel needs to satisfy to survive. */
    #[derive(Copy, Clone, PartialEq)]
    pub enum Need {
        WATER, FOOD, SLEEP, LOVE, FUN, WARMTH, HEALTH,
    }

    impl Need {
        /** Custom iteratable values. */
        pub const VALUES: [Self; 7] = [
            Self::WATER,  Self::FOOD,
            Self::SLEEP, Self::LOVE, Self::FUN,
            Self::WARMTH, Self::HEALTH,
        ];

        const DEFAULT_NEED_DECAY_PER_MINUTE: [u32; 7] = [
            1, 1, 1, 1, 1,
            0/*warmth*/, 0/*health*/, // by outer sources
        ];

        fn name(&self) -> &str {
            return match self {
                Self::WATER => "water", Self::FOOD   => "food"  ,
                Self::WARMTH => "warmth", Self::SLEEP => "sleep",
                Self::HEALTH => "health", Self::LOVE   => "love",
                Self::FUN   => "fun",
            }
        }

        fn get_default_decay(&self) -> u32 {
            for i in 0 .. Self::VALUES.len() {
                if self == &Self::VALUES[i] {
                    return Self::DEFAULT_NEED_DECAY_PER_MINUTE[i];
                }
            }
            return 0; // default: no decay.
        }
    }

    /** An ability, the wusel can learn to improve their lifestyle. */
    #[derive(Copy, Clone, PartialEq)]
    pub enum Ability {
        COOKING,
        COMMUNICATION,
        FITNESS,
        FINESSE,
    }

    impl Ability {
        fn name(&self) -> &str {
            return match self {
                Self::COOKING => "cooking",
                Self::COMMUNICATION => "communication",
                Self::FITNESS => "fitness",
                Self::FINESSE => "finesse",
            }
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
            Self { name: name, duration: 0, passive_part: TaskTag::WaitLike }
        }

        /** Get the name of the future task or all then created tasks. */
        pub fn get_name(self: &Self) -> String {
            self.name.clone()
        }

        /** Get the duration of the future task or all then created tasks. */
        pub fn get_duration(self: &Self) -> usize {
            self.duration
        }

        /** Set the duration in the Task Builder. */
        pub fn set_duration(mut self, time: usize) -> Self {
            self.duration = time;
            return self;
        }

        /** Set the duration in the passive part. */
        pub fn set_passive_part(mut self, passive: TaskTag) -> Self {
            self.passive_part = passive;
            return self;
        }

        /** Create a new Task from the builder for the requesting [actor](Wusel). */
        fn assign(self, actor: &Wusel) -> Task {
            Task {
                name: self.name,
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
        duration: usize,
        done_steps: usize,

        active_actor_id: usize, // wusel id.
        passive_part: TaskTag, // position | object-to-be | object | wusel | nothing.
    }

    #[derive(Debug, Clone)]
    pub enum TaskTag {
        WaitLike,
        MoveToPos(u32, u32),
        GetFromPos(u32, u32), // pick up a thing from positon.
        PutAtPos(u32, u32), // drop something at pos.
        MeetWith(usize, bool), // commute with another wusel (id)
    }

    impl Task {
        /** Get the approximatly rest time (im ticks), this task needs. */
        fn get_rest_time(self: &Self) -> usize {
            self.duration - self.done_steps
        }
    }

    /** Wusel.
     * Bundle of information on a certain position and abilities.
     */
    pub struct Wusel {
        id: usize,

        /* Name */
        name: String,

        position: (u32, u32),

        female: bool, // female => able to bear children, male => able to inject children
        pregnant: bool,

        life: Life, // alive | dead | ghost
        lived_days: u32, // last lived day.

        needs: Vec<(Need, u32)>,

        /* Abilities. */
        abilities: Vec<(Ability, u32)>, // ability levels.

        /* List of tasks. */
        tasklist: Vec<Task>,
    }

    impl Wusel {

        /** From full to 0, how many ticks does it need, when it's only normally decreasing. */
        const WUSEL_FULL_NEEDS: [(Need, u32); 7]
            = [ (Need::WATER, (24 * 60 * 2) * 3) // 3 days until dehydrate.
            , (Need::FOOD, (24 * 60 * 2) * 7) // a week until starve.
            , (Need::WARMTH, (8 * 60 * 2)) // 8h until freeze to death.
            , (Need::SLEEP,  (24 * 60 * 2) * 7) // a week until suffer from sleep loss.
            , (Need::HEALTH, (24 * 60 * 2) * 14) // 2 weeks until die of illness.
            , (Need::LOVE, (24 * 60 * 2) * 14) // 2 weeks until become lonely.
            , (Need::FUN, (24 * 60 * 2) * 14) // 2 weeks until unmotivated and depressive.
            ];

        /** Create a new Wusel with name. */
        fn new(id: usize, name: String, female: bool) -> Self {
            let mut new = Self {
                id: id,
                name: name,

                position: (0, 0), // start at root

                female: female,
                pregnant: false,

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

        /** Show position of its body. */
        pub fn get_position(self: &Self) -> (u32, u32) {
            (self.position.0, self.position.1)
        }

        /** Tick one unit.
         * @return if the wusel is still alive in the end. */
        fn tick(self: &mut Self) -> bool {

            /* If in action, need changes may also apply, eg. eating. */
            // self.act(); // proceed on task, if tasklist is providing one.

            /* Decrease every value by DEFAULT_NEED_DECAY_PER_MINUTE * minutes. */
            for i in 0 .. self.needs.len() {
                let (n, v) = self.needs[i];
                let decay = n.get_default_decay();

                // XXX when SICK: decay health
                // XXX when IN COLD: decay warmth

                self.needs[i] = (n, if v < decay { 0 } else { v - decay });
            }

            return self.is_alive()
        }

        /** Count a new day to the lived lifed. */
        fn add_new_day(self: &mut Self) {
            if self.is_alive() {
                /* Age one day. */
                self.lived_days += 1;

                /* Decay all abilities by one point. */
                for i in 0 .. self.abilities.len() {
                    let (abi, val) = self.abilities[i];
                    self.abilities[i] = (abi, val - 1);
                }
            }
        }

        /** Check, if this Wusel is alive. */
        fn is_alive(self: &Self) -> bool {
            return match self.life {
                Life::ALIVE => true, // all but alive are not alive.
                _ => false,
            }
        }

        /** Get name of the Wusel. */
        fn get_name(&self) -> String {
            self.name.clone()
        }

        /** Show the name, gender and age. */
        fn show(self: &Self) -> String {
            /* The name */
            let mut string = self.name.clone();
            string.push(' ');

            /* Gender */
            string.push_str(if self.female { "\u{2640}" } else { "\u{2642}" });

            /* Birth tick. */
            string.push_str(" (");

            /* Show life and age. */
            match self.life {
                Life::ALIVE => println!(""),
                Life::DEAD => println!("dead, "),
                Life::GHOST => println!("ghost, "),
            }
            string.push_str(&self.lived_days.to_string());
            string.push_str("d)");

            return string;
        }

        /** Show collected data. */
        fn show_overview(self: &Self) {
            println!("==={:=<40}", "");

            println!("  {}", self.show());

            /* Show needs. */
            println!("---{:-<40}", " NEEDS: ");
            self.show_needs();

            /* Show abilities. */
            println!("---{:-<40}", " ABILITIES: ");
            self.show_abilities();

            /* Show relations. */
            // TODO (2020-11-16) show relations.
            println!("{:_<43}", "");
        }

        /** Show all assigned needs. */
        fn show_needs(self: &Self) {
            for (n, v) in self.needs.iter() {

                let full = Self::default_need_full(n);

                let max_len = 20;
                let bar_len = (*v  * max_len / full) as usize;

                println!(" {name:>14} {value:5} {end:.>bar_len$} ",
                         name = n.name(),
                         value = v,
                         bar_len = usize::min(bar_len, max_len as usize),
                         end = "");
            }
        }

        /** Print the Wusel's abilities. */
        fn show_abilities(self: &Self) {
            for (ability, value) in &self.abilities {
                println!("{a:>15} {v:5} {bar:*<v$}",
                         a = ability.name(),
                         v = *value as usize,
                         bar = "");
            }
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
            return default
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
        fn add_need(self: &mut Self, need: Need, change_value: u32) -> u32 {
            // 3x iterating. O(3n + c)
            let current = self.get_need(need) as i64; // get current value (or default)

            let mut changed = current + (change_value as i64);
            if changed < 0 {
                changed = 0;  // don't go below 0.
            }

            self.set_need(need, changed as u32); // change the value.

            return self.get_need(need); // return final need's value.
        }

        /** Improve the given ability by one point. */
        fn improve(self: &mut Self, ability: Ability) {
            /* Improve the given ability. */
            for i in 0 .. (self.abilities.len()) {
                let (a, v) = self.abilities[i];
                if ability == a {
                    self.abilities[i] = (a, v + 1);
                    return;
                }
            }
            /* If the given ability is not yet learned, add it to the abilities. */
            self.abilities.push((ability, 1));
        }

        /** Append a new task to the task list. */
        fn add_task(self: &mut Self, task_builder: TaskBuilder) {
            /* Task apply self as actor. */
            println!("Wusel {} received new task '{}'", self.name, task_builder.name);
            let task = task_builder.assign(self);
            self.tasklist.insert(0, task); // revert queue
        }

        /** Abort a task in the task list. */
        fn abort_task(self: &mut Self, index: usize) {
            if index < self.tasklist.len() {
                println!("Wusel {} aborted a task '{}'",
                         self.name,
                         self.tasklist[index].name);
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

        /** Peek the ongoing task. */
        fn peek_ongoing_task(self: &Self) -> Option<&Task> {
            self.tasklist.last()
        }

        /** Pop the ongoing task (queue reversed). */
        fn pop_ongoing_task(self: &mut Self) -> Option<Task> {
            self.tasklist.pop()
        }

        /** Eat Food, take one bite.
         * This may use up the food. It may update the needs. */
        fn eat(self: &mut Self, food: &mut Food) {
            /* Take a bite. */
            let successfully_fed: bool = food.take_a_bite();

            if successfully_fed {
                self.add_need(Need::SLEEP, food.needed_energy);
                self.add_need(Need::FOOD, food.satisfied_food);
                self.add_need(Need::WATER, food.satisfied_water);
            }
        }
    }

    /** Simple Test if eating works. */
    pub fn test_wusel_eats() {
        let mut w0 = Wusel::new(0, String::from("Test Eat"), false);

        let mut sushi = Food::new(String::from("Sushi"), 2, 1, 5, 2, 2);
        println!("Food: {}", sushi.show());

        sushi.position = (0, 1);

        w0.eat(&mut sushi);
        w0.show_overview();

        println!("Food: {}", sushi.show());

        w0.eat(&mut sushi);
        w0.show_overview();

        println!("Food: {}", sushi.show());

        w0.eat(&mut sushi);
        w0.show_overview();

        println!("Food: {}", sushi.show());
    }
}
