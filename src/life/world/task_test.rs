#![cfg(test)]

use super::wusels;

#[test]
fn test_walking() {
    let width: u32 = 20;
    let depth: u32 = 30;

    let mut test_world: super::World = super::World::new(width, depth); // small world.

    // indices
    let wusel_on_x: usize = 0;
    let wusel_on_y: usize = 1;
    let wusel_wild: usize = 2;

    test_world.wusel_new(
        "on_x".to_string(),
        super::wusels::WuselGender::Female,
        super::areas::Position { x: 0, y: 0, z: 0 },
    );

    test_world.wusel_new(
        "on_y".to_string(),
        super::wusels::WuselGender::Male,
        super::areas::Position { x: 2, y: 2, z: 0 },
    );

    test_world.wusel_new(
        "random".to_string(),
        super::wusels::WuselGender::Male,
        super::areas::Position { x: 3, y: 4, z: 0 },
    );

    let repetition: usize = 1000;
    let bad_behaviour_acceptance = 3;

    let mut last_x_position = test_world.wusel_get_position(wusel_on_x).unwrap();
    let mut last_y_position = test_world.wusel_get_position(wusel_on_y).unwrap();
    // let mut wuselr_pos = test_world.wusel_get_position(wusel_wild).unwrap();

    let mut wuselx_left = false;
    let mut wusely_tofront = false;
    // let mut wuselr_direction = 0;

    let mut wuselx_bad_count_in_row = 0;
    let mut wusely_bad_count_in_row = 0;
    // let mut wuselr_bad_count_in_row = 0;

    let mut goal_x: u32 = 0;
    let mut goal_y: u32 = 0;
    let mut goal_rand: super::areas::Position;

    for i in 0..repetition {
        // assign random walking (x axis).
        if test_world.wusel_get_tasklist_len(wusel_on_x) == Some(0) {
            goal_x = (rand::random::<u32>() + 1 + goal_x) % width;
            wuselx_left = goal_x < last_x_position.x;
            println!("{:03}$ Wusel x goal to ({:2}, 0, 0)", i, goal_x);
            test_world.wusel_assign_to_task(
                wusel_on_x,
                super::tasks::TaskBuilder::move_to(super::areas::Position {
                    x: goal_x,
                    y: 0,
                    z: 0,
                }),
            );
        }

        // assign random walking (y axis).
        if test_world.wusel_get_tasklist_len(wusel_on_y) == Some(0) {
            goal_y = (rand::random::<u32>() + 1 + goal_y) % depth;
            wusely_tofront = goal_y < last_y_position.y;
            println!("{:03}$ Wusel y goal to ( 0, {:2}, 0)", i, goal_y);
            test_world.wusel_assign_to_task(
                wusel_on_y,
                super::tasks::TaskBuilder::move_to(super::areas::Position {
                    x: 0,
                    y: goal_y,
                    z: 0,
                }),
            );
        }

        // assign random walking (x-y plane).
        if test_world.wusel_get_tasklist_len(wusel_wild) == Some(0) {
            goal_rand = test_world.position_random();
            println!(
                "{:03}$ Wusel ? goal to ({:2},{:2}, 0)",
                i, goal_rand.x, goal_rand.y
            );
            test_world
                .wusel_assign_to_task(wusel_wild, super::tasks::TaskBuilder::move_to(goal_rand));
        }

        test_world.tick();

        if let Some(p) = test_world.wusel_get_position(wusel_on_x) {
            println!(
                "{:03}> Wusel x on  ({:2},{:2},{:2}) => ({:2},{:2},{:2}) {}",
                i,
                last_x_position.x,
                last_x_position.y,
                last_x_position.z,
                p.x,
                p.y,
                p.z,
                if wuselx_left { "left" } else { "right" }
            );

            let expected_x = if wuselx_left {
                last_x_position.x.saturating_sub(1)
            } else {
                last_x_position.x.saturating_add(1)
            };
            let well_behaved = expected_x == p.x || p.x == goal_x;

            if !well_behaved {
                wuselx_bad_count_in_row += 1;
            } else {
                wuselx_bad_count_in_row = 0;
            }

            assert!(well_behaved || wuselx_bad_count_in_row < bad_behaviour_acceptance);

            last_x_position = p;
        }

        if let Some(p) = test_world.wusel_get_position(wusel_on_y) {
            println!(
                "{:03}> Wusel y on  ({:2},{:2},{:2}) => ({:2},{:2},{:2}) {}",
                i,
                last_y_position.x,
                last_y_position.y,
                last_y_position.z,
                p.x,
                p.y,
                p.z,
                if wusely_tofront {
                    "to front"
                } else {
                    "to back"
                }
            );

            let expected_y = if wusely_tofront {
                last_y_position.y.saturating_sub(1)
            } else {
                last_y_position.y.saturating_add(1)
            };
            let well_behaved = expected_y == p.y || p.y == goal_y;

            if !well_behaved {
                wusely_bad_count_in_row += 1;
            } else {
                wusely_bad_count_in_row = 0;
            }

            assert!(well_behaved || wusely_bad_count_in_row < bad_behaviour_acceptance);

            last_y_position = p;
        }

        if let Some(p) = test_world.wusel_get_position(wusel_wild) {
            println!("{:03}> Wusel ? on ({:2},{:2},{:2})", i, p.x, p.y, p.z);
        }
    }
}

#[test]
fn test_consume_bread() {
    // TODO refactor test.

    log::debug!("[TEST] Creating new stuff, let the wusels eat the bread.");
    let mut test_world: super::World = super::World::new(20, 5); // small world.
    log::debug!("Test World created");

    // Empty test_world tick.
    test_world.tick();
    log::debug!("Test World ticked");

    test_world.wusel_new(
        "Eater".to_string(),
        super::wusels::WuselGender::Female,
        super::areas::Position { x: 1, y: 0, z: 0 },
    );

    test_world.wusel_new(
        "Starver".to_string(),
        super::wusels::WuselGender::Male,
        super::areas::Position { x: 2, y: 0, z: 0 },
    );

    log::debug!("Test World's wusels created.");

    // Create food: transportable, no storage.
    let food1 = test_world.food_new("Bread", 100);

    let food1_id = food1;

    log::debug!("Test World's food created, index: {}.", food1_id);

    let food2 = test_world.object_duplicate(0).unwrap(); // unsafe, but must be true.

    let food2_id = food2;
    test_world.object_set_position(food2_id, test_world.position_random());

    log::debug!("Test World's food duplicated, index: {}.", food2_id);

    // Put a copy into the world.
    test_world.object_set_position(food1_id, test_world.position_random());

    log::debug!("Test World's food put onto a position.");

    // Get the food and transport it somewhere else.
    test_world.wusel_assign_to_task(1, super::tasks::TaskBuilder::use_object(food1_id, 1)); // take
    test_world.wusel_assign_to_task(
        1,
        super::tasks::TaskBuilder::move_to(test_world.position_random()),
    );
    test_world.wusel_assign_to_task(1, super::tasks::TaskBuilder::use_object(food1_id, 2)); // drop
    test_world.wusel_assign_to_task(
        1,
        super::tasks::TaskBuilder::move_to(test_world.position_random()),
    );
    test_world.wusel_assign_to_task(1, super::tasks::TaskBuilder::use_object(food1_id, 1)); // take not exisiting?

    // Let the other wusel wait, than it's tries to get the food as well, and consume it.
    test_world.wusel_assign_to_task(
        0,
        super::tasks::TaskBuilder::move_to(super::areas::Position {
            x: test_world.get_width() - 1,
            y: test_world.get_depth() - 1,
            z: 0,
        }),
    );
    test_world.wusel_assign_to_task(0, super::tasks::TaskBuilder::use_object(food1_id, 1)); // take as well.
    test_world.wusel_assign_to_task(
        0,
        super::tasks::TaskBuilder::move_to(test_world.position_random()),
    );
    test_world.wusel_assign_to_task(0, super::tasks::TaskBuilder::use_object(food1_id, 3)); // consume.
    test_world.wusel_assign_to_task(
        0,
        super::tasks::TaskBuilder::move_to(test_world.position_random()),
    );
    test_world.wusel_assign_to_task(
        0,
        super::tasks::TaskBuilder::move_to(test_world.position_random()),
    );
    log::debug!("Test World's task to work at the workbench assigned.");

    // show everyone's stats.
    for i in 0usize..2 {
        // test_world.wusel_show_tasklist(i); // tasks
        for n in crate::life::wusels::needs::Need::VALUES.iter() {
            test_world.wusel_set_need(i, n, 100);
        }
    }
    log::debug!("Test World's wusels' needs artificially reduced.");

    // Show the grid..
    let (_w, _h): (usize, usize) = (
        test_world.get_width() as usize,
        test_world.get_depth() as usize,
    );

    // clear the test screen
    println!(
        "{clear}{hide}",
        clear = termion::clear::All,
        hide = termion::cursor::Hide
    );

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
    }
}

/// Test doing tasks.
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

    // Or knife and Pot as part of the station.
    // Cut a meal, boil the meal => consumes tomatoes, creates tomato soup.

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

/// Test mutually meeting, which may cause deadlocks.
/// -----
/// 1at: [Read, Meet 2nd].
/// 2nd: [Meet 3rd]
/// 3rd: [Meet 4th]
/// 4th: [Meet 1st]
/// -----
/// 1at: [Read, Meet 2nd] + [Met by 4th]
/// 2nd: [Meet 3rd]
/// 3rd: [Meet 4th] + [Met by 2nd]
/// 4th: [Meet 1st] + [Met by 3rd]
/// -----
/// 1st done with reading and wants to meet 2nd.
/// -----
/// 1at: [Meet 2nd, Met by 4th]
/// 2nd: [Meet 3rd] + [Met by 1st]
/// 3rd: [Meet 4th, Met by 2nd]
/// 4th: [Meet 1st, Met by 3rd]
/// -----
/// Nothing happens, since everyone waits for the other to be done.
/// 2nd, 3rd and 4th stop meeting. (they waited too long)
/// -----
/// 1at: [Meet 2nd, Met by 4th]
/// 2nd: [Met by 1st]
/// 3rd: [Met by 2nd]
/// 4th: [Met by 3rd]
/// -----
/// The active meeter, they were about to be met by is gone, stop being met.
/// 1at: [Meet 2nd, Met by 4th]
/// 2nd: [Met by 1st]
/// 3rd: []
/// 4th: []
/// -----
/// 1st meets 2nd; 4th is not meeting 1st anymore. No tasks left.
#[test]
fn test_mutual_meeting() {
    // TODO refactor test.

    println!("[test] Mutual Meeting, causes for circular deadlocks.");
    let mut test_world: super::World = super::World::new(80, 30);

    // Empty test_world tick.
    test_world.tick();

    test_world.wusel_new(
        "1st".to_string(),
        super::wusels::WuselGender::Female,
        super::areas::Position { x: 1, y: 0, z: 0 },
    );

    test_world.wusel_new(
        "2nd".to_string(),
        super::wusels::WuselGender::Female,
        super::areas::Position { x: 3, y: 0, z: 0 },
    );
    test_world.wusel_new(
        "3rd".to_string(),
        super::wusels::WuselGender::Male,
        super::areas::Position { x: 5, y: 0, z: 0 },
    );

    test_world.wusel_new(
        "4th".to_string(),
        super::wusels::WuselGender::Male,
        super::areas::Position { x: 9, y: 0, z: 0 },
    );

    // 4 wusels created.
    assert_eq!(4, test_world.wusel_count());

    // => no preconditions.
    // => does 'nothing' for ticks steps.
    let reading: super::tasks::TaskBuilder =
        super::tasks::TaskBuilder::new(String::from("Reading")).set_duration(5); // ticks

    test_world.tick();

    // first wusel is also doing something else
    test_world.wusel_assign_to_task(0, reading.clone()); // do reading.

    // scenario: everyone wants too meet the next one.
    // mutual meeting.
    test_world.wusel_assign_to_task(
        0,
        super::tasks::TaskBuilder::meet_with(1, true, false).set_duration(7),
    );
    // mutual meeting.
    test_world.wusel_assign_to_task(
        1,
        super::tasks::TaskBuilder::meet_with(2, true, false).set_duration(7),
    );
    // mutual meeting.
    test_world.wusel_assign_to_task(
        2,
        super::tasks::TaskBuilder::meet_with(3, true, false).set_duration(7),
    );
    // mutual meeting.
    test_world.wusel_assign_to_task(
        3,
        super::tasks::TaskBuilder::meet_with(0, true, false).set_duration(7),
    );

    // 90 ticks later.
    for _ in 0..90 {
        test_world.tick();
        // println!("\nTasks at time {}:", test_world.get_time());
        // for w in 0..4 { test_world.wusel_show_tasklist(w); }
    }
}
