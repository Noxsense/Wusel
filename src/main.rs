extern crate rand;

use std::io;
// use std::io::{Read, Write, stdout, stdin};
// use termion::raw::IntoRawMode;

/** The main method of the wusel world. */
fn main() -> Result<(), io::Error> {
    let mut world: liv::World = liv::World::new(80, 30);
    println!(
        "Created a new world: w:{w}, h:{h}",
        w = world.get_width(),
        h = world.get_height()
    );

    /* Empty world tick. */
    world.tick();

    world.new_wusel("1st".to_string(), true, (0, 0)); // female
    world.new_wusel("2nd".to_string(), true, (20, 0)); // female
    world.new_wusel("3rd".to_string(), false, (30, 0)); // male
    world.new_wusel("4th".to_string(), false, (40, 0)); // male

    /* Create an easy talk, without any preconditions.
     * => no preconditions.
     * => does 'nothing' for ticks steps. */
    let reading: liv::TaskBuilder = liv::TaskBuilder::new(String::from("Reading"))
        // .set_passive_part(String::from("Any Book"))
        .set_duration(5 /*ticks*/);

    world.tick();

    // wusel.improve(liv::Ability::COOKING);
    // wusel.improve(liv::Ability::COMMUNICATION);
    // wusel.improve(liv::Ability::FITNESS);
    // wusel.improve(liv::Ability::FINESSE);

    println!("World Clock: {}", world.get_time());
    world.show_wusel_overview();
    for i in 0usize..4 {
        world.show_wusel_tasklist_for(i);
    }
    println!("\n\n");

    world.select_wusel(1);
    world.show_wusel_tasklist();

    println!("World Clock: {}", world.get_time());
    world.show_wusel_overview();
    for i in 0usize..4 {
        world.show_wusel_tasklist_for(i);
    }
    println!("\n\n");

    world.assign_task_to_wusel(0, reading.clone()); // do reading.

    world.tick();
    println!("\n\n"); // spent some time doing the task.

    world.abort_task_from_wusel(0, 0); // abort task
    world.assign_task_to_wusel(0, reading.clone()); // re-add reading.

    world.tick();
    println!("\n\n");

    // show state before rendering the world ever and ever again. \_(o_0)_/

    println!("World Clock: {}", world.get_time());

    world.show_wusel_overview(); // needs
    world.show_wusel_relations_for(world.get_selected_wusel()); // relations
    for i in 0usize..4 {
        world.show_wusel_tasklist_for(i);
    } // everyone's task list

    /* Every 500 ms, make a tick. */
    let duration = std::time::Duration::from_millis(500);

    /* Draw the field and make some real automation. */
    for _ in 0..100 {
        // world.recalculate_all_positions();
        draw_field(
            world.get_width() as usize,
            world.get_height() as usize,
            world.get_positions(),
        );

        /* Tick the world, maybe print the ongoing tasks. */
        print!("Time: {}\n", world.get_time());
        world.tick();

        /* Give some unbusy wusels the task to move around. */
        let unbusy = world.get_unbusy_wusels();
        let wusel_len = world.wusel_count();
        for widx in unbusy {
            let r = rand::random::<usize>() % (3 * wusel_len);
            match r {
                i if i < wusel_len && i != widx => {
                    /* Meet randomly with someone. */
                    world.assign_task_to_wusel(
                        widx,
                        liv::TaskBuilder::meet_with(i, true, true).set_duration(10),
                    );
                }
                i if i >= wusel_len && i < 2 * wusel_len => {
                    /* Walk randomly somewhere. */
                    let x = rand::random::<u32>() % world.get_width();
                    let y = rand::random::<u32>() % world.get_height();
                    world.assign_task_to_wusel(widx, liv::TaskBuilder::move_to((x, y)));
                }
                _ => {} // do nothing randomly.
            }
        }

        /* Draw selected wusel's needs (right position below field). */
        // print!("{pos}", pos = termion::cursor::Goto(
        //         world.get_width() as u16 - 22,
        //         world.get_height() as u16 + 3));
        // world.show_wusel_overview();

        std::thread::sleep(duration); // wait.
    }

    Ok(())
}

/** Clean he view and draw the field, put the cursor, two lines below the field, to write there. */
fn draw_field(w: usize, h: usize, positions: Vec<Vec<(char, usize)>>) {
    /* Draw field. */
    for p in 0..positions.len() {
        let on_pos = &positions[p];
        print!(
            "{pos}{sym}",
            pos = termion::cursor::Goto(
                (p % w) as u16 + 2, // x
                (p / w) as u16 + 2
            ), // y
            sym = if on_pos.len() < 1 { '`' } else { on_pos[0].0 }
        );
    }

    /* Draw border. */
    let mut i: u16 = 0;
    let w2: u16 = w as u16 + 2;
    let h2: u16 = h as u16 + 2;
    let around: u16 = (w2 * h2) as u16;
    while i < around {
        /* Draw symbol. */
        print!(
            "{pos}{border}",
            pos = termion::cursor::Goto(i % w2 + 1, i / w2 + 1),
            border = match i % w2 {
                _ if i == 0 || i == w2 - 1 || i == around - w2 || i == around - 1 => "+",
                0 => "|",
                x if x == (w2 - 1) => "|",
                _ => "=",
            }
        );
        /* Go around field. */
        i += if i < w2 || i >= around - w2 - 1 || i % w2 == w2 - 1 {
            1
        } else {
            w2 - 1
        };
    }

    /* Position to below field, clear everything below. */
    print!(
        "{pos_clear}{clear}{pos_then}",
        pos_clear = termion::cursor::Goto(1, h as u16 + 3),
        pos_then = termion::cursor::Goto(1, h as u16 + 4),
        clear = termion::clear::AfterCursor
    );
}

/** Test doing tasks. */
#[test]
fn test_creating_in_game() {
    println!("[test] Creating new stuff, let the wusels create stuff in their world.");
    let mut test_world: liv::World = liv::World::new(80, 30);

    /* Empty test_world tick. */
    test_world.tick();

    test_world.new_wusel("1st".to_string(), true, (1, 0)); // female
    test_world.new_wusel("2nd".to_string(), true, (2, 0)); // female
    test_world.new_wusel("3rd".to_string(), false, (3, 0)); // male
    test_world.new_wusel("4th".to_string(), false, (4, 0)); // male
    println!("World's wusel created.");

    // let food: liv::Consumable = liv::Consumable {};

    /* Get the food from food's position. */
    let get_the_food: liv::TaskBuilder = liv::TaskBuilder::new(String::from("Get the food"));

    /* Eat the held food. */
    let eat_the_food: liv::TaskBuilder = liv::TaskBuilder::new(String::from("Eating"));

    // TODO: Something can be a tool (to create, dependency) and a consumable (used up by usage)
    //
    // Example: Wusel wants to cook.
    // 1. Go to (free) cooking station: (move)
    // 2. Wait for the Station to be free
    // 3. Work on station.
    // 4. Fetch tomatoes to be cut and prepared (needs Tomatoes)
    // 5. Cut (consume) tomatoes, create sauce
    // 6. Heat up sauce. (> use up cold <? Consumable with extra states?)
    // 7. Creates hot tomato sauce. (can get cold or be eaten.)
    //
    // or should tools also be "Consumed" after 1M uses?
    // knife dull and then .. gone
    // couch is sit broken after several times?

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
    println!("[test] Mutual Meeting, causes for circular deadlocks.");
    let mut test_world: liv::World = liv::World::new(80, 30);

    /* Empty test_world tick. */
    test_world.tick();

    test_world.new_wusel("1st".to_string(), true, (1, 0)); // female
    test_world.new_wusel("2nd".to_string(), true, (3, 0)); // female
    test_world.new_wusel("3rd".to_string(), false, (5, 0)); // male
    test_world.new_wusel("4th".to_string(), false, (9, 0)); // male

    // 4 wusels created.
    assert_eq!(4, test_world.wusel_count());

    /* Create an easy talk, without any preconditions.
     * => no preconditions.
     * => does 'nothing' for ticks steps. */
    let reading: liv::TaskBuilder =
        liv::TaskBuilder::new(String::from("Reading")).set_duration(5 /*ticks*/);

    test_world.tick();

    // first wusel is also doing something else
    test_world.assign_task_to_wusel(0, reading.clone()); // do reading.

    // scenario: everyone wants too meet the next one.
    test_world.assign_task_to_wusel(
        0,
        liv::TaskBuilder::meet_with(1, true, false).set_duration(7),
    ); // mutual meeting.
    test_world.assign_task_to_wusel(
        1,
        liv::TaskBuilder::meet_with(2, true, false).set_duration(7),
    ); // mutual meeting.
    test_world.assign_task_to_wusel(
        2,
        liv::TaskBuilder::meet_with(3, true, false).set_duration(7),
    ); // mutual meeting.
    test_world.assign_task_to_wusel(
        3,
        liv::TaskBuilder::meet_with(0, true, false).set_duration(7),
    ); // mutual meeting.

    /* 90 ticks later. */
    for _ in 0..90 {
        test_world.tick();
        println!("\nTasks at time {}:", test_world.get_time());
        for w in 0..4 {
            test_world.show_wusel_tasklist_for(w);
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

mod liv {

    /** (Private) Wrapping Wusels and positions together. */
    struct WuselOnPosIdx { wusel: Wusel, position_idx: usize, }

    /** The place of existence, time and relations. */
    pub struct World {
        height: u32,
        width: u32,
        positions: Vec<Vec<(char, usize)>>, // all positions [height x width] contain a vector of ids and type/set indicators.

        clock: usize, // time of the world.

        // wusels the main live force in here
        wusels_created: usize, // all wusels ever created. => for each wusel identifier, maybe a few are available, but already the Xth generation
        wusels: Vec<WuselOnPosIdx>, // vector of { wusels, their positions }
        wusel_selected: usize, // currently selected wusel

        relations: std::collections::BTreeMap<(usize, usize), Relation>, // vector of wusel relations
    }

    impl World {
        /** Create a new world. */
        pub fn new(width: u32, height: u32) -> Self {
            return Self {
                height: height,
                width: width,
                positions: vec![vec![]; width as usize * height as usize],
                clock: 0,
                wusels_created: 0,
                wusels: vec![],
                wusel_selected: 0,
                relations: std::collections::BTreeMap::new(),
            };
        }

        pub const TICKS_PER_DAY: usize = 2880; // 24h by 0.5 minutes

        /** Get width of the world. */
        pub fn get_width(self: &Self) -> u32 {
            self.width
        }

        /** Get height of the world. */
        pub fn get_height(self: &Self) -> u32 {
            self.height
        }

        pub fn get_time(self: &Self) -> usize {
            self.clock
        }

        const CHAR_WUSEL: char = 'w';

        /** Add a wusel to the world.
         * ID is the current wusel count.
         * TODO (2020-11-20) what is about dead wusels and decreasing length? */
        pub fn new_wusel(self: &mut Self, name: String, female: bool, pos: (u32, u32)) {
            let id = self.wusels_created; // almost identifier (for a long time unique)
            let w = Wusel::new(id, name, female); // new wusel at (pos)

            /* Add wusel to positions, start at (pos). */
            let pos_idx = self.pos_to_idx(pos);
            if pos_idx < self.positions.len() {
                self.positions[pos_idx].push((Self::CHAR_WUSEL, w.id));
            }

            self.wusels.push(WuselOnPosIdx{ wusel: w, position_idx: pos_idx }); // wusel on position (by index)
            // self.wusels_positions.push(pos_idx); // index.
            self.wusels_created += 1;
        }

        /** Count how many wusels are currently active. */
        pub fn wusel_count(self: &Self) -> usize {
            self.wusels.len()
        }

        /** Select a wusel by index/living count. */
        pub fn select_wusel(self: &mut Self, selection: usize) {
            self.wusel_selected = usize::min(selection, self.wusels.len());
        }

        /** Get the index of the wusel, which is currently selected. */
        #[allow(dead_code)]
        pub fn get_selected_wusel(self: &Self) -> usize {
            self.wusel_selected
        }

        /** Get the indices of all wusels, which are currently having no tasks to do. */
        pub fn get_unbusy_wusels(self: &Self) -> Vec<usize> {
            let mut unbusy: Vec<usize> = vec![];
            for i in 0..self.wusels.len() {
                if self.wusels[i].wusel.tasklist.len() < 1 {
                    unbusy.push(i);
                }
            }
            return unbusy;
        }

        /** Give an available wusel (by index) a new task. */
        pub fn assign_task_to_wusel(self: &mut Self, wusel_index: usize, taskb: TaskBuilder) {
            if wusel_index < self.wusels.len() {
                /* Task apply wusel[index] as actor. */
                self.wusels[wusel_index].wusel.add_task(self.clock, taskb);
            }
        }

        /** Abort an assigned task from an available wusel (by index). */
        pub fn abort_task_from_wusel(self: &mut Self, wusel_index: usize, task_index: usize) {
            if wusel_index < self.wusels.len() {
                /* Remove task. */
                self.wusels[wusel_index].wusel.abort_task(task_index);
            }
        }

        /** Print overview of (selected) wusel to std::out.*/
        pub fn show_wusel_overview(self: &Self) {
            self.show_wusel_overview_for(self.wusel_selected);
        }

        /** Print overview of (selected) wusel to std::out.*/
        pub fn show_wusel_overview_for(self: &Self, wusel_index: usize) {
            /* No wusel is there to show. */
            if self.wusels.len() <= wusel_index {
                println!("There is no wusel to show.");
                return;
            }
            println!("{}", self.wusels[wusel_index].wusel.show_overview());
        }

        /** Print tasklist of (selected) wusel to std::out.*/
        pub fn show_wusel_tasklist(self: &Self) {
            self.show_wusel_tasklist_for(self.wusel_selected);
        }

        /** Print tasklist of (selected) wusel to std::out.*/
        pub fn show_wusel_tasklist_for(self: &Self, wusel_index: usize) {
            if wusel_index >= self.wusels.len() {
                println!("There is no wusel to show.");
                return;
            }
            println!(
                "Tasks of {}: {}",
                self.wusels[wusel_index].wusel.get_name(),
                self.wusels[wusel_index].wusel.show_takslist()
            );
        }

        /** Show all relations for a wusel, given by index.
         * Prints directly to std::out. */
        pub fn show_wusel_relations_for(self: &Self, wusel_index: usize) {
            if wusel_index >= self.wusels.len() {
                println!("There is no wusel to show.");
                return;
            }

            let wusel_id = self.wusels[wusel_index].wusel.id;

            print!("Relations with {}: ", self.wusels[wusel_index].wusel.get_name());

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

                let other_name = self.wusels[other_id].wusel.get_name();

                /* Print Relation. */
                print!("[{:?}: {}]", other_name, relation.show());
                has_relations = true;
            }

            if !has_relations {
                print!("Has no relations.");
            }

            println!("");
        }

        /** Get an index for the wusel with the requesting index.
         * Return LEN, if none is found. */
        fn wusel_identifier_to_index(self: &Self, id: usize) -> usize {
            for i in 0..self.wusels.len() {
                if self.wusels[i].wusel.id == id {
                    return i; // return matching id.
                }
            }
            return self.wusels.len();
        }

        /** Check if the identifier for a requesting wusel is currently active. */
        #[allow(dead_code)]
        fn is_wusel_identifier_active(self: &Self, id: usize) -> bool {
            return self.wusel_identifier_to_index(id) < self.wusels.len();
        }

        /** Get the position of the indexed wusel. */
        pub fn get_wusel_position(self: &Self, wusel_index: usize) -> (u32, u32) {
            if wusel_index < self.wusels.len() {
                self.idx_to_pos(self.wusels[wusel_index].position_idx)
            } else {
                (self.width, self.height) // outside the map.
            }
        }

        /** Set the position of the indexed wusel to the nearest valid position
         * If the position may land out of the grid, put it to the nearest border. */
        pub fn set_wusel_position(self: &mut Self, wusel_index: usize, pos: (u32, u32)) {
            if wusel_index < self.wusels.len() {
                let id = self.wusels[wusel_index].wusel.id;

                /* Update the self.positions. */
                let old_pos_idx = self.wusels[wusel_index].position_idx;

                let new_pos = (u32::min(pos.0, self.width), u32::min(pos.1, self.height));
                let new_pos_idx = self.pos_to_idx(new_pos);

                /* Set the new position. */
                self.wusels[wusel_index].position_idx = new_pos_idx;

                /* Representation in positions. */
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
        pub fn get_all_wusels_positions(self: &Self) -> Vec<(u32, u32)> {
            let mut positions = vec![];
            for w in self.wusels.iter() {
                positions.push(self.idx_to_pos((*w).position_idx)); // usize -> (u32, u32)
            }
            return positions;
        }

        /** Check all positions.
         * Recalculate all positions, if they really consist what they promised. */
        #[allow(dead_code)]
        pub fn recalculate_all_positions(self: &mut Self) {
            self.positions = vec![vec![]; self.width as usize * self.height as usize];

            let valid_idx = self.positions.len();

            let mut wusel_idx = 0usize;
            for w in self.wusels.iter() {
                let idx = self.wusels[wusel_idx].position_idx;
                wusel_idx += 1;

                /* Add id to position. */
                if idx < valid_idx {
                    self.positions[idx].push((Self::CHAR_WUSEL, w.wusel.id));
                }
            }
        }

        /** Get the `positions` index for the requesting position.
         * If the position is not in world, this index is not in [0,positions.len()).*/
        fn pos_to_idx(self: &Self, pos: (u32, u32)) -> usize {
            (pos.0 + self.width * pos.1) as usize
        }

        /** Get the position tuple from the given index in this world. */
        fn idx_to_pos(self: &Self, idx: usize) -> (u32, u32) {
            (idx as u32 % self.width, idx as u32 / self.width)
        }

        /** Get all the positions as they are. */
        pub fn get_positions(self: &Self) -> Vec<Vec<(char, usize)>> {
            self.positions.clone()
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
            for w in self.wusels.iter_mut() {
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
                        println!("Pop the baby!");
                        let female = rand::random::<bool>();
                        new_babies.push((w.wusel.id, father, female));
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
                if let Some(t) = self.wusels[*w].wusel.peek_ongoing_task() {
                    /* Decide how to progress the command. */
                    let u = (*t).clone();
                    self.proceed(u);
                }
            }

            for _ in self.relations.iter() { /* Decay of relations over time. */ }

            /* Command further name giving and attention from the player. */
            for baby in new_babies.iter() {
                println!(
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

            let wusel_size = self.wusels.len();
            let actor_id = task.active_actor_id;
            let actor_index = self.wusel_identifier_to_index(actor_id);

            if actor_index >= wusel_size {
                return; // abort, because actor unavailable
            }

            let start_time = match task.started {
                true => task.start_time,
                false => {
                    /* Notify the start of the task (for the wusel). */
                    self.wusels[actor_id].wusel.start_ongoing_task(self.clock);

                    self.clock // starting now
                }
            };

            /* Decide what to do, and if the task case done a step. */
            let succeeded = match task.passive_part {
                TaskTag::WaitLike => {
                    println!("{}", task.name);
                    true
                }
                TaskTag::BeMetFrom(other_id) => {
                    let other_index = self.wusel_identifier_to_index(other_id);

                    /* Other wusel needs also to exist or still wants to meet.
                     * Otherwise pop. */

                    match other_index {
                        /* => proceed, since the other party was invalid. */
                        invalid_index if invalid_index >= self.wusels.len() => true,

                        /* Meeting party is valid, check their ongoing task. */
                        _ => match self.wusels[other_index].wusel.peek_ongoing_task() {
                            /* => Proceed, since the other party is doing nothing, so no meeting. */
                            None => true,

                            /* Other party is busy. */
                            Some(t) => match t.passive_part {
                                /* => Do not end (proceed), since the other party is still meeting with this actor. */
                                TaskTag::MeetWith(id, _nice, _love) if id == actor_id => false,

                                /* => proceed task, other party is busy with sth else. */
                                _ => true,
                            },
                        },
                    }
                }
                TaskTag::MeetWith(other_id, nice, romantically) => {
                    let other_index = self.wusel_identifier_to_index(other_id);

                    /* Other wusel needs also to exist. */
                    if other_index >= self.wusels.len() {
                        self.wusels[actor_index].wusel.pop_ongoing_task();
                        return; // task can not be done, without target.
                    }

                    /* Check all preconditions, maybe solve one and maybe do the actually meeting.
                     * 0, when they met, like the C-ish "OK".
                     * 1, when the actor walked.
                     * 2, when the actual knocking was just applied.
                     * 3, when the knocking was done, but the passive is still busy.
                     */
                    let meeting_result =
                        self.let_two_wusels_meet(actor_index, other_index, nice, romantically);

                    /* On Final Success with own step,
                     * also let the BeMetFrom() succeed. */

                    match meeting_result {
                        // waiting, but don't wait too long.
                        Self::MEET_RESULT_WAITED => {
                            if self.clock - start_time >= Task::PATIENCE_TO_MEET {
                                self.wusels[actor_index].wusel.pop_ongoing_task();
                            }
                            false // => do not notify succession
                        }

                        /* They met and the task is over. */
                        Self::MEET_RESULT_OK => true, // => notify process
                        _ => false, // => no process (FOLLOWED, KNOCKED or an unexpected)
                    }
                }
                TaskTag::MoveToPos(x, y) => {
                    /* Let the wusel walk; check if they stopped. */
                    let stopped: bool = self.let_wusel_walk_to_position(actor_index, (x, y));

                    stopped // true == stop == success.
                }
                TaskTag::GetFromPos(x, y) => {
                    println!("Pick from Goal: ({}, {}).", x, y);
                    // == MoveToPos(x,y) && pick up on field?
                    // XXX implement.
                    true
                }
                TaskTag::PutAtPos(x, y) => {
                    println!("Drop at Goal: ({}, {}).", x, y);
                    // == MoveToPos(x,y) && drop up on field?
                    // XXX implement.
                    true
                }
            };

            /* Notify the task succeeded to do a step. */
            if succeeded {
                self.wusels[actor_index].wusel.notify_ongoing_succeeded();
            }
        }

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
            println!(
                "Meet with {}, nice: {}.",
                self.wusels[passive_index].wusel.show(),
                intention_good
            );

            /* If not close to the other wusel, use this step to get closer,
             * return as not yet ready. */
            let pos_a = self.get_wusel_position(active_index);
            let pos_o = self.get_wusel_position(passive_index);

            println!("Meet at {:?} and {:?}", pos_a, pos_o);

            /* If they are not close, try to get closer to the passive target. */
            if Self::get_distance_between(pos_a, pos_o) > 2.0 {
                /* Try to get closer, create a sub task walking to the current target's
                 * position. */
                // XXX if real path finding is implemented this will always calculate the path there, even if the target moves.
                println!("Follow to {:?}.", pos_o);
                self.let_wusel_walk_to_position(active_index, pos_o);
                return Self::MEET_RESULT_FOLLOWED;
            }

            let active_id = self.wusels[active_index].wusel.id;
            let passive_id = self.wusels[passive_index].wusel.id;

            /* Get the passive wusel's current task.
             * If it is being met by the active, succeed a step with the meeting,
             * otherwise see below. */
            let passives_ongoing_tasktag: Option<TaskTag> =
                if let Some(t) = self.wusels[passive_index].wusel.peek_ongoing_task() {
                    Some(t.passive_part)
                } else {
                    None
                };

            let active_is_met = TaskTag::BeMetFrom(active_id);

            let handshake_okay = match &passives_ongoing_tasktag {
                Some(tag) if *tag == active_is_met => true,
                _ => false,
            };

            if handshake_okay {
                let performance: bool; // how well is the communication

                performance = true;
                // random influence of 10%
                // current value and intention
                // communication ability

                /* Update the relation between active and passive. */
                self.update_wusel_relations(
                    active_id,
                    passive_id,
                    intention_good && performance,
                    romantically && performance,
                );

                return Self::MEET_RESULT_OK; // they actually met.
            }

            /* Check, if the passive is already waiting (in tasklist). */
            let passive_is_waiting = self.wusels[passive_index].wusel.has_task_with(active_is_met);

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
                    _a if self.wusels[active_index].wusel
                        .has_task_with(TaskTag::BeMetFrom(passive_id)) =>
                    {
                        active_index
                    }
                    _ => self.wusels.len(),
                };

                /* Move already waiting task to active tasks. */
                if already_waiting_index < self.wusels.len() {
                    /* What happens:
                     * A: [Talk B, Task A2, Task A3]
                     * B: [Talk A, Task B3, Listen A] // B already knows.
                     * ----
                     * A: [Talk B, Task A2, Task A3]
                     * B: [Listen A, Talk A, Task B2, Task B3] // let B listen first.
                     */

                    let mut i = self.wusels[already_waiting_index].wusel.tasklist.len();
                    while i > 0 {
                        i -= 1;
                        if self.wusels[already_waiting_index].wusel.tasklist[i].passive_part
                            == active_is_met
                        {
                            let met_task = self.wusels[already_waiting_index].wusel.tasklist.remove(i);
                            self.wusels[already_waiting_index].wusel.tasklist.push(met_task); // append to back (ongoing)
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
                let c0 = self.wusels[active_index].wusel.get_ability(&skill);
                let c1 = self.wusels[passive_index].wusel.get_ability(&skill);

                let (more_active, more_passive) = match c0 {
                    better if better > c1 => (active_index, passive_index),
                    worse if worse < c1 => (passive_index, active_index),
                    _tie if active_index < passive_index => (active_index, passive_index),
                    _ => (passive_index, active_index),
                };

                self.assign_task_to_wusel(
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
                self.assign_task_to_wusel(passive_index, TaskBuilder::be_met_from(active_id));
                return Self::MEET_RESULT_KNOCKED;
            }

            /* If the passive target is not yet ready to talk, wait.  */
            return Self::MEET_RESULT_WAITED;
        }

        /** Let the wusel walk to a position.
         * If the path is already calculated, let it walk the pre-calculated path.
         * If not, calculate a new path.
         * If an obstacle occurs, recalculate the path and continue walking.
         * If the goal, is reached, the walk is done.
         *
         * #Return, if wusel has stopped walking / is on goal (true), otherwise true, if they are still walking. */
        fn let_wusel_walk_to_position(
            self: &mut Self,
            wusel_index: usize,
            goal: (u32, u32),
        ) -> bool {
            let pos = self.get_wusel_position(wusel_index);

            /* Check if the goal is already reached. */
            if pos.0 == goal.0 && pos.1 == goal.1 {
                println!("Reached Goal ({},{}).", goal.0, goal.1);
                return true; // stopped walking.
            }

            println!("Move to Goal {:?}.", goal);

            /* Check, if the pre-calculated path is blocked. */
            if false { /* Abort the pre-calculated, but blocked path. */ }

            /* Check, if the path is (still) pre-calculated. */
            if true {
                /* Walk the path. */
                // XXX easy placeholder walking, ignoring all obstacles.

                /* Get the current positions neighbours. */
                let neighbours = Self::get_neighbour_positions(self.width, self.height, pos);

                if neighbours.len() < 1 {
                    println!("Wusel cannot move, it's enclosed, wait forever");
                    return true;
                }

                let goal: (u32, u32) = (goal.0, goal.1);
                let mut closest: (u32, u32) = neighbours[0];
                let mut closest_distance: f32 = f32::MAX;

                /* Find closest neighbour to goal. */
                for p in neighbours.iter() {
                    let distance = Self::get_distance_between(goal, *p);

                    if distance < closest_distance {
                        closest = *p;
                        closest_distance = distance;
                    }
                }

                /* move to closest position. */
                self.set_wusel_position(wusel_index, closest);
                return false; // still walking.
            } else {
                /* Calculate the path and go it next time. */
                println!("Calculate the path to {:?}", goal);
                return false; // still walking.
            }
        }

        /** Get the (valid) neighbours for a position. */
        pub fn get_neighbour_positions(
            box_width: u32,
            box_height: u32,
            pos: (u32, u32),
        ) -> Vec<(u32, u32)> {
            let mut neighbours: Vec<(u32, u32)> = vec![];

            /* Get all the valid neighbours. */
            for d in Way::NEIGHBOURING.iter() {
                if let Some(n) = Self::get_neighbour_on(box_width, box_height, pos, *d) {
                    neighbours.push(n);
                }
            }

            return neighbours;
        }

        /** Get the next optional neighbour to the given position within the given box. */
        pub fn get_neighbour_on(
            box_width: u32,
            box_height: u32,
            pos: (u32, u32),
            direction: Way,
        ) -> Option<(u32, u32)> {
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

            return Some((
                (pos.0 as i64 + change.0 as i64) as u32,
                (pos.1 as i64 + change.1 as i64) as u32,
            ));
        }

        /** Get the distance between two positions. */
        pub fn get_distance_between(a: (u32, u32), b: (u32, u32)) -> f32 {
            (((a.0 as i64 - b.0 as i64).pow(2) + (a.1 as i64 - b.1 as i64).pow(2)) as f32).sqrt()
        }

        /** Update the relation of two wusels, given by their id. */
        pub fn update_wusel_relations(
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

    /** A place which a creator needs to produce stuff. */
    struct CreationStation {
        position: (u32, u32),
        occupied_by: usize,
    }

    /** Requirements and further steps, to create the requested consumable.
     * Requirements, such as a certain ability and items.
     * Steps, such as use station A, consume ingredient 1 on Station B, Wait. */
    struct Recipe {
        goal: Consumable,
        // requirements
        difficulty: (Ability, u32),
        ingredients: Vec<Consumable>,
        // steps, what to do and in which order
        steps: usize,
        // current progress and quality of the recipe
        completed_steps: usize,
        quality: i32,
    }

    // XXX 2020-12-04 Thing making (like Cooking, Crafting, etc.)
    impl Recipe {
        /** Uses up the recipe and get the result. */
        fn finish_with_result(self: Self) -> Option<Consumable> {
            return if self.completed_steps < self.steps {
                /* Recipe was not done yet. Abort and get nothing. */
                None
            } else if (self.quality) < 10 {
                /* Bad Performance (below 10%) leads to No Result. */
                None
            } else if (rand::random::<u16>() % 100) < 5 {
                /* Some random possibility, this whole task still fails. */
                None
            } else {
                Some(self.goal)
            };
        }

        /** Check, if the given Wusel is qualified for the task. */
        fn is_qualified(self: &Self, wusel: &Wusel) -> bool {
            wusel.get_ability(&self.difficulty.0) >= self.difficulty.1
        }

        /** Mark another step as completed.
         * Get a random quality change (-10,10) percent. */
        fn succeed(self: &mut Self) {
            self.completed_steps += 1;
            self.quality += rand::random::<i32>() % 10;
            // reduce by difficulty differences
        }

        /** Try to translate the current step into a task. */
        fn get_task_for_current_step(self: &Self) {}
    }

    /** Something a Wusel can consume (= destroying by usage).
     * Consuming it might modify the needs and skills. */
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

    impl Consumable {
        fn new(
            name: String,
            size: u32,
            energy_per_bite: u32,
            food_per_bite: u32,
            water_per_bite: u32,
            spoils: u32,
        ) -> Self {
            Self {
                name: name,

                size: size,        // => 0.5 per minute
                available: 1.0f32, // start fully.

                spoils_after: spoils, // after days, 0: does not spoil.
                age: 0,

                need_change: std::collections::HashMap::new(),
            }
        }

        /** Render a nice String to show the food, it's availability and spoilage. */
        fn show(self: &Self) -> String {
            let maybe_spoiled = if self.is_spoiled() { " (spoiled)" } else { "" };
            let avail: i32 = (self.available * 1000f32) as i32;
            return match avail {
                0 => format!("Eaten {} (fully gone)", self.name),
                1000 => format!("{}{}", self.name, maybe_spoiled),
                _ => format!("{} {}{}", self.available, self.name, maybe_spoiled),
            };
        }

        /** Check if the food is already spoiled. */
        fn is_spoiled(self: &Self) -> bool {
            self.age >= self.spoils_after
        }

        /** Get the size of one bite. */
        fn bite_part(self: &Self) -> f32 {
            1.0 / self.size as f32
        }

        /** Take a bite: Decrease availability by one bite.
         * @return true, if the bite was successful. */
        fn reduce(self: &mut Self) -> bool {
            if self.available > 0.0 {
                self.available -= self.bite_part(); // remove a part.
                true
            } else {
                false
            }
        }
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
    #[derive(Copy, Clone, PartialEq, PartialOrd)]
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

        fn name(self: &Self) -> &str {
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

        fn get_default_decay(self: &Self) -> u32 {
            for i in 0..Self::VALUES.len() {
                if self == &Self::VALUES[i] {
                    return Self::DEFAULT_NEED_DECAY_PER_MINUTE[i];
                }
            }
            return 0; // default: no decay.
        }
    }

    /** An ability, the Wusel can learn to improve their lifestyle. */
    #[derive(Copy, Clone, PartialEq)]
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
        pub fn move_to(pos: (u32, u32)) -> Self {
            Self {
                name: "Moving".to_string(),
                duration: 1,
                passive_part: TaskTag::MoveToPos(pos.0, pos.1),
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

        active_actor_id: usize, // wusel id.
        passive_part: TaskTag,  // position | object-to-be | object | wusel | nothing.
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum TaskTag {
        WaitLike,
        MoveToPos(u32, u32),
        GetFromPos(u32, u32),        // pick up a thing from position.
        PutAtPos(u32, u32),          // drop something at position.
        MeetWith(usize, bool, bool), // commute with another wusel (id)
        BeMetFrom(usize),            // be met by another wusel (id)
    }

    impl Task {
        const PATIENCE_TO_MEET: usize = 20; // TODO
        /** Get the approximately rest time (in ticks), this task needs. */
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

        female: bool, // female => able to bear children, male => able to inject children
        pregnancy: Option<(usize, u8)>, // optional pregnancy with father's id and remaining days.

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
        fn new(id: usize, name: String, female: bool) -> Self {
            let mut new = Self {
                id: id,
                name: name,

                female: female,
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
            string.push_str("d)"); // days

            return string;
        }

        /** Show collected data. */
        fn show_overview(self: &Self) -> String {
            let mut s = format!("==={:=<40}\n", "");

            s += &format!("  {}\n", self.show());

            /* Show needs. */
            s += &format!("---{:-<40}\n", " NEEDS: ");
            s += &self.show_needs();

            /* Show abilities. */
            s += &format!("---{:-<40}\n", " ABILITIES: ");
            s += &self.show_abilities();

            /* Show relations. */
            // TODO (2020-11-16) show relations.
            s += &format!("{:_<43}\n", "");

            return s;
        }

        /** Print the tasklist (as queue). */
        fn show_takslist(self: &Self) -> String {
            let n = self.tasklist.len();
            if n < 1 {
                return String::from("Nothing to do.");
            }
            let mut s = String::new();
            let mut i = n - 1;
            loop {
                /* Write task: [Activity Name], */
                s += &format!(
                    "[{active}{name} {progress}/{duration}]",
                    active = if i == n - 1 { "#" } else { "" },
                    name = self.tasklist[i].name,
                    progress = self.tasklist[i].done_steps,
                    duration = self.tasklist[i].duration
                );

                /* Break or next task, if available. */
                if i == 0 {
                    break;
                } else {
                    s.push_str(", ");
                    i -= 1;
                }
            }
            return s;
        }

        /** Show all assigned needs. */
        fn show_needs(self: &Self) -> String {
            let mut s = String::new();
            for (n, v) in self.needs.iter() {
                let full = Self::default_need_full(n);

                let max_len = 20;
                let bar_len = (*v * max_len / full) as usize;

                s += &format!(
                    " {name:>14} {value:5} {end:.>bar_len$} \n",
                    name = n.name(),
                    value = v,
                    bar_len = usize::min(bar_len, max_len as usize),
                    end = ""
                );
            }
            return s;
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
        fn add_need(self: &mut Self, need: Need, change_value: u32) -> u32 {
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
        fn has_task_with(self: &Self, task_tag: TaskTag) -> bool {
            for t in self.tasklist.iter() {
                if t.passive_part == task_tag {
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

        /** Eat Consumable, take one bite.
         * This may use up the food. It may update the needs. */
        fn eat(self: &mut Self, food: &mut Consumable) {
            /* Take a bite. */
            let successfully_fed: bool = food.reduce();

            if successfully_fed {
                // self.add_need(Need::SLEEP, food.need_change.get(Ability::SLEEP));
                // self.add_need(Need::FOOD, food.satisfied_food);
                // self.add_need(Need::WATER, food.satisfied_water);
            }
        }

        /** Check, if the wusel is pregnant. */
        #[allow(dead_code)]
        pub fn is_pregnant(self: &Self) -> bool {
            return self.pregnancy != None;
        }

        /** Get the remaining days of an possible Pregnancy. */
        #[allow(dead_code)]
        pub fn get_remaining_pregnancy_days(self: &Self) -> Option<u8> {
            match self.pregnancy {
                Some((_father, days)) => Some(days),
                _ => None,
            }
        }
    }

    /** Simple Test if eating works.
     * XXX implement foe task and remove or put to test. */
    #[test]
    pub fn test_wusel_eats() {
        let mut w0 = Wusel::new(0, String::from("Test Eat"), false);

        let mut sushi = Consumable::new(String::from("Sushi"), 2, 1, 5, 2, 2);
        println!("Consumable: {}", sushi.show());

        assert!(available1 >= 1.0);

        w0.eat(&mut sushi);
        w0.show_overview();

        println!("Consumable: {}", sushi.show());

        w0.eat(&mut sushi);
        w0.show_overview();

        println!("Consumable: {}", sushi.show());

        w0.eat(&mut sushi);
        w0.show_overview();

        println!("Consumable: {}", sushi.show());
    }
}
