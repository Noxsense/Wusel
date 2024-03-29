//! # Wusel
//!
//! ... and gender, lifestates, needs, abilities.
//!
//! ## Author
//! Ngoc (Nox) Le <noxsense@gmail.com>

pub mod abilities;
pub mod needs;
pub mod relations;
pub mod tasks;

pub type WuselId = usize;

/// Life state of a Wusel.
/// All but alive leads to a not living state, though a ghost may wander and interact.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Life {
    ALIVE,
    #[allow(unused)]
    DEAD,
    #[allow(unused)]
    GHOST,
}

/// A non-binary gender type for a Wusel
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WuselGender {
    Female,
    Male,
    Undefined,
    // TODO (2021-12-11) Though it is still discrete.
    // Make something like: can carry / can impregnate (not mutually exclusive, independent from gender identification
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
            Self::Undefined => 'x',
        }
    }

    pub fn to_char_pretty(&self) -> char {
        match self {
            Self::Female => '\u{2640}',
            Self::Male => '\u{2642}',
            Self::Undefined => '\u{26b2}',
        }
    }
}

/// Wusel.
/// Bundle of information on a certain position and abilities.
pub struct Wusel {
    id: WuselId,
    name: String,
    gender: WuselGender,
    pregnancy: Option<(Option<WuselId>, u8)>, // other partner optional
    life: Life,
    lived_days: u32,
    needs: std::collections::HashMap<needs::Need, u32>,
    abilities: std::collections::HashMap<abilities::Ability, u32>,
    tasklist: Vec<tasks::Task>,
}

impl std::cmp::Eq for Wusel {}

impl std::cmp::PartialEq for Wusel {
    fn eq(&self, other: &Self) -> bool {
        other.id == self.id
    }
}

impl std::fmt::Display for Wusel {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "{}: {} (days: {}, status: {:?}, gender: {:?})",
            self.id, self.name, self.lived_days, self.life, self.gender,
        )
    }
}

impl Wusel {
    pub fn new(id: WuselId, name: String, gender: WuselGender) -> Self {
        let mut new = Self {
            id,
            name,
            gender,
            pregnancy: None,
            life: Life::ALIVE,
            lived_days: 0,
            needs: std::collections::HashMap::new(),
            abilities: std::collections::HashMap::new(),
            tasklist: vec![],
        };

        for n in needs::Need::VALUES.iter() {
            new.needs.insert(*n, n.get_full());
        }

        new
    }

    pub fn get_id(&self) -> WuselId {
        self.id
    }

    pub fn is_alive(&self) -> bool {
        matches!(self.life, Life::ALIVE)
    }

    pub fn get_lived_days(&self) -> u32 {
        self.lived_days
    }

    pub fn set_life_state(&mut self, life_state: Life) -> bool {
        self.life = life_state;
        self.is_alive()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn get_gender(&self) -> WuselGender {
        self.gender
    }

    pub fn set_gender(&mut self, new_gender: WuselGender) {
        self.gender = new_gender;
    }

    /// Tick one unit.
    /// Reduce the satisfaction of each needs by default values.
    /// Maybe let it age one day.
    /// @return if the wusel is still alive in the end.
    pub fn wusel_tick(&mut self, add_day: bool) -> bool {
        let is_ill = false;
        let in_cold_environment = false;

        let mut life_state = Life::ALIVE;

        // Decrease every value by DEFAULT_NEED_DECAY_PER_MINUTE * minutes.
        for (need, value) in self.needs.iter_mut() {
            let mut decay = need.get_default_decay();

            if is_ill {
                // XXX when SICK: decay health
                decay += 1;
            }

            if in_cold_environment {
                // XXX when IN COLD: decay warmth
                decay += 1;
            }

            *value = value.saturating_sub(decay);

            if *value < 1 && need.is_fatal() {
                life_state = Life::DEAD;
            }
        }

        if life_state != Life::ALIVE {
            self.life = life_state;
        }

        if add_day {
            self.add_new_day()
        }

        self.is_alive()
    }

    fn add_new_day(&mut self) {
        if self.is_alive() {
            self.lived_days += 1;

            for (_, value) in self.abilities.iter_mut() {
                *value = value.saturating_sub(1);
            }

            if let Some((other_parent, days)) = self.pregnancy {
                self.pregnancy = Some((other_parent, days.saturating_sub(1)));
            }
        }
    }

    pub fn get_need(&self, need: needs::Need) -> u32 {
        *self.needs.get(&need).unwrap_or(&0u32)
    }

    /// Set the value for a need.
    /// This may append the needs with the new given value.
    pub fn set_need(&mut self, need: needs::Need, new_value: u32) -> u32 {
        self.needs.insert(need, new_value).unwrap_or(0u32)
    }

    /// Change the value for a need relatively.
    /// This may create a new value, with default input changed by the change value.
    /// @return the new value.
    pub fn set_need_relative(&mut self, need: needs::Need, change_value: i16) -> u32 {
        let current: i64 = self.get_need(need) as i64; // get current value (or default)
        let changed: i64 = i64::max(0, current.saturating_add(change_value as i64));
        self.set_need(need, changed as u32) // change the value.
    }

    pub fn get_ability(&self, ability: abilities::Ability) -> u32 {
        *self.abilities.get(&ability).unwrap_or(&0u32)
    }

    pub fn set_ability(&mut self, ability: abilities::Ability, new_value: u32) -> u32 {
        self.abilities.insert(ability, new_value).unwrap_or(0u32)
    }

    pub fn improve(&mut self, ability: abilities::Ability) {
        let value = *self.abilities.get(&ability).unwrap_or(&0u32);
        self.abilities.insert(ability, value.saturating_add(1));
    }

    pub fn has_tasklist_empty(&self) -> bool {
        self.tasklist.is_empty()
    }

    pub fn get_tasklist_len(&self) -> usize {
        self.tasklist.len()
    }

    pub fn get_tasklist_names(&self) -> Vec<String> {
        return self.tasklist.iter().map(|task| task.get_name()).collect();
    }

    pub fn assign_to_task(&mut self, init_time: usize, task_builder: tasks::TaskBuilder) {
        let task = task_builder.assign(init_time, self);
        self.tasklist.insert(0, task); // revert queue
    }

    pub fn prioritize_task(&mut self, task_index: usize) -> bool {
        if task_index < self.tasklist.len() {
            let task = self.tasklist.remove(task_index);
            self.tasklist.push(task); // push to end (which is next in row)
            true
        } else {
            false
        }
    }

    pub fn abort_task(&mut self, index: usize) {
        if index < self.tasklist.len() {
            self.tasklist.remove(index);
        }
    }

    /// * Check if tasklist contains a task with a given passive part.
    /// (supportive, not for the user.)
    pub fn has_task_with(&self, task_tag: &tasks::TaskTag) -> bool {
        let index = self
            .get_next_task_index_with(&|task: &tasks::Task| task.get_passive_part() == *task_tag);
        index.is_some()
    }

    /// * Check if tasklist contains a matching the given expression.
    /// (supportive, not for the user.)
    pub fn get_next_task_index_with(
        &self,
        task_matcher: &dyn Fn(&tasks::Task) -> bool,
    ) -> Option<usize> {
        self.tasklist
            .iter()
            .rev()
            .position(|task| task_matcher(task))
            .map(|index| self.tasklist.len() - 1 - index) // re-reverse
    }

    pub fn peek_ongoing_task(&self) -> Option<&tasks::Task> {
        self.tasklist.last()
    }

    /// Start the ongoing task.
    /// This may set the started flag to true, if not yet set and maybe
    /// updates the starting time.
    /// (supportive, not for the user.)
    pub fn start_ongoing_task(&mut self, start_time: usize) {
        if let Some(t) = self.tasklist.last_mut() {
            if !t.has_started() {
                // update to then.
                t.start_at(start_time);
            }
        }
    }

    /// Notify the ongoing task, that its done steps are increased
    /// This increases the optional ongoing tasks [done_steps](tasks::Task.done_steps).
    /// (supportive, not for the user.)
    pub fn increase_ongoing_task_steps(&mut self) {
        if let Some(ongoing) = self.tasklist.last_mut() {
            ongoing.increase_done_steps();
        }
    }

    /// * Drop last task.
    /// (supportive, not for the user.)
    pub fn pop_ongoing_task(&mut self) -> Option<tasks::Task> {
        self.tasklist.pop()
    }

    /// Clean task list.
    /// Remove ongoing tasks if there are no steps left.
    /// (supportive, not for the user.)
    pub fn auto_clean_tasks(&mut self) {
        // Remove ongoing task, if it is done.
        while let Some(ongoing) = self.peek_ongoing_task() {
            if ongoing.get_rest_time() < 1 {
                self.tasklist.pop();
            } else {
                break; // ongoing task not yet done.
            }
        }
    }

    pub fn is_pregnant(&self) -> bool {
        self.pregnancy != None
    }

    pub fn set_pregnancy(
        &mut self,
        other_parent_index: Option<WuselId>,
        remaining_days: Option<u8>,
    ) {
        if let Some(remaining_days) = remaining_days {
            // other parent can be none, ... divine intervention.
            self.pregnancy = Some((other_parent_index, remaining_days));
        } else {
            // no remaining days unsets the pregnancy.
            self.pregnancy = None;
        }
    }

    pub fn get_other_parent(&self) -> Option<WuselId> {
        // self.pregnancy.0.map(|other_parent| other_parent.get_id());
        if let Some((other_parent, _)) = self.pregnancy {
            other_parent // can also be only one parent (self)
        } else {
            None
        }
    }

    pub fn get_remaining_pregnancy_days(&self) -> Option<u8> {
        if let Some((_, days)) = self.pregnancy {
            Some(days)
        } else {
            None
        }
    }
}
