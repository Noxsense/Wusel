//! # Tasks
//!
//! Module to manage and create tasks.
//!
//! ## Author
//! Ngoc (Nox) Le <noxsense@gmail.com>

use crate::life::areas;
use crate::life::objects;
#[allow(unused_imports)]
use crate::life::world;
use crate::life::wusels;

/// Id Type of an Action
pub type ActionId = usize;

/// TaskBuilder, to create a Task for a Wusel.
/// a up
/// Name, Target, duration and conditions are set with the builder.
#[derive(Debug, Clone)]
pub struct TaskBuilder {
    name: String,
    duration: usize,
    passive_part: TaskTag,
}

impl TaskBuilder {
    /// Create a new Task Builder.
    pub fn new(name: String) -> Self {
        Self {
            name,
            duration: 0,
            passive_part: TaskTag::WaitLike,
        }
    }

    /// Create a new Task Builder, preset for moving.
    pub fn move_to(pos: areas::Position) -> Self {
        Self {
            name: "Moving".to_string(),
            duration: 1,
            passive_part: TaskTag::MoveToPos(pos),
        }
    }

    /// Create a new Task Builder, preset for meeting.
    pub fn meet_with(passive: wusels::WuselId, friendly: bool, romantically: bool) -> Self {
        Self {
            name: "Meeting".to_string(),
            duration: 1,
            passive_part: TaskTag::MeetWith(passive, friendly, romantically),
        }
    }

    /// Create a new Task Builder, preset for working on a workbench.
    pub fn use_object(object_id: objects::ObjectId, action_id: ActionId) -> Self {
        Self {
            name: format!("Use[{}] Object[{:?}]", action_id, object_id),
            duration: 1,
            passive_part: TaskTag::UseObject(object_id, action_id),
        }
    }

    /// Create a new Task Builder, preset for being met.
    pub fn be_met_from(active: wusels::WuselId) -> Self {
        Self {
            name: "Being Met".to_string(),
            duration: 1,
            passive_part: TaskTag::BeMetFrom(active),
        }
    }

    /// Get the name of the future task or all then created tasks.
    #[allow(dead_code)]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Get the duration of the future task or all then created tasks.
    #[allow(dead_code)]
    pub fn get_duration(&self) -> usize {
        self.duration
    }

    /// Rename the task builder in the Task Builder.
    pub fn set_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    /// Set the duration in the Task Builder.
    pub fn set_duration(mut self, time: usize) -> Self {
        self.duration = time;
        self
    }

    /// Set the duration in the passive part.
    #[allow(dead_code)]
    pub fn set_passive_part(mut self, passive: TaskTag) -> Self {
        self.passive_part = passive;
        self
    }

    /// Create a new Task from the builder for the requesting [actor](crate::life::wusel::Wusel).
    pub fn assign(self, start_time: usize, actor: &wusels::Wusel) -> Task {
        Task {
            name: self.name,
            started: false,
            start_time,
            duration: self.duration,
            done_steps: 0,

            active_actor_id: actor.get_id(),
            passive_part: self.passive_part,
        }
    }
}

/// Type for a Core Task
///
/// Such as social interactions, object interactions or walking situations.
/// Also for just very tasks such as waiting or reading.
#[derive(Debug, Clone, PartialEq)]
pub enum TaskTag {
    WaitLike,
    MoveToPos(areas::Position),

    UseObject(objects::ObjectId, ActionId), // object_id, and action_id

    MeetWith(wusels::WuselId, bool, bool), // commute with another wusel (ID)
    BeMetFrom(wusels::WuselId),            // be met by another wusel (ID)
}

/// Task, a Wusel can do.
///
/// A task can contain multiple steps. This task struct also is stateful and
/// keeps track of its progress.
#[derive(Clone)]
pub struct Task {
    name: String,
    started: bool,
    start_time: usize,
    duration: usize,
    done_steps: usize,

    active_actor_id: wusels::WuselId, // wusel ID.
    passive_part: TaskTag,           // position | object-to-be | object | wusel | nothing.
}

impl Task {
    pub const PATIENCE_TO_MEET: usize = 20; // TODO

    /// Get the approximately rest time (in ticks), this task needs.
    pub fn get_rest_time(&self) -> usize {
        self.duration - self.done_steps
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn has_started(&self) -> bool {
        self.started
    }
    pub fn get_start_time(&self) -> usize {
        self.start_time
    }

    pub fn start_at(&mut self, time: usize) -> bool {
        if self.started {
            false
        } else {
            self.start_time = time;
            self.started = true;
            true
        }
    }

    pub fn get_duration(&self) -> usize {
        self.duration
    }

    pub fn get_done_steps(&self) -> usize {
        self.done_steps
    }

    pub fn increase_done_steps(&mut self) -> usize {
        self.done_steps += 1;
        self.done_steps
    }

    pub fn get_active_actor_id(&self) -> usize {
        self.active_actor_id
    }

    pub fn get_passive_part(&self) -> TaskTag {
        self.passive_part.clone()
    }
}

// TODO (2021-11-21) improve type
/// Relation between the usage of an [Object](objects::ObjectId) and [Wusel Needs](wusel::Need).
pub type ActionAffect = (
    objects::ObjectId,       // affected object
    usize,                   // object subtype
    &'static str,            // name
    Vec<(wusels::needs::Need, i16)>, // affected needs
);
