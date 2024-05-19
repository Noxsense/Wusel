use crate::config::*;
use crate::renderer::*;
use crate::save::*;
use crate::simulation::*;

/// The main method of the wusel world.
pub fn run(configfile: &str, savefile: &str) -> Result<(), std::io::Error> {
    let config = load_configuration(configfile).unwrap();
    let save = load_save(savefile).unwrap_or_else(new_save);

    let simulation_done = execute(config, &save, get_renderer(config)).unwrap();

    store_save(simulation_done)
}

fn execute(
    config: Config,
    save: &Save,
    renderer: impl Fn(&Save, UserView) -> Result<(), std::io::Error>,
) -> Result<Save, std::io::Error> {
    println!("Configuration: {:?}", config);
    println!("Save:          {:?}", save);

    // make clone of initial save.
    let mut simulating = save.clone();

    let max_iterations = config.max_iterations();

    let mut i = 0;
    while i < max_iterations {
        // run simulation.
        simulating = tick(&simulating)?;

        // render.
        if let Err(render_error) = renderer(&simulating, 0u8) {
            // interupt on renddr error.
            return Err(std::io::Error::new(
                render_error.kind(),
                format!("Render Error ({:?})", render_error),
            ));
        }

        i += 1;
    }

    Ok(simulating)
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::creature::*;
    use crate::model::world::*;

    #[test]
    fn should_simulate_time_within_the_execute() {
        // arrange.
        let config = Config::new(1, 11, 0u8);
        let renderer = |_: &_, _| Ok(()); // do nothing.
        let wusel0 = Wusel::new(WuselId::generate());
        let wusel1 = Wusel::new(WuselId::generate());
        let save = World::from(7, vec![wusel0, wusel1]);

        // act.
        let simulation_done = execute(config, &save, renderer).unwrap();

        // assert.
        assert_eq!(
            18u64,
            simulation_done.time(),
            "Time Passed within the save on normal time."
        );
        assert_eq!(7u64, save.time(), "Initial Save is untouched.");
    }

    #[test]
    fn should_yield_valid_saveable_world_on_every_execution_interruption() {
        // arrange.
        let config = Config::new(1, 11, 0u8);
        let renderer = |_: &_, _| Ok(()); // do nothing.
        let wusel0 = Wusel::new(WuselId::generate());
        let wusel1 = Wusel::new(WuselId::generate());
        let old_save = World::from(7, vec![wusel0, wusel1]);

        // act.
        let simulation_done = execute(config, &old_save, renderer).unwrap();
        let save_result = store_save(simulation_done);

        if let Err(_) = save_result {
            panic!("Could not save the simulated world.");
        }
    }
}
