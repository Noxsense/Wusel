extern crate rand;

use std::io;

/** The main method of the wusel world. */
fn main() -> Result<(), io::Error> {
    liv::test_wusel_walking();
    liv::test_wusel_eats();

    let mut world: liv::World = liv::World::new(100, 100);
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

    println!("\n----\nCreated a new common Task: {} ({} ticks).", reading.get_name(), reading.get_duration());

    println!("\n\n");

    world.tick();
    println!("\n\n");

    // wusel.improve(liv::Ability::COOKING);
    // wusel.improve(liv::Ability::COMMUNICATION);
    // wusel.improve(liv::Ability::FITNESS);
    // wusel.improve(liv::Ability::FINESSE);

    world.show_wusel_overview();
    println!("\n\n");

    world.select_wusel(1);

    world.show_wusel_overview();
    println!("\n\n");

    world.assign_task_to_wusel(0, reading.clone());
    world.tick();
    println!("\n\n");

    world.assign_task_to_wusel(1, reading.clone());
    world.tick();
    println!("\n\n");

    world.abort_task_from_wusel(0, 0);
    world.tick();
    println!("\n\n");

    world.assign_task_to_wusel(0, reading.clone());

    for _ in 0..100 { world.tick(); world.show_wusel_overview(); println!("\n\n"); }

    Ok(())
}




mod liv {
    static WORLD_HEIGHT: u32 = 100;
    static WORLD_WIDTH: u32 = 100;

    /** The place of existence, time and relations. */
    pub struct World {
        height: u32,
        width: u32,

        clock: usize, // time of the world.

        things: Vec<Thing>, // vector of things ?

        wusels: Vec<Wusel>, // vector of wusels?
        wusel_selected: usize, // currently selected wusel
    }

    impl World {
        /** Create a new world. */
        pub fn new(height: u32, width: u32) -> Self {
            return Self{
                height: height, width: width,
                clock: 0,
                things: vec![],
                wusels: vec![],
                wusel_selected: 0,
            };
        }

        /** Get width of the world. */
        pub fn get_width(&self) -> u32 {
            self.width
        }

        /** Get height of the world. */
        pub fn get_height(&self) -> u32 {
            self.height
        }

        const TICKS_A_DAY: usize = 2880; // 24h by 0.5 minutes

        /** Add a wusel to the world.
         * ID is the current wusel count.
         * TODO (2020-11-20) what is about dead wusels and decreasing length? */
        pub fn new_wusel(self: &mut Self, name: String, female: bool) {
            println!("Create a new wusel at time {}, with the name {} {}",
                     self.clock,
                     name, match female { true => "\u{2640}", _ => "\u{2642}"});
            let w = Wusel::new(self.wusels.len(), name, female);
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
                if self.wusels[i].id == id { return i; }
            }
            return self.wusels.len();
        }

        /** Increase clock and proceed decay of all things and relations. */
        pub fn tick(self: &mut Self) {

            self.clock += 1;

            /* A new day is over: Forward the day structure to the world. */
            let new_day: bool = self.clock % Self::TICKS_A_DAY == 0;

            println!("New Tick. {}", if new_day { "A new day"} else { "" });

            let mut ongoing_tasks: Vec<Task> = vec![];

            /* Decay on every object and living. */
            //for i in 0..self.wusels.len() {
            for w in self.wusels.iter_mut() {

                /* Peek ongoing tasks of (all wusels) and try to proceed.
                 * While peeking, this may remove done tasks and maybe return Nothing. */
                w.auto_clean_tasks();

                if let Some(task) = w.pop_ongoing_task() {
                    ongoing_tasks.push(task);
                }

                w.tick();
                if new_day { w.add_new_day() }
                println!("Updated: {}", w.show());
            }

            /* Execute ongoing tasks. */
            while ongoing_tasks.len() > 0 {
                if let Some(t) = ongoing_tasks.pop() {
                    self.proceed(t);
                }
            }

            for _ in self.things.iter() {
                /* Decay of things over time. */
            }
        }

        /** Proceed the task in this world.
         * @return true, if they are still ongoing. */
        fn proceed(self: &mut World, mut task: Task) -> bool {

            println!("World proceeds task: {}", task.name);
            println!(" - steps: {}/{}", task.done_steps, task.duration);

            let wusel_size = self.wusels.len();
            let actor_index = self.wusel_identifier_to_index(task.active_actor_id);

            if actor_index >= wusel_size {
                println!(" - actor not available.");
                return false; // abort.
            }

            let actor = &mut self.wusels[actor_index];

            println!(" - actor: {} => {}", task.active_actor_id, actor.show());

            match task.passive_part {
                TaskTag::WaitLike => println!("Yep."),
                TaskTag::MoveToPos(x,y) => println!("Move Goal: ({}, {}).", x, y),
                TaskTag::GetFromPos(x,y) => println!("Pick from Goal: ({}, {}).", x, y),
                TaskTag::PutAtPos(x,y) => println!("Drop at Goal: ({}, {}).", x, y),
                TaskTag::CommuteWith(other, nice) => println!("Commute with ID: {}, nice: {}.", other, nice),
            }


            /* If not done yet, put back to tasklist (the ongoing task.) */

            task.done_steps += 1;
            let still_running = task.done_steps < task.duration;

            if still_running {
                /* Insert back, where current task are (reversed queue) */
                actor.tasklist.push(task);
            }

            return still_running;
        }
    }

    /** Way in the world. */
    #[derive(Debug)]
    pub enum Way {
        NW, N, NE,
        W,      E,
        SW, S, SE,
    }

    /** A world body reprensentation, objects in the world have. */
    pub struct Thing {
        /* Position */
        pos_x: u32, // unsigned int 32
        pos_y: u32, // unsigned int 32

    }

    impl Thing {
        fn new(x: u32, y: u32) -> Self {
            Self { pos_x: x, pos_y: y }
        }

        /** Show position of npc. */
        fn show_pos(self: &Self) -> String {
            return format!("(x: {} | y: {})", self.pos_x, self.pos_y);
        }

        /** Move the thing in certain way. */
        fn step(self: &mut Self, way: Way) {
            /* Mutable variable. Will contain the possible new position. */
            let mut _x: i64 = self.pos_x as i64;
            let mut _y: i64 = self.pos_y as i64;

            /* Determine changes. */
            match way {
                Way::N =>  { _x +=  0; _y +=  1},
                Way::NE => { _x +=  1; _y +=  1},
                Way::NW => { _x += -1; _y +=  1},
                Way::S =>  { _x +=  0; _y += -1},
                Way::SE => { _x +=  1; _y += -1},
                Way::SW => { _x += -1; _y += -1},
                Way::E =>  { _x +=  1; _y +=  0},
                Way::W =>  { _x += -1; _y +=  0},
            }

            if _x > (WORLD_HEIGHT as i64) || _x < 0 {
                println!("Already at Vertical Bordee.");
            } else {
                /* Apply to real position. */
                self.pos_x = _x as u32;
            }

            if _y > (WORLD_WIDTH as i64) || _y < 0 {
                println!("Already at Horizontal Border");
            } else {
                /* Apply to real position. */
                self.pos_y = _y as u32;
            }
        }

        /** Place the thing on a new Position. */
        fn place_at(self: &mut Self, new_x: u32, new_y: u32) {
            self.pos_x = if new_x <= WORLD_HEIGHT { new_x } else { WORLD_HEIGHT };
            self.pos_y = if new_y <= WORLD_WIDTH { new_y } else { WORLD_WIDTH };
        }

    }

    /** Something a Wusel can consume. */
    pub struct Food {
        name: String,

        body: Thing,

        available: f32, // 1.0f whole, 0.0f gone.
        bites: u32, // eats an xth part per minute of this food. => partition_per_bite

        /* Per bite. */
        needed_energy: u32, // needed energy to eat; eg. eating a cup of Water or eating raw meat
        satisfied_food: u32, // satiesfies hunger need.
        satisfied_water: u32, // satiesfies water need.

        spoils_after: u32, // spoils after 0: infinite, or N daya.
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

                body: Thing::new(0, 0), // start at center.

                available: 1.0f32, // start fully.
                bites: bites, // => 0.5 per minute

                needed_energy: energy_per_bite,
                satisfied_food: food_per_bite,
                satisfied_water: water_per_bite,

                spoils_after: spoils, // after days, o: do noy spoil.
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

    /** Pregancy: Not pregnant or pregnant with unfinished Wusel. */
    pub enum Pregnant {
        // an optional pregnancy.
        YES(u8), // dayes until arrival.
        NO,
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
        WATER, FOOD, WARMTH, SLEEP, HEALTH, LOVE, FUN,
    }

    impl Need {
        /** Custom iteratable values. */
        pub const VALUES: [Self; 7] = [
            Self::WATER,  Self::FOOD, Self::WARMTH, Self::SLEEP,
            Self::HEALTH, Self::LOVE, Self::FUN];

        const DEFAULT_NEED_DECAY_PER_MINUTE: [u32; 7] = [
            1, 1, 1, 1, 0/*health*/, 1, 1,
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
        pub fn get_name(self: &Self) -> String { self.name.clone() }

        /** Get the duration of the future task or all then created tasks. */
        pub fn get_duration(self: &Self) -> usize { self.duration }

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
        CommuteWith(usize, bool), // commute with another wusel (id)
    }

    impl Task {
        /** Proceed on task.
         * If a condition is not yet satisfied, try to satisfy first.
         * Satisfying means: Proceeding on Linked Tasks.
         * This may move the actor around. create/move/destroy targets, imfluence the relatioms etc.*/
        // fn proceed(self: &mut Self) {
        fn proceed(self: &mut Self, actor: &mut Wusel) {
            if actor.id != self.active_actor_id {
                println!("Current Actor is not Assigned Actor.\nTODO Do we need the assigned Actor?\n");
                return;
            }

            println!("Living {} is doing  '{}'", actor.name, self.name);

            // XXX (2020-11-16) implement pre-conditon testsing. remove place holder.
            let dissatisfied: u8 = rand::random::<u8>() % 11;
            /* Check, if satified. Maybe proceed to satisfy condition. */
            for i in 0u8..10 {
                println!("Check Task Conditon {}", i);
                if i == dissatisfied {
                    println!("Condition not mer, satisfy first.");
                    // subtask_to_solve_the_issue.proceed();
                    // or wait.proceed();
                    return;
                }
            } // if precondition not satisfied, the code will return before here.

            match self.passive_part {
                TaskTag::WaitLike => println!("Yep."),
                TaskTag::MoveToPos(x,y) => println!("Move Goal: ({}, {}).", x, y),
                TaskTag::GetFromPos(x,y) => println!("Pick from Goal: ({}, {}).", x, y),
                TaskTag::PutAtPos(x,y) => println!("Drop at Goal: ({}, {}).", x, y),
                TaskTag::CommuteWith(other, nice) => println!("Commute with ID: {}, nice: {}.", other, nice),
            }

            /* Proceed on task, while condition is met. */
            /* Inc. spent time. */
            // TODO (2020-11-19) what if when going to a moving target needs ;ater additional steps?
        }

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

        body: Thing,

        female: bool, // female => able to bear children, male => able to inject children
        pregnant: Pregnant,

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

                body: Thing::new(0, 0),

                female: female,
                pregnant: Pregnant::NO,

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

        /** Create a new female Wusel. */
        fn xx(id: usize, name: String) -> Self { Self::new(id, name, true) }

        /** Create a new male Wusel. */
        fn xy(id: usize, name: String) -> Self { Self::new(id, name, false) }

        /** Show position of its body. */
        fn show_pos(self: &Self) -> String { self.body.show_pos() }

        /** Move position of its body. */
        fn step(self: &mut Self, way: Way) { self.body.step(way) }

        /** Tick one unit.
         * @return if the wusel is still alive in the end. */
        fn tick(self: &mut Self) -> bool {

            /* If in action, need changes may also apply, eg. eating. */
            // self.act(); // proceed on task, if tasklist is providing one.

            /* Decrease every value by DEFAULT_NEED_DECAY_PER_MINUTE * minutes. */
            for i in 0 .. self.needs.len() {
                let (n, v) = self.needs[i];
                let decay = n.get_default_decay();

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
        fn get_name(&self) -> String { self.name.clone() }

        /** Get Wusel's age. */
        fn get_age(self: &Self) -> u32 {
            self.lived_days
        }

        /** Show the name, gender and age. */
        fn show(self: &Self) -> String {
            /* The name */
            let mut string = self.name.clone();
            string.push(' ');

            /* Gender */
            string.push_str(if self.female { "\u{2640}" } else { "\u{2642}" });

            /* Birth tick. */
            string.push_str(", (");
            string.push_str(&self.lived_days.to_string());
            string.push_str("d)");

            return string;
        }

        /** Show collected data. */
        fn show_overview(self: &Self) {
            println!("==={:=<40}", "");
            /* Show name. */
            print!("  {}", self.name);

            /* Show Gender.. [\u2640 (9792) female, \u2642 (9794) male]. */
            print!("  {}\n", match self.female {
                true => "\u{2640}",
                _ => "\u{2642}",
            });

            /* Show age. */
            print!(" {age} days ", age = self.get_age());

            /* Show life and age. */
            match self.life {
                Life::ALIVE => println!("(alive)"),
                Life::DEAD => println!("(dead)"),
                Life::GHOST => println!("(ghost)"),
            }

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
                let bar_len = (*v  * max_len / full + 1) as usize;

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
                if n == need { return v; } // return assigned value
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
            if changed < 0 { changed = 0; } // don't go below 0.

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

        /** Check, if the Wusel is busy right now. */
        fn is_busy(self: &Self) -> bool {
            self.tasklist.len() > 0 // busy while tasklistnis not empty.
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

        /** Talk with an NPC.
         * @param npc Non-Playable Character, this Wusel talks with.
         * @param romantically If the talk meant to be romantically, the romantic value changed as well.
         * @param good If the talk was good, the relation has improved, otherwise worsen.
         */
        fn interact_with(self: &mut Self, other: &mut Self, romantically: bool, good: bool) {
            let val: i32;

            if good {
                self.improve(Ability::COMMUNICATION);
                val = 1;
            } else {
                val = -1;
            }

            /* Change romance, if intended. */
            if romantically {
            } // romance updated
        }

        /** Talk with an NPC romantically.
         * @param npc Non-Playable Character, this Wusel talks with.
         * @param good If the talk was good, the relation has improved, otherwise worsen.
         */
        fn flirt(self: &mut Self, npc: &mut Self, good: bool) {
            self.interact_with(npc, true, good);
        }

        /** Talk with an NPC friendly.
         * @param npc Non-Playable Character, this Wusel talks with.
         * @param good If the talk was good, the relation has improved, otherwise worsen.
         */
        fn chat(self: &mut Self, npc: &mut Self, good: bool) {
            self.interact_with(npc, false, good);
        }

        /** Try to mate with the given NPC.
         * This may lead to pregnancy.
         */
        fn mate_with(self: &mut Self, npc: &mut Self, good: bool) {
            /* Mating is accepted, if romantically and friendly feelings are high. */
            let accepted: bool = good; // TODO (2020-11-15)  npc.romantic_value > 0 && npc.friendly_value > 0 && good;

            /* Pregnancy is allowed, if the mating genders are different, and randomly. */
            let impregnating: bool = self.female != npc.female;

            /* Abort, if mating is not accepted. */
            if !accepted { return; }

            /* This increases the friendship and romantic feelings.
             * It also improves the communication.
             * (5x more worth than normal flirt). */
            let factor = 2;
            for _ in 0u16..factor { self.flirt(npc, true); }

            if impregnating {
                let pregnancy_length = 7;

                if self.female {
                    self.pregnant = Pregnant::YES(pregnancy_length);
                } else {
                    npc.pregnant = Pregnant::YES(pregnancy_length);
                }
            }
        }

        /** Check, if the Wusel is pregnant. */
        fn is_pregnant(self: &Self) -> bool {
            match self.pregnant {
                Pregnant::NO => return false,
                _ => return true,
            }
        }
    }

    /** Simple Test of stepping works. */
    pub fn test_wusel_walking() {
        let mut w0 = Wusel::xy(0, String::from("Test Move"));

        w0.step(Way::N); // 0,1
        println!("{} at {}", w0.get_name(), w0.show_pos());
        assert_eq!(w0.body.pos_x, 0); assert_eq!(w0.body.pos_y, 1);

        w0.step(Way::E); // 1,1
        println!("{} at {}", w0.get_name(), w0.show_pos());
        assert_eq!(w0.body.pos_x, 1); assert_eq!(w0.body.pos_y, 1);

        w0.step(Way::S); // 1,0
        println!("{} at {}", w0.get_name(), w0.show_pos());
        assert_eq!(w0.body.pos_x, 1); assert_eq!(w0.body.pos_y, 0);

        w0.step(Way::W); // 0,0
        println!("{} at {}", w0.get_name(), w0.show_pos());
        assert_eq!(w0.body.pos_x, 0); assert_eq!(w0.body.pos_y, 0);

        w0.step(Way::NE); // 1,1
        println!("{} at {}", w0.get_name(), w0.show_pos());
        assert_eq!(w0.body.pos_x, 1); assert_eq!(w0.body.pos_y, 1);

        w0.step(Way::SE); // 2,0
        println!("{} at {}", w0.get_name(), w0.show_pos());
        assert_eq!(w0.body.pos_x, 2); assert_eq!(w0.body.pos_y, 0);

        w0.step(Way::NW); // 1,1
        println!("{} at {}", w0.get_name(), w0.show_pos());
        assert_eq!(w0.body.pos_x, 1); assert_eq!(w0.body.pos_y, 1);

        w0.step(Way::SW); // 0,0
        println!("{} at {}", w0.get_name(), w0.show_pos());
        assert_eq!(w0.body.pos_x, 0); assert_eq!(w0.body.pos_y, 0);
    }

    /** Simple Test if eating works. */
    pub fn test_wusel_eats() {
        let mut w0 = Wusel::xy(0, String::from("Test Eat"));

        let mut sushi = Food::new(String::from("Sushi"), 2, 1, 5, 2, 2);
        println!("Food: {}", sushi.show());

        sushi.body.place_at(0, 1);

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
