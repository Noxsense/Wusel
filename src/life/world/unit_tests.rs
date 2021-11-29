#![cfg(test)]

extern crate env_logger;

use crate::life;
use crate::life::areas;
use crate::life::objects;
use crate::life::tasks;
#[allow(unused_imports)] use crate::life::world;
use crate::life::wusel;

#[allow(dead_code)]
fn init_log() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn wusel_init() {
    init_log();

    let wusel: wusel::Wusel;

    wusel = wusel::Wusel::new(
        0,
        "Wusel Name Start".to_string(),
        wusel::WuselGender::Female,
    );

    assert_eq!(wusel.get_id(), 0usize);
    assert_eq!(wusel.get_name(), "Wusel Name Start".to_string());
    assert_eq!(wusel.get_gender(), wusel::WuselGender::Female);

    assert_eq!(wusel.is_alive(), true);

    for &need in wusel::Need::VALUES.iter() {
        assert_eq!(wusel.get_need(need), need.get_full()); // init_log full.
    }

    for &ability in wusel::Ability::VALUES.iter() {
        assert_eq!(wusel.get_ability(ability), 0u32);
    }

    assert!(!wusel.is_pregnant());
    assert_eq!(wusel.get_other_parent(), None);
    assert_eq!(wusel.get_remaining_pregnancy_days(), None);

    assert!(wusel.has_tasklist_empty());
    assert_eq!(wusel.get_tasklist_len(), 0usize);
    assert_eq!(wusel.get_tasklist_names().len(), 0usize);
    assert_eq!(wusel.get_next_task_index_with(&|_| true), None);
}

#[test]
fn wusel_rename_and_co() {
    init_log();

    let mut wusel: wusel::Wusel;

    wusel = wusel::Wusel::new(
        0,
        "Wusel Name Start".to_string(),
        wusel::WuselGender::Female,
    );

    assert_eq!(wusel.get_id(), 0usize);
    assert_eq!(wusel.get_name(), "Wusel Name Start".to_string());
    assert_eq!(wusel.get_gender(), wusel::WuselGender::Female);

    wusel.set_name("NewName".to_string());
    assert_eq!(wusel.get_name(), "NewName".to_string());

    wusel.set_gender(wusel::WuselGender::Male);
    assert_eq!(wusel.get_gender(), wusel::WuselGender::Male);

    // rest stayes the same.

    assert_eq!(wusel.is_alive(), true);

    for &need in wusel::Need::VALUES.iter() {
        assert_eq!(wusel.get_need(need), need.get_full()); // init_log full.
    }

    for &ability in wusel::Ability::VALUES.iter() {
        assert_eq!(wusel.get_ability(ability), 0u32);
    }

    assert!(!wusel.is_pregnant());
    assert_eq!(wusel.get_other_parent(), None);
    assert_eq!(wusel.get_remaining_pregnancy_days(), None);

    assert!(wusel.has_tasklist_empty());
    assert_eq!(wusel.get_tasklist_len(), 0usize);
    assert_eq!(wusel.get_tasklist_names().len(), 0usize);
    assert_eq!(wusel.get_next_task_index_with(&|_| true), None);
}

#[test]
fn wusel_tick_to_death() {
    init_log();

    let mut wusel: wusel::Wusel;

    wusel = wusel::Wusel::new(
        0,
        "Wusel Name Start".to_string(),
        wusel::WuselGender::Female,
    );

    assert!(wusel.is_alive());

    for &need in wusel::Need::VALUES.iter() {
        assert_eq!(wusel.get_need(need), need.get_full()); // init_log full.
    }

    for &ability in wusel::Ability::VALUES.iter() {
        assert_eq!(wusel.get_ability(ability), 0u32);
    }

    let water_full = wusel::Need::WATER.get_full();

    log::debug!("Start ticking, {} times.", water_full);

    // almost to die of thirst.
    for i in 0..water_full - 1 {
        log::debug!("- {} Tick.", i);
        wusel.wusel_tick(i % life::DAY == 0 && i > 0); // day.
        assert_ne!(wusel.get_need(wusel::Need::WATER), 0u32);
        assert_ne!(wusel.get_need(wusel::Need::WATER), water_full);
        assert!(wusel.is_alive());
    }

    let days_passed = water_full / life::DAY;
    let day_range = (days_passed - 1)..(days_passed + 1);

    println!(
        "Dayes passed: {}, age: {}",
        days_passed,
        wusel.get_lived_days()
    );

    // around the days old.
    assert!(day_range.contains(&wusel.get_lived_days()));

    assert!(wusel.is_alive());
    assert_eq!(wusel.get_need(wusel::Need::WATER), 1u32);

    assert!(!wusel.is_pregnant());
    assert_eq!(wusel.get_other_parent(), None);
    assert_eq!(wusel.get_remaining_pregnancy_days(), None);

    assert!(wusel.has_tasklist_empty());
    assert_eq!(wusel.get_tasklist_len(), 0usize);
    assert_eq!(wusel.get_tasklist_names().len(), 0usize);
    assert_eq!(wusel.get_next_task_index_with(&|_| true), None);

    wusel.wusel_tick(true);
    assert_eq!(wusel.is_alive(), false); // just died.
}

#[test]
fn wusel_impregnate() {
    init_log();

    let mut wusel0 = wusel::Wusel::new(0, "Wusel0".to_string(), wusel::WuselGender::Female);
    let mut wusel1 = wusel::Wusel::new(1, "Wusel1".to_string(), wusel::WuselGender::Female); // can also be female, no restrictions.

    wusel0.set_pregnancy(Some(wusel1.get_id()), Some(3)); // 3 days pregnant with wusel1's child.

    assert!(wusel0.is_pregnant());
    assert!(!wusel1.is_pregnant()); // not pregnant.

    assert!(matches!(wusel0.get_other_parent(), Some(id) if id == wusel1.get_id()));
    assert!(matches!(wusel0.get_remaining_pregnancy_days(), Some(days) if days == 3));

    for i in 1..4 {
        // tick day.
        wusel0.wusel_tick(true);
        wusel1.wusel_tick(true);
        assert_eq!(wusel0.get_lived_days(), i);
        assert_eq!(wusel1.get_lived_days(), i);
    }

    assert!(wusel0.is_pregnant()); // still pregnant
    assert!(!wusel1.is_pregnant()); // still not pregnant.

    assert!(matches!(wusel0.get_other_parent(), Some(id) if id == wusel1.get_id())); // still the other parent.
    assert!(matches!(wusel0.get_remaining_pregnancy_days(), Some(days) if days == 0)); // no remaning days.

    // would now be resolved by the world.

    // define gender.
    // set parents.

    let baby = wusel::Wusel::new(2, "Baby".to_string(), wusel::WuselGender::random());
    wusel0.set_pregnancy(None, None); // done.

    assert_eq!(baby.get_lived_days(), 0);
    assert!(baby.is_alive());

    // assert(parents are set.)

    // yay.
}

#[test]
fn wusel_test_assignment() {
    init_log();

    let mut wusel0 = wusel::Wusel::new(0, "Wusel0".to_string(), wusel::WuselGender::Female);
    let wusel1 = wusel::Wusel::new(1, "Wusel1".to_string(), wusel::WuselGender::Female); // can also be female, no restrictions.

    let init_time = 0;
    let friendly = true;
    let romantically = true;

    let food1_id: objects::ObjectId = (objects::ObjectType::Food, "Bread", 0); // (ObjectType, ObjectSubtype, 0);

    wusel0.assign_to_task(
        init_time,
        tasks::TaskBuilder::move_to(areas::Position { x: 0, y: 4, z: 0 }),
    );
    wusel0.assign_to_task(init_time, tasks::TaskBuilder::use_object(food1_id, 1));
    wusel0.assign_to_task(
        init_time,
        tasks::TaskBuilder::meet_with(wusel1.get_id(), friendly, romantically),
    );

    assert!(!wusel0.has_tasklist_empty());
    assert!(wusel1.has_tasklist_empty()); // does nothing.
    assert_eq!(wusel0.get_tasklist_len(), 3);
    assert_eq!(wusel1.get_tasklist_len(), 0);

    if let Some(task) = wusel0.peek_ongoing_task() {
        if let tasks::TaskTag::MoveToPos(pos) = task.get_passive_part() {
            let areas::Position { x, y, z } = pos;
            assert_eq!(x, 0);
            assert_eq!(y, 4);
            assert_eq!(z, 0);
        } else {
            assert!(false);
        }
    } else {
        assert!(false);
    }

    let opt_index: Option<usize> =
        wusel0.get_next_task_index_with(&|task: &tasks::Task| match task.get_passive_part() {
            tasks::TaskTag::MeetWith(other_id, friend, lover) => {
                other_id == wusel1.get_id() && friend == friendly && lover == romantically
            }
            _ => false,
        });

    println!("Got index: {}", opt_index.unwrap_or(999));

    assert!(opt_index.is_some());
    let index = opt_index.unwrap();
    assert_ne!(index, wusel0.get_tasklist_len() - 1); // != ongoing
    assert_eq!(index, 0); // last set.

    let prioritize_task_success = wusel0.prioritize_task(index);
    assert!(prioritize_task_success);
}
