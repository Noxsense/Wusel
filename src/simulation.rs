use crate::model::creature::*;
use crate::model::world::*;

pub fn tick(last_save: &World) -> Result<World, std::io::Error> {
    let new_time = last_save.time() + 1;
    let mut updated_wusels = vec![];

    // apply on all wusels:

    for wusel0 in last_save.wusels_iter() {
        let mut wusel: Wusel;
        wusel = increase_age(wusel0);
        wusel = decrease_needs(&wusel);
        updated_wusels.push(wusel);
    }

    Ok(World::from(new_time, updated_wusels))
}

fn increase_age(wusel: &Wusel) -> Wusel {
    let mut copy = *wusel;
    copy.set_age(wusel.age().saturating_add(1));
    copy
}

fn decrease_needs(wusel: &Wusel) -> Wusel {
    let mut copy = *wusel;
    copy.set_nourishment(copy.nourishment().saturating_sub(1));
    copy.set_wakefulness(copy.wakefulness().saturating_sub(1));
    copy.set_digestion(copy.digestion().saturating_sub(1));
    copy.set_tidiness(copy.tidiness().saturating_sub(1));
    copy
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_simulate_time_within_the_tick() {
        let save = World::from(7, vec![Wusel::new(WuselId::generate())]);

        let simulation_done = tick(&save).unwrap();

        assert_eq!(
            save.time() + 1,
            simulation_done.time(),
            "Time Passed ticked one time."
        );
    }

    #[test]
    fn should_decrease_fullness_of_wusel_wellbeing_every_tick() {
        let wusel_id = WuselId::generate();
        let mut wusel = Wusel::new(wusel_id);
        wusel.set_position(Position::from(0, 0, 0));
        wusel.set_age(0u64);
        wusel.set_wakefulness(10u64);
        wusel.set_nourishment(10u64);
        wusel.set_digestion(10u64);
        wusel.set_tidiness(10u64);

        let save = World::from(0, vec![wusel]);
        let wusel_old = save.wusel_by_id(wusel_id).unwrap();

        // act.
        let simulation_result = tick(&save).unwrap();
        let wusel_udpated = simulation_result.wusel_by_id(wusel_id).unwrap();

        // assert.
        assert!(wusel_udpated.wakefulness() < wusel_old.wakefulness());
        assert!(wusel_udpated.nourishment() < wusel_old.nourishment());
        assert!(wusel_udpated.digestion() < wusel_old.digestion());
        assert!(wusel_udpated.tidiness() < wusel_old.tidiness());
    }

    #[test]
    fn should_increase_wusel_age_every_tick() {
        let wusel_id = WuselId::generate();
        let mut wusel = Wusel::new(wusel_id);
        wusel.set_position(Position::from(0, 0, 0));
        wusel.set_age(0u64);
        wusel.set_wakefulness(10u64);
        wusel.set_nourishment(10u64);
        wusel.set_digestion(10u64);
        wusel.set_tidiness(10u64);

        let save = World::from(0, vec![wusel]);
        let wusel_old = save.wusel_by_id(wusel_id).unwrap();

        // act.
        let simulation_result = tick(&save).unwrap();
        let wusel_udpated = simulation_result.wusel_by_id(wusel_id).unwrap();

        // assert.
        assert!(wusel_udpated.age() > wusel_old.age());
    }
}
