extern crate rand;

use std::io;
// use std::io::{Read, Write, stdout, stdin};
// use termion::raw::IntoRawMode;

// TODO (2020-11-22) TUI/CUI: Look at termion: https://github.com/redox-os/termion/tree/master/examples
// TODO (2020-11-22) TUI/CUI: Look at termion: https://lib.rs/crates/termion
// TODO (2020-11-22) TUI/CUI: Look at termion: https://github.com/redox-os/games/blob/master/src/minesweeper/main.rs https://raw.githubusercontent.com/redox-os/games/master/src/minesweeper/main.rs
// TODO (2020-11-22) TUI/CUI: Look at tui (backend termion): https://docs.rs/tui/0.13.0/tui/index.html

// TODO (2020-11-22) Meeting with someone: Need to be close to the one.

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

    /* Create an easy talk, without any preconditions.
     * => no preconditions.
     * => does 'nothing' for ticks steps. */
    let reading: liv::TaskBuilder = liv::TaskBuilder::new(
        String::from("Reading"))
        // .set_passive_part(String::from("Any Book"))
        .set_passive_part(liv::TaskTag::WaitLike)
        .set_duration(5 /*ticks*/);

    /* Create walking task. (runs until on goal or aborted)
     * => condition: is able to walk, not on the goal.
     * => position is now the goal. */
    let mut walking: liv::TaskBuilder = liv::TaskBuilder::new(
        String::from("Walking"))
        // .set_passive_part(String::from("Any Book"))
        .set_passive_part(liv::TaskTag::MoveToPos(10, 10))
        .set_duration(1 /*ticks*/);

    /* Create talking task
     * => the other wusel needs to exist, needs to be close, then until the time ran out.
     * => outcome: changed relation and maybe improved ability. */
    let talking: liv::TaskBuilder = liv::TaskBuilder::new(
        String::from("Talking (friendly)"))
        // .set_passive_part(String::from("Any Book"))
        .set_passive_part(liv::TaskTag::MeetWith(1, true, false))
        .set_duration(10 /*ticks*/);

    world.tick();

    // wusel.improve(liv::Ability::COOKING);
    // wusel.improve(liv::Ability::COMMUNICATION);
    // wusel.improve(liv::Ability::FITNESS);
    // wusel.improve(liv::Ability::FINESSE);

    println!("World Clock: {}", world.get_time());
    world.show_wusel_overview();
    for i in 0usize..4 { world.show_wusel_tasklist_for(i); }
    println!("\n\n");

    world.select_wusel(1);
    world.show_wusel_tasklist();

    println!("World Clock: {}", world.get_time());
    world.show_wusel_overview();
    for i in 0usize..4 { world.show_wusel_tasklist_for(i); }
    println!("\n\n");

    world.assign_task_to_wusel(0, reading.clone());
    world.tick();
    println!("\n\n");

    world.assign_task_to_wusel(1, reading.clone()); // 1: read
    world.assign_task_to_wusel(1, walking.clone()); // 1: then run away
    world.assign_task_to_wusel(2, talking.clone()); // 2: talk with 1 | 1: be then contacted by 2
    world.tick();
    println!("\n\n");

    world.abort_task_from_wusel(0, 0);
    world.tick();
    println!("\n\n");

    world.assign_task_to_wusel(0, reading.clone());

    println!("World Clock: {}", world.get_time());
    world.show_wusel_overview();
    for i in 0usize..4 { world.show_wusel_tasklist_for(i); }

    /* Spend time until almost frozen. */
    for _ in 0..900 { world.tick(); }

    println!("World Clock: {}", world.get_time());
    world.show_wusel_overview();
    for i in 0usize..4 { world.show_wusel_tasklist_for(i); }

    for i in 0usize..4 { world.show_relations_for(i); }

    print!("\n");

    let duration = std::time::Duration::from_millis(500);

    /* Draw the field and make some real automation. */
    while false {
        // world.recalculate_all_positions();
        draw_field(world.get_width() as usize, world.get_height() as usize, world.get_positions());

        /* Tick the world, maybe print the ongoing tasks. */
        print!("Time: {}\n", world.get_time());
        world.tick();

        /* Give some unbusy wusels the task to move around. */
        let unbusy = world.get_unbusy_wusels();
        for widx in unbusy {
            if rand::random::<bool>() {
                /* Walk randomly somewhere. */
                walking = walking.set_passive_part(liv::TaskTag::MoveToPos(
                        rand::random::<u32>() % world.get_width(),
                        rand::random::<u32>() % world.get_height()));
                world.assign_task_to_wusel(widx, walking.clone());
            }
        }

        std::thread::sleep(duration); // wait.
    }

    Ok(())
}

/** Clean he view and draw the field, put the cursor, two lines below the field, to write there. */
fn draw_field(w: usize, h: usize, positions: Vec<Vec<(char, usize)>>) {
    /* Draw field. */
    for p in 0..positions.len() {
        let on_pos = &positions[p];
        print!("{pos}{sym}",
               pos = termion::cursor::Goto(
                   (p % w) as u16 + 2, // x
                   (p / w) as u16 + 2), // y
               sym = if on_pos.len() < 1 { '`' } else { on_pos[0].0 });
    }

    /* Draw border. */
    let mut i: u16 = 0;
    let w2: u16 = w as u16 + 2;
    let h2: u16 = h as u16 + 2;
    let around: u16 = (w2 * h2) as u16;
    while i < around {
        /* Draw symbol. */
        print!("{pos}{border}",
               pos = termion::cursor::Goto(i % w2 + 1, i / w2 + 1),
               border = match i % w2 {
                   _ if i == 0 || i == w2 - 1 || i == around - w2 || i == around - 1 => "+",
                   0 => "|",
                   x if x == (w2-1) => "|",
                   _ => "=",
               });
        /* Go around field. */
        i += if i < w2 || i >= around - w2 -1 || i % w2 == w2 - 1 { 1 } else {w2 - 1 };
    }

    /* Positiion to below field, clear everything below. */
    print!("{pos_clear}{clear}{pos_then}",
           pos_clear = termion::cursor::Goto(1, h as u16 + 3),
           pos_then = termion::cursor::Goto(1, h as u16 + 4),
           clear = termion::clear::AfterCursor);
}

///////////////////////////////////////////////////////////////////////////////

mod liv {
    /** The place of existence, time and relations. */
    pub struct World {
        height: u32,
        width: u32,
        positions: Vec<Vec<(char, usize)>>, // all positions [height x width] contain a vector of ids and type/set indicators.

        clock: usize, // time of the world.

        wusels: Vec<Wusel>, // vector of wusels
        wusels_created: usize, // all wusels ever created. => for each wusel identifier.
        wusel_selected: usize, // currently selected wusel

        relations: std::collections::BTreeMap<(usize,usize), Relation>, // vector of wusel relations
    }

    impl World {
        /** Create a new world. */
        pub fn new(width: u32, height: u32) -> Self {
            return Self{
                height: height, width: width,
                positions: vec![vec![]; width as usize * height as usize],
                clock: 0,
                wusels: vec![],
                wusels_created: 0,
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
        pub fn new_wusel(self: &mut Self, name: String, female: bool) {
            let id = self.wusels_created; // almost identifier (for a long time unique)
            let w = Wusel::new(id, name, female); // new wusel at (0,0)

            /* Add wusel to positions, start at 0. */
            let pos_idx = self.pos_to_idx(w.get_position());
            if pos_idx < self.positions.len() {
                self.positions[pos_idx].push((Self::CHAR_WUSEL, w.id));
            }

            self.wusels.push(w);
            self.wusels_created += 1;
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
                if self.wusels[i].tasklist.len() < 1 {
                    unbusy.push(i);
                }
            }
            return unbusy;
        }

        /** Give an available wusel (by index) a new task. */
        pub fn assign_task_to_wusel(self: &mut Self, wusel_index: usize, taskb: TaskBuilder) {
            if wusel_index < self.wusels.len() {
                /* Task apply wusel[index] as actor. */
                self.wusels[wusel_index].add_task(taskb);
            }
        }

        /** Abort an assigned task from an available wusel (by index). */
        pub fn abort_task_from_wusel(self: &mut Self, wusel_index: usize, task_index: usize) {
            if wusel_index < self.wusels.len() {
                /* Remove task. */
                self.wusels[wusel_index].abort_task(task_index);
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
            println!("{}", self.wusels[wusel_index].show_overview());
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
            println!("Tasks of {}: {}",
                     self.wusels[wusel_index].get_name(),
                     self.wusels[wusel_index].show_takslist());
        }

        pub fn show_relations_for(self: &Self, wusel_index: usize) {
            if wusel_index >= self.wusels.len() {
                println!("There is no wusel to show.");
                return;
            }

            let wusel_id = self.wusels[wusel_index].id;

            print!("Relations with {}: ", self.wusels[wusel_index].get_name());

            let mut has_relations: bool = false;

            for (who, relation) in self.relations.iter() {
                let other_id: usize;

                /* Get the other wusel.
                 * Skip where this wusel is even not part in the relation. */
                if wusel_id == who.0 { other_id = who.1; }
                else if wusel_id == who.1 { other_id = who.0; }
                else { continue; } // not in relation

                let other_name = self.wusels[other_id].get_name();

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
            for i in 0 .. self.wusels.len() {
                if self.wusels[i].id == id {
                    return i; // return matching id.
                }
            }
            return self.wusels.len();
        }

        /** Check if the identifer for a requesting wusel is crrently active. */
        #[allow(dead_code)]
        fn is_wusel_identifier_active(self: &Self, id: usize) -> bool {
            return self.wusel_identifier_to_index(id) < self.wusels.len();
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
        pub fn get_all_wusels_positions(self: &Self) -> Vec<(u32, u32)> {
            let mut positions = vec![];
            for w in self.wusels.iter() {
                positions.push((w.position.0, w.position.1));
            }
            return positions;
        }

        /** Check all positions.
         * Recalculate all positions, if they really consist what they promised. */
        #[allow(dead_code)]
        pub fn recalculate_all_positions(self: &mut Self) {
            self.positions = vec![vec![]; self.width as usize * self.height as usize];

            let valid_idx = self.positions.len();

            for w in self.wusels.iter() {
                let pos = w.position;
                let idx = (pos.0 + self.width * pos.1) as usize;

                /* Add id to position. */
                if idx < valid_idx {
                    self.positions[idx].push((Self::CHAR_WUSEL, w.id));
                }
            }
        }

        /** Get the `positions` index for the requesting position.
         * If the position is not in world, this index is not in [0,positions.len()).*/
        fn pos_to_idx(self: &Self, pos: (u32, u32)) -> usize {
            (pos.0 + self.width * pos.1) as usize
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

            let mut ongoing_tasks: Vec<Task> = vec![];
            let mut new_babies: Vec<(usize, usize, bool)> = vec![];

            /* Decay on every object and living. */
            for w in self.wusels.iter_mut() {

                /* Peek ongoing tasks of (all wusels) and try to proceed.
                 * While peeking, this may remove done tasks and maybe return Nothing. */
                w.auto_clean_tasks();

                /* Peek into the ongoing task, and maybe proceed them.
                 * This may lead to remove the done task. */
                if let Some(task) = w.peek_ongoing_task() {
                   ongoing_tasks.push(task.clone());
                } else {
                    /* Wusel is currently not busy. => maybe apply an idle/auto task. */
                }

                /* If pregnant: Maybe push out the child => Failure, Early or too late. */
                if let Some((father, pregnancy_days)) = w.pregnancy {
                    let maybe_now: u8 = rand::random::<u8>() % 100;
                    let possibility: u8 = match pregnancy_days {
                        0 => 90, 1 => 75, _ => 10
                    };
                    if (0u8 .. possibility).contains(&maybe_now) {
                        println!("Pop the baby!");
                        let female = rand::random::<bool>();
                        new_babies.push((w.id, father, female));
                    }
                }

                w.tick(new_day);
            }

            /* Execute ongoing tasks. */
            while ongoing_tasks.len() > 0 {
                if let Some(t) = ongoing_tasks.pop() {
                    /* Decide how to progress the command. */
                    self.proceed(t);
                }
            }

            for _ in self.relations.iter() {
                /* Decay of relations over time. */
            }

            for baby in new_babies.iter() {
                println!("New parents {}  and {}: It is a {} ",
                         baby.0, baby.1,
                         if baby.2 { "Girl" } else {" Boy" });
            }

            for i in 0..self.wusels.len() {
                self.show_wusel_tasklist_for(i);
            }
        }

        /** Proceed the task in this world. */
        fn proceed(self: &mut World, task: Task) {
            /* World proceeds task. */

            let wusel_size = self.wusels.len();
            let actor_index = self.wusel_identifier_to_index(task.active_actor_id);

            if actor_index >= wusel_size {
                return; // abort, because actor unavailable
            }

            /* Decide what to do, and if the task case done a step. */
            let succeeded = match task.passive_part {
                TaskTag::WaitLike => {
                    println!("Wait?");
                    true
                },
                TaskTag::MoveToPos(x,y) => {
                    /* Let the wusel walk; check if they stopped. */
                    let stopped: bool
                        = self.let_wusel_walk_to_position(actor_index, (x, y));

                    !stopped // succession, because it's on goal, no success, still walking.
                },
                TaskTag::GetFromPos(x,y) => {
                    println!("Pick from Goal: ({}, {}).", x, y);
                    // == MoveToPos(x,y) && pick up on field?
                    true
                },
                TaskTag::PutAtPos(x,y) => {
                    println!("Drop at Goal: ({}, {}).", x, y);
                    // == MoveToPos(x,y) && drop up on field?
                    true
                },
                TaskTag::MeetWith(other_id, nice, romantically) => {
                    let other_index = self.wusel_identifier_to_index(other_id);

                    /* Other wusel needs also to exist. */
                    if other_index >= self.wusels.len() {
                        self.wusels[actor_index].pop_ongoing_task();
                        return; // task can not be done, without target.
                    }

                    /* Check all preconditions, maybe solve one and maybe do the actually meeting. */
                    let actually_talking = self.let_two_wusels_meet(
                        actor_index, other_index,
                        nice, romantically);

                    /* If they are not actually meeting, let the task wait longer.
                     * No succession. */
                    actually_talking
                },
            };

            /* Notify the task succeeded to do a step. */
            if succeeded {
                self.wusels[actor_index].notify_ongoing_succeeded();
            }
        }

        /** Arrange the meeting of two wusels.
         * They must both exist.
         * They must be close to each other (neighbour fields or shared desk/bench...).
         * If not, the active wusels walk to the passive wusel.
         * The passive wusel must be free or ready to receive the active wusel's approaches.
         * If not, let the active wait for longer and add the request to the passive wusel.
         * The outcome may be influenced by random and the communocation abilites the active member.
         *
         * #Return, if they actually met (true), or only precondintions needed to be satisfied (false). */
        fn let_two_wusels_meet(self: &mut Self, active_index: usize, passive_index: usize, intention_good: bool, romantically: bool) -> bool {
            println!("Meet with {}, nice: {}.", self.wusels[passive_index].show(), intention_good);

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
                return false;
            }

            /* If the target does not know yet about the meeting, let them know. */
            // XXX (2020-11-22) a wusel working on only one task with higher index than active, might look unbusy, but they are not.
            // XXX SOLVING IDEA: not popping, but referencing with get in tick.
            if false {
                return false;
            }

            /* If the passive target is not yet ready to talk, wait. */
            // XXX (2020-11-22) a wusel currently waiting might have popped the task kn tick.
            if false {
                return false;
            }

            let performance: bool; // how well is the communication

            performance = true;
            // random influence of 10%
            // current value and intention
            // communication ability

            self.update_wusel_relations(
                self.wusels[active_index].id, self.wusels[passive_index].id,
                intention_good && performance,
                romantically && performance);

            return true; // they actually met.
        }

        /** Let the wusel walk to a position.
         * If the path is already calculated, let it walk the pre-calculated path.
         * If not, calculate a new path.
         * If an obstacle occurs, recalculate the path and continue walking.
         * If the goal, is reached, the walk is done.
         *
         * #Return, if wusel has stopped walking / is on goal (true), otherwise true, if they are still walking. */
        fn let_wusel_walk_to_position(self: &mut Self, wusel_index: usize, goal: (u32, u32)) -> bool {
            let pos = self.get_wusel_position(wusel_index);

            /* Check if the goal is already reached. */
            if pos.0 == goal.0 && pos.1 == goal.1 {
                println!("Reached Goal ({},{}).", goal.0, goal.1);
                return true; // stopped walking.
            }

            println!("Move to Goal {:?}.", goal);

            /* Check, if the pre-calculated path is blocked. */
            if false {
                /* Abort the pre-calculated, but blocked path. */
            }

            /* Check, if the path is (still) pre-calculated. */
            if true {
                /* Walk the path. */
                // XXX easy placeholder walking, ignoring all obstacles.

                /* Get the current positions neighbours. */
                let neighbours = Self::get_neighbour_positions(
                    self.width, self.height,
                    pos);

                if neighbours.len() < 1 {
                    println!("Wusel cannot move, it's enclosed, wait forever");
                    return true;
                }

                let goal: (u32, u32) = (goal.0, goal.1);
                let mut closest: (u32, u32) = neighbours[0];
                let mut closest_distance: f32 = f32::MAX;

                /* Find clostest neighbour to goal. */
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
            (((a.0 as i64- b.0 as i64).pow(2) + (a.1 as i64 - b.1 as i64)
              .pow(2)) as f32).sqrt()
        }

        /** Update the relation of two wusels, given by their id. */
        pub fn update_wusel_relations(self: &mut Self, wusel0_id: usize, wusel1_id: usize, nice: bool, romantic: bool) {
            /* Decide for a relation key: (Greater ID, Smaller ID). */

            let key = if wusel0_id <= wusel1_id {
                (wusel0_id, wusel1_id)
            } else {
                (wusel1_id, wusel0_id)
            };

            let change = if nice { 1 } else { -1 };

            /* Get the relation if available.
             * update a key, guarding against the key possibly not being set. */
            let rel = self.relations
                .entry(key)
                .or_insert_with(Relation::new);

            (*rel).friendship += change;

            if romantic {
                (*rel).romance += change;
            }
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
        fn new() -> Self{
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
            format!("'{official}' {rel_f}{friendly} {rel_r}{romance}{kinship}",
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
                    })
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

        fn name(self: &Self) -> &str {
            return match self {
                Self::WATER => "water", Self::FOOD   => "food"  ,
                Self::WARMTH => "warmth", Self::SLEEP => "sleep",
                Self::HEALTH => "health", Self::LOVE   => "love",
                Self::FUN   => "fun",
            }
        }

        fn get_default_decay(self: &Self) -> u32 {
            for i in 0 .. Self::VALUES.len() {
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
        #[allow(dead_code)]
        pub fn get_name(self: &Self) -> String {
            self.name.clone()
        }

        /** Get the duration of the future task or all then created tasks. */
        #[allow(dead_code)]
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
        GetFromPos(u32, u32), // pick up a thing from position.
        PutAtPos(u32, u32), // drop something at position.
        MeetWith(usize, bool, bool), // commute with another wusel (id)
    }

    impl Task {
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

        position: (u32, u32),

        female: bool, // female => able to bear children, male => able to inject children
        pregnancy: Option<(usize, u8)>, // optional pregnancy with father's id and remaining days.

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

        /** Show position of its body. */
        pub fn get_position(self: &Self) -> (u32, u32) {
            (self.position.0, self.position.1)
        }

        /** Tick one unit.
         * @return if the wusel is still alive in the end. */
        fn tick(self: &mut Self, add_day: bool) -> bool {

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

            /* Add a new day. */
            if add_day {
                self.add_new_day()
            }

            return self.is_alive()
        }

        /** Count a new day to the lived lived. */
        fn add_new_day(self: &mut Self) {
            if self.is_alive() {
                /* Age one day. */
                self.lived_days += 1;

                /* Decay all abilities by one point. */
                for i in 0 .. self.abilities.len() {
                    let (abi, val) = self.abilities[i];
                    self.abilities[i] = (abi, val - 1);
                }

                /* If pregnant, reduce time until arrival. */
                self.pregnancy = match self.pregnancy {
                    Some((father, days)) if days > 0 => Some((father, days - 1)),
                    _ => self.pregnancy
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
                s += &format!("[{active}{name} {progress}/{duration}]",
                             active = if i == n -1 { "#" } else { "" },
                             name = self.tasklist[i].name,
                             progress = self.tasklist[i].done_steps,
                             duration = self.tasklist[i].duration);

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
                let bar_len = (*v  * max_len / full) as usize;

                s += &format!(" {name:>14} {value:5} {end:.>bar_len$} \n",
                         name = n.name(),
                         value = v,
                         bar_len = usize::min(bar_len, max_len as usize),
                         end = "");
            }
            return s;
        }

        /** Print the Wusel's abilities. */
        fn show_abilities(self: &Self) -> String {
            let mut s = String::new();
            for (ability, value) in &self.abilities {
                s += &format!("{a:>15} {v:5} {bar:*<v$}",
                         a = ability.name(),
                         v = *value as usize,
                         bar = "");
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
            let task = task_builder.assign(self);
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

        /** Peek the ongoing task. */
        fn peek_ongoing_task(self: &Self) -> Option<&Task> {
            self.tasklist.last()
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
