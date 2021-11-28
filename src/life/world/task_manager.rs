/**
 * Module Task Manager.
 * - Let Tasks progress and check their preconditions and their after-effects.
 *
 * @author Nox
 * @version 2021.0.1
 */
use crate::life::areas;
use crate::life::tasks;
use crate::life::world;
use crate::life::wusel;

const MEET_RESULT_ERROR: i8 = -1; //  meeting error.
const MEET_RESULT_OK: i8 = 0; //  When they met, like the C-ish "OK".
const MEET_RESULT_FOLLOWED: i8 = 1; //  When the actor walked, they might not have met yet.
const MEET_RESULT_KNOCKED: i8 = 2; //  When the actual knocking was just applied, they know both of the meeting, but that may come next.
const MEET_RESULT_WAITED: i8 = 3; //  When the knocking was done, but the passive is still busy, they actually have not met like intended.

/** Proceed the task in this world. */
pub fn proceed(world: &mut world::World, task: tasks::Task) {
    /* World proceeds task. */

    let actor_id = task.get_active_actor_id();
    let actor_index = world.get_wusels_index_by_id(actor_id);

    if actor_index == None {
        return; // abort, because actor unavailable
    }

    let actor_index = actor_index.unwrap();

    let start_time = match task.has_started() {
        true => task.get_start_time(),
        false => {
            /* Notify the start of the task (for the wusel). */
            world.wusels[actor_id].start_ongoing_task(world.clock);

            world.clock // starting now
        }
    };

    /* Decide what to do, and if the task case done a step. */
    let succeeded = match task.get_passive_part() {
        tasks::TaskTag::WaitLike => {
            log::debug!("{}", task.get_name());
            true
        }
        tasks::TaskTag::BeMetFrom(other_id) => {
            let other_index = world.get_wusels_index_by_id(other_id);

            /* Other wusel needs also to exist or still wants to meet.
             * Otherwise pop. */

            /* Meeting party is valid, check their ongoing task. */
            if let Some(other_index) = other_index {
                match world.wusels[other_index].peek_ongoing_task() {
                    /* => Proceed, since the other party is doing nothing, so no meeting. */
                    None => true,

                    /* Other party is busy. */
                    Some(t) => {
                        !matches!(t.get_passive_part(), tasks::TaskTag::MeetWith(id, _nice, _love) if id == actor_id)
                    }
                }
            } else {
                /* => proceed, since the other party was invalid. */
                true
            }
        }
        tasks::TaskTag::MeetWith(other_id, nice, romantically) => {
            let other_index = world.get_wusels_index_by_id(other_id);

            /* Other wusel needs also to exist. */
            if other_index == None {
                world.wusels[actor_index].pop_ongoing_task();
                return; // task can not be done, without target.
            }

            let other_index = other_index.unwrap();

            /* Check all preconditions, maybe solve one and maybe do the actually meeting.
             * 0, when they met, like the C-ish "OK".
             * 1, when the actor walked.
             * 2, when the actual knocking was just applied.
             * 3, when the knocking was done, but the passive is still busy. */
            let meeting_result =
                let_two_wusels_meet(world, actor_index, other_index, nice, romantically);

            /* On Final Success with own step,
             * also let the BeMetFrom() succeed. */

            match meeting_result {
                // waiting, but don't wait too long.
                MEET_RESULT_WAITED => {
                    if world.clock - start_time >= tasks::Task::PATIENCE_TO_MEET {
                        world.wusels[actor_index].pop_ongoing_task();
                    }
                    false // => do not notify succession
                }

                /* They met and the task is over. */
                MEET_RESULT_OK => true, // => notify process
                _ => false,             // => no process (FOLLOWED, KNOCKED or an unexpected)
            }
        }
        tasks::TaskTag::MoveToPos(position) => {
            /* Let the wusel walk; check if they stopped. */
            let stopped: bool = let_wusel_walk_to_position(world, actor_index, position);

            stopped // true == stop == success.
        }
        tasks::TaskTag::UseObject(object_id, action_id) => {
            // TODO: get index for the given object ID.
            let object_index = world
                .objects
                .iter()
                .position(|wo| wo.get_object_id() == object_id);

            // TODO: get index for the given action ID.
            let action_index = if action_id >= world.actions.len() {
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
                let_wusel_use_object(world, actor_index, object_index, action_index)
            }
        }
    };

    /* Notify the task succeeded to do a step. */
    if succeeded {
        world.wusels[actor_index].increase_ongoing_task_steps();
    }
}

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
    world: &mut world::World,
    active_index: usize,
    passive_index: usize,
    intention_good: bool,
    romantically: bool,
) -> i8 {
    log::debug!(
        "Meet with {}, nice: {}.",
        world.wusels[passive_index].get_name(),
        intention_good
    );

    /* If not close to the other wusel, use this step to get closer,
     * return as not yet ready. */
    let opt_passive_position_index = world.wusels_index_on_position_index.get(passive_index);

    if opt_passive_position_index.is_none() {
        return MEET_RESULT_ERROR; // No position.
    }

    let &passive_position_index = opt_passive_position_index.unwrap();

    let position_passive_wusel = world.position_from_index(passive_position_index);

    if position_passive_wusel == None {
        return MEET_RESULT_ERROR; // No position.
    }

    let position_passive_wusel = position_passive_wusel.unwrap();

    log::debug!("Meet at {:?}", position_passive_wusel);

    /* If the actor is close enough, do the next steps. */
    let following = let_wusel_walk_to_position_if_not_close(world, active_index, position_passive_wusel, 2.0);

    /* Just followed. */
    if !following {
        return MEET_RESULT_FOLLOWED;
    }

    let active_id = world.wusels[active_index].get_id();
    let passive_id = world.wusels[passive_index].get_id();

    /* Get the passive wusel's current task.
     * If it is being met by the active, succeed a step with the meeting,
     * otherwise see below. */
    let passives_ongoing_tasktag: Option<tasks::TaskTag>
        = world.wusels[passive_index]
        .peek_ongoing_task()
        .map(|t| t.get_passive_part());

    let active_is_met = &tasks::TaskTag::BeMetFrom(active_id);

    let handshake_okay = matches!(&passives_ongoing_tasktag, Some(tag) if *tag == *active_is_met);

    if handshake_okay {
        let performance: bool; // how well is the communication

        performance = true;
        // random influence of 10%
        // current value and intention
        // communication ability

        /* Update the relation between active and passive. */
        world.wusel_update_relations(
            active_id,
            passive_id,
            intention_good && performance,
            if romantically {
                wusel::RelationType::Romance
            } else {
                wusel::RelationType::Friendship
            },
        );

        return MEET_RESULT_OK; // they actually met.
    }

    /* Check, if the passive is already waiting (in tasklist). */
    let passive_is_waiting
        = world.wusels[passive_index].has_task_with(active_is_met);

    /* Check if they both want an (actively) Meeting each other. */
    let mutuall_meeting_as_actives = matches!(&passives_ongoing_tasktag, Some(tasks::TaskTag::MeetWith(id, _, _)) if *id == active_id);

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
            _a if world.wusels[active_index].has_task_with(&tasks::TaskTag::BeMetFrom(passive_id)) =>
            {
                active_index
            }
            _ => world.wusels.len(),
        };

        /* Move already waiting task to active tasks. */
        if already_waiting_index < world.wusels.len() {
            /* What happens:
             * A: [Talk B, tasks::Task A2, tasks::Task A3]
             * B: [Talk A, tasks::Task B3, Listen A] // B already knows.
             * ----
             * A: [Talk B, tasks::Task A2, tasks::Task A3]
             * B: [Listen A, Talk A, tasks::Task B2, tasks::Task B3] // let B listen first.
             */
            let waiting_task_index_opt = world.wusels[already_waiting_index]
                .get_next_task_index_with(&|task| task.get_passive_part() == *active_is_met);

            if let Some(waiting_task_index) = waiting_task_index_opt {
                world.wusels[already_waiting_index]
                    .prioritize_task(waiting_task_index);
            }

            return MEET_RESULT_KNOCKED; // even if it might be knocked before.
        }

        /* Non of them requested anything before.
         * Decide it on communication skill.
         * On tie, let this active be the first one.
         * (No waiting-to-be-met needs to be deleted.) */

        let skill = wusel::Ability::COMMUNICATION;
        let c0 = world.wusels[active_index].get_ability(skill);
        let c1 = world.wusels[passive_index].get_ability(skill);

        let (more_active, more_passive) = match c0 {
            better if better > c1 => (active_index, passive_index),
            worse if worse < c1 => (passive_index, active_index),
            _tie if active_index < passive_index => (active_index, passive_index),
            _ => (passive_index, active_index),
        };

        world.wusel_assign_to_task(
            more_passive,
            tasks::TaskBuilder::be_met_from(more_active)
                .set_name(format!("Be met by {}", more_active)),
        );

        return MEET_RESULT_KNOCKED;
    }

    /* Else, just notify them, if not yet done,
     * I am there and wait for them to be ready. */
    if !passive_is_waiting {
        /* Tell passive to be ready for active. */
        world.wusel_assign_to_task(passive_index, tasks::TaskBuilder::be_met_from(active_id));
        return MEET_RESULT_KNOCKED;
    }

    /* If the passive target is not yet ready to talk, wait.  */
    MEET_RESULT_WAITED
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
    world: &mut world::World,
    wusel_index: usize,
    object_index: usize,
    action_index: usize,
) -> bool {
    /* Invalid wusel index. */
    if !world.check_valid_wusel_index(wusel_index) {
        return false;
    }

    let wusel_id = world.wusels[wusel_index].get_id();

    /* Invalid object index. */
    if object_index >= world.objects.len() {
        log::warn!("No such object.");
        return false;
    }

    let opt_object_id
        = world.objects
        .get(object_index)
        .map(|object| object.get_object_id());

    if opt_object_id.is_none() {
        log::warn!("No such object.");
        return false;
    }

    let object_id = opt_object_id.unwrap();

    /* Check where the object is.
     * If AtPosition(position) => go to position (position).
     * If StoredIn(storage) => get from storage.
     * If HeldBy(holder_id) => holder_id ==~ wusel_id => ok, else abort. */
    let object_position = world.object_get_position(object_id);

    /* If not close to object, go to it. */
    let close_enough = if let Some(object_position) = object_position {
        log::debug!("Go to object's position.");
        let_wusel_walk_to_position_if_not_close(
            world,
            wusel_index,
            object_position, // current object position.
            1.2,     // max distance.
        )
    } else {
        false
    };

    let object_position = object_position.unwrap();
    let object_position_index = world.position_to_index(object_position);

    if !close_enough {
        return false;
    }

    let object_whereabouts = &world.objects_index_with_whereabouts.get(object_index).unwrap_or(&(world::InWorld::Nowhere));

    /* Invalid action index. */
    if action_index >= world.actions.len() {
        log::warn!("No such action.");
        return false;
    }

    log::debug!(
        "Used object ({:?} on {:?}).",
        world.actions[action_index],
        world.objects[object_index]
    );

    /* Get the effect of interacting with the object. */
    let effect = world.actions_effects.iter().find(
        |((object_type, object_subtype, _), act_id, _effect_str, _effect_vec)| {
            *object_type == object_id.0 && *object_subtype == object_id.1 && *act_id == action_index
        },
    );

    if let Some(effect) = effect {
        log::debug!("Using the object has the following effect: {:?}", effect);
        let (_, _, _, effect_vec) = effect;
        for e in effect_vec {
            log::debug!("- Apply effect: {:?}", e);
            world.wusels[wusel_index].set_need_relative(e.0, e.1);
        }
    }

    /* Do the actual action. */
    return match world.actions[action_index].as_ref() {
        "View" => {
            log::info!("Just view.");
            // TODO can u view sth, if it's held by another wusel?
            TASK_PROCEED
        }
        "Take" => {
            // if close, or already holding. => update whereabouts and TASK_PROCEED
            if let world::InWorld::OnPositionIndex(_) | world::InWorld::InStorageId(_) = object_whereabouts {
                log::info!("Get it, if possible.");
                world.object_set_whereabouts(object_index, world::InWorld::HeldByWuselId(wusel_id));
                return TASK_PROCEED;
            }
            log::warn!("Item is already hold, just look and stop.");
            TASK_PROCEED // if already held, cannot be held, but just stop to do so.
        }
        "Drop" => {
            if let world::InWorld::HeldByWuselId(holder_id) = object_whereabouts {
                if *holder_id == wusel_id {
                    log::info!("Drop it, if held by wusel themself.");
                    world.object_set_whereabouts(
                        object_index,
                        world::InWorld::OnPositionIndex(object_position_index),
                    ); // == wusel_position, as position of all containers
                    log::debug!("Object placed, somewhere.");
                    return TASK_PROCEED;
                }
            }
            TASK_PROCEED // if not held, it cannot be dropped, but the wusel will be done, to drop the object.
        }
        "Consume" => {
            let consumable = world.objects[object_index].get_consumable();
            if consumable != None {
                let left_over = consumable.unwrap();
                log::debug!("Consume a part of the consumable object.");

                if left_over <= 1usize {
                    world.object_destroy(object_index); // delete from world.
                    log::debug!("Consumable Object fully consumed.");
                    return TASK_PROCEED;
                }
                world.objects[object_index].set_consumable(Some(left_over - 1));

                // return TASK_PROCEED;
                return TASK_HOLD; // ddbug.
            }
            log::warn!("Tried to consume something  unconsumable");
            TASK_HOLD // if not held, it cannot be dropped, but the wusel will be done, to drop the object.
        }
        _ => {
            log::info!("Undefined action?");
            TASK_HOLD
        }
    };
}

/** Let the wusel walk to a position, if they are not close.
 * Return true, if they are close enough. */
fn let_wusel_walk_to_position_if_not_close(
    world: &mut world::World,
    wusel_index: usize,
    goal: areas::Position,
    max_distance: f32,
) -> bool {
    let wusel_position
        = world.wusels_index_on_position_index.get(wusel_index)
        .map(|&position_index| world.position_from_index(position_index))
        .map(|opt_opt_position| opt_opt_position.unwrap());

    if wusel_position == None {
        return false; // wusel itself has no position.
    }

    let wusel_position = wusel_position.unwrap();

    if wusel_position.distance_to(&goal) > max_distance {
        let_wusel_walk_to_position(world, wusel_index, goal);
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
fn let_wusel_walk_to_position(
    world: &mut world::World,
    wusel_index: usize,
    goal: areas::Position,
) -> bool {

    let position
        = world.wusels_index_on_position_index.get(wusel_index)
        .map(|&position_index| world.position_from_index(position_index))
        .map(|opt_opt_position| opt_opt_position.unwrap())
        ;

    if position == None {
        return true; // couldn't move => stopped walking.
    }

    let position = position.unwrap();

    /* Check if the goal is already reached. */
    if position.x == goal.x && position.y == goal.y {
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
        let neighbours = world.position_get_all_neighbours(position);

        if neighbours.is_empty() {
            log::info!("Wusel cannot move, it's enclosed, wait forever");
            return true;
        }

        let goal: areas::Position = areas::Position::new(goal.x, goal.y);
        let mut closest: areas::Position = neighbours[0];
        let mut closest_distance: f32 = f32::MAX;

        /* Find closest neighbour to goal. */
        for p in neighbours.iter() {
            let distance = goal.distance_to(p);

            if distance < closest_distance {
                closest = *p;
                closest_distance = distance;
            }
        }

        /* move to closest position. */
        world.wusel_set_position_by_index(wusel_index, closest);
        false // still walking.
    } else {
        /* Calculate the path and go it next time. */
        log::info!("Calculate the path to {:?}", goal);
        false // still walking.
    }
}
