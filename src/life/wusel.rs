/**
 * Module Wusel ( and gender, lifestates, needs, abilities,).
 *
 * @author Nox
 * @version 2021.0.1
 */

use crate::life::tasks;

/** Wusel.
 * Bundle of information on a certain position and abilities. */
pub struct Wusel {
    id: usize,

    /* Name */
    name: String,

    gender: WuselGender, // WuselGender::Female => able to bear children, male => able to inject children
    pregnancy: Option<(Option<usize>, u8)>, // optional pregnancy with father's ID and remaining days.

    life: Life,      // alive | dead | ghost
    lived_days: u32, // last lived day.

    needs: Vec<(Need, u32)>,

    /* Abilities. */
    abilities: Vec<(Ability, u32)>, // ability levels.

    /* List of tasks. */
    tasklist: Vec<tasks::Task>,
}

const MINUTE: u32 = 2; // ticks
const HOUR: u32 = 60 * MINUTE; // ticks
const DAY: u32 = 24 * HOUR; // ticks
const WEEK: u32 = 7 * DAY; // ticks

impl Wusel {
    /** From full to 0, how many ticks does it need, when it's only normally decreasing. */
    const WUSEL_FULL_NEEDS: [(Need, u32); 7] = [
        (Need::WATER, DAY * 3),   // 3 days until dehydrate.
        (Need::FOOD, WEEK),    // a week until starve.
        (Need::WARMTH, (8 * HOUR)),       // 8h until freeze to death.
        (Need::SLEEP, WEEK),   // a week until suffer from sleep loss.
        (Need::HEALTH, WEEK * 2), // 2 weeks until die of illness.
        (Need::LOVE, WEEK * 2),   // 2 weeks until become lonely.
        (Need::FUN, WEEK * 2),    // 2 weeks until unmotivated and depressive.
    ];

    /** Create a new Wusel with name. */
    pub fn new(id: usize, name: String, gender: WuselGender) -> Self {
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

        new
    }

    #[allow(dead_code)]
    pub fn get_id(&self) -> usize {
        self.id
    }

    /** Check, if this Wusel is alive. */
    pub fn is_alive(&self) -> bool {
        matches!(self.life, Life::ALIVE)
    }

    /** Get name of the Wusel. */
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_gender(&self) -> WuselGender {
        self.gender
    }

    /** Tick one unit.
     * Reduce the satisfaction of each needs by default values.
     * Maybe let it age one day.
     * @return if the wusel is still alive in the end. */
    pub fn wusel_tick(&mut self, add_day: bool) -> bool {
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

        self.is_alive()
    }

    /** Count a new day to the lived lived. */
    fn add_new_day(&mut self) {
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

    /** Get the default need value. */
    pub fn default_need_full(need: &Need) -> u32 {
        for (n, v) in Self::WUSEL_FULL_NEEDS.iter() {
            if n == need {
                return *v;
            }
        }
        0 // else 0, if not an default need.
    }

    /** Get the value for a need.
     * This may append the needs with a new default value, if the need is not
     * yet inserted. */
    pub fn get_need(&mut self, need: Need) -> u32 {
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
        default
    }

    /** Set the value for a need.
     * This may append the needs with the new given value. */
    pub fn set_need(&mut self, need: Need, new_value: u32) {
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
    pub fn modify_need(&mut self, need: Need, change_value: i16) -> u32 {
        let current: i64 = self.get_need(need) as i64; // get current value (or default)
        let changed: i64 = i64::max(0, current.saturating_add(change_value as i64));

        self.set_need(need, changed as u32); // change the value.
        self.get_need(need) // final need's value.
    }

    /** Improve the given ability by one point. */
    #[allow(dead_code)]
    fn improve(&mut self, ability: &Ability) {
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
    pub fn get_ability(&self, ability: &Ability) -> u32 {
        for (a, v) in self.abilities.iter() {
            if a == ability {
                return *v;
            }
        }
        0
    }

    pub fn is_tasklist_empty(&self) -> bool {
        self.tasklist.is_empty()
    }

    pub fn get_tasklist_len(&self) -> usize {
        self.tasklist.len()
    }

    /** Print the tasklist (as queue). */
    pub fn get_tasklist_names(&self) -> Vec<String> {
        return self
            .tasklist
            .iter()
            .map(|task| task.get_name())
            .collect();
    }

    /** Append a new task to the task list. */
    pub fn add_task(&mut self, init_time: usize, task_builder: tasks::TaskBuilder) {
        /* Task apply self as actor. */
        let task = task_builder.assign(init_time, self);
        self.tasklist.insert(0, task); // revert queue
    }

    /** Put task from task list to next-ongoing-task position. */
    pub fn prioritize_task(&mut self, task_index: usize) {
        if task_index < self.tasklist.len() {
            let task = self.tasklist.remove(task_index);
            self.tasklist.push(task); // push to end (which is next in row)
        }
    }

    /** Get the next task in row, that fulfills the given check. */
    pub fn get_next_task_index_with(
        &mut self,
        task_matcher: &dyn Fn(&tasks::Task) -> bool
    ) -> usize {
        let mut i: usize = self.get_tasklist_len();
        while i > 0usize {
            i -= 1;
            if task_matcher(&self.tasklist[i]) {
                return i;
            }
        }
        usize::MAX
    }

    /** Abort a task in the task list. */
    pub fn abort_task(&mut self, index: usize) {
        if index < self.tasklist.len() {
            self.tasklist.remove(index);
        }
        /* Otherwise no task is aborted. */
    }

    /** Clean task list.
     * Remove ongoing tasks if there are no steps left. */
    pub fn auto_clean_tasks(&mut self) {
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
    pub fn has_task_with(&self, task_tag: &tasks::TaskTag) -> bool {
        for t in self.tasklist.iter() {
            if t.get_passive_part() == *task_tag {
                return true;
            }
        }
        false
    }

    /** Peek the ongoing task. */
    pub fn peek_ongoing_task(&self) -> Option<&tasks::Task> {
        self.tasklist.last()
    }

    /** Start the ongoing task.
     * This may set the started flag to true, if not yet set and maybe
     * updates the starting time. */
    pub fn start_ongoing_task(&mut self, start_time: usize) {
        if let Some(t) = self.tasklist.last_mut() {
            if !t.has_started() {
                // update to then.
                t.start_at(start_time);
            }
        }
    }

    /** Notify the ongoing task, that its done steps are increased
     * This increases the optional ongoing tasks [done_steps](tasks::Task.done_steps). */
    pub fn notify_ongoing_succeeded(&mut self) {
        if let Some(ongoing) = self.tasklist.last_mut() {
            ongoing.increase_done_steps();
        }
    }

    /** Pop the ongoing task (queue reversed). */
    pub fn pop_ongoing_task(&mut self) -> Option<tasks::Task> {
        self.tasklist.pop()
    }

    /** Check, if the wusel is pregnant. */
    #[allow(dead_code)]
    pub fn is_pregnant(&self) -> bool {
        self.pregnancy != None
    }

    pub fn get_other_parent(&self) -> Option<usize> {
        // self.pregnancy.0.map(|other_parent| other_parent.get_id());
        if let Some((other_parent, _)) = self.pregnancy {
            other_parent // can also be only one parent (self)
        } else {
            None
        }
    }

    /** Get the remaining days of an possible Pregnancy. */
    #[allow(dead_code)]
    pub fn get_remaining_pregnancy_days(&self) -> Option<u8> {
        if let Some((_father, days)) = self.pregnancy {
            Some(days)
        } else {
            None
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WuselGender {
    Female,
    Male,
}

impl WuselGender {
    pub const VALUES: [Self; 2] = [Self::Female, Self::Male];

    pub fn random() -> Self {
        Self::VALUES[rand::random::<usize>() % Self::VALUES.len()]
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Female => 'f',
            Self::Male => 'm',
        }
    }

    pub fn to_char_pretty(&self) -> char {
        match self {
            Self::Female => '\u{2640}',
            Self::Male => '\u{2642}'
        }
    }
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

    pub fn name(&self) -> &str {
        match self {
            Self::WATER => "water",
            Self::FOOD => "food",
            Self::WARMTH => "warmth",
            Self::SLEEP => "sleep",
            Self::HEALTH => "health",
            Self::LOVE => "love",
            Self::FUN => "fun",
        }
    }

    pub fn get_default_decay(&self) -> u32 {
        for i in 0..Self::VALUES.len() {
            if self == &Self::VALUES[i] {
                return Self::DEFAULT_NEED_DECAY_PER_MINUTE[i];
            }
        }
        0 // default: no decay.
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
    fn get_name(&self) -> &str {
        match self {
            Self::COOKING => "cooking",
            Self::COMMUNICATION => "communication",
            Self::FITNESS => "fitness",
            Self::FINESSE => "finesse",
        }
    }
}

pub enum RelationType {
    Romance,
    Friendship,
}

impl RelationType {

    pub fn from_romantically(try_romance: bool) -> Self {
        if try_romance {
            Self::Romance
        } else {
            Self::Friendship
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Friendship => '\u{2639}', // smiley
            Self::Romance =>  '\u{2661}', // heart
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

impl Default for Relation {
    fn default() -> Self {
        Self::new()
    }
}

impl Relation {
    /** Create a new empty relationship for just met strangers. */
    pub fn new() -> Self {
        Self {
            officially: String::from("Strangers"),
            friendship: 0,
            romance: 0,
            kindred_distance: -1,
        }
    }

    pub fn update(&mut self, relationtype: RelationType, change: i32) {
        match relationtype {
            RelationType::Friendship => self.update_friendship(change),
            RelationType::Romance => self.update_romance(change),
        }
    }

    pub fn update_romance(&mut self, change: i32) {
        self.romance += change;
    }

    pub fn update_friendship(&mut self, change: i32) {
        self.friendship += change;
    }

    /** Print this relation to a String. */
    pub fn show(&self) -> String {
        format!(
            "'{official}' {relation_friendly_char}{friendly} {relation_romance_char}{romance}{kinship}",
            official = self.officially,
            relation_friendly_char = RelationType::Friendship.to_char(),
            friendly = self.friendship,
            relation_romance_char = RelationType::Romance.to_char(),
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
