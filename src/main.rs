//! # Wusel - Game
//!
//! This is a life simulation game where life is given to multiple wusels whose life can be in your
//! hand, otherwise they will try really hard to keep them alive on their own and you can watch
//! their cute little waddling and 'wuseln'.
//!
//! ## Author
//! Ngoc (Nox) Le <noxsense@gmail.com>

use wusel::config::*;
use wusel::save::*;
use wusel::renderer::*;
use wusel::simulation::*;

/// The main method of the wusel world.
fn main() -> Result<(), std::io::Error> {
    env_logger::init();
    let config = load_configuration("res/config.yaml").unwrap();
    let save = load_save("res/.wusel").unwrap_or_else(new_save);

    let simulation_done = run(config, &save, get_renderer(config)).unwrap();

    store_save(simulation_done)
}

pub fn run(
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
    use wusel::model::creature::*;
    use wusel::model::world::*;

    use super::*;

    #[test]
    fn should_simulate_time_within_the_run() {
        // arrange.
        let save = World::from(7, vec![Wusel::new(WuselId::generate())]);
        let config = Config::new(1, 11, 0u8);
        let renderer = |_: &_, _| { Ok(()) };

        // act.
        let simulation_done = run(config, &save, renderer).unwrap();

        // assert.
        assert_eq!(
            18u64,
            simulation_done.time(),
            "Time Passed within the save on normal time."
        );
        assert_eq!(7u64, save.time(), "Initial Save is untouched.");
    }
}
