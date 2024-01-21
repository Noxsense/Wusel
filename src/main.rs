//! # Wusel - Game
//!
//! This is a life simulation game where life is given to multiple wusels whose life can be in your
//! hand, otherwise they will try really hard to keep them alive on their own and you can watch
//! their cute little waddling and 'wuseln'.
//!
//! ## Author
//! Ngoc (Nox) Le <noxsense@gmail.com>

/// The main method of the wusel world.
fn main() -> Result<(), std::io::Error> {
    env_logger::init();
    let config = load_configuration("res/config.yaml").unwrap();
    let save = load_save("res/.wusel").unwrap_or_else(new_save);

    let simulation_done = run(config, save, get_renderer(config)).unwrap();

    store_save(simulation_done)
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Config {
    /// how many simulated time units are played withim one real time unit. (normal: 1)
    velocity: usize,

    /// max iterations (debug): how many iterations should the simulation run
    max_iterations: usize,
}

type Save = u64;
type UserView = u8;


fn load_configuration(config_file_name: &str) -> Result<Config, std::io::Error> {
    let file = std::fs::File::open(config_file_name);
    if let Err(error) = file {
        return Err(error);
    }
    Ok(Config { velocity: 1, max_iterations: 10})
}

fn load_save(wusel_save_file: &str) -> Option<Save> {
    let file = std::fs::File::open(wusel_save_file);
    if let Err(error) = file {
        return None;
    }
    Some(42u64)
}

fn new_save() -> Save {
    21u64
}

fn store_save(to_be_saved: Save) -> Result<(), std::io::Error> {
    Ok(())
}

fn get_renderer(config: Config) -> impl Fn(Save, UserView) -> Result<(), std::io::Error> {
    |save, view| {
        println!("render: {}, user_view: {}", save, view);
        Ok(())
    }
}

fn run(
    config: Config,
    save: Save,
    renderer: impl Fn(Save, UserView) -> Result<(), std::io::Error>
) -> Result<Save, std::io::Error> {
    println!("Configuration: {:?}", config);
    println!("Save:          {:?}", save);

    // make clone of initial save.
    let mut simulating = *&save;

    let mut i = 0;
    while i < config.max_iterations {
        // run simulation.
        simulating = simulating.saturating_add(1);

        // render.
        renderer(simulating, 0u8);
        i += 1;
    }

    Ok(simulating)
}

//////

#[cfg(test)]
mod main_test {
    #[test]
    fn should_loads_default_configloading() {
        // TODO setup: with configuration file
        let config = crate::load_configuration("src/test-res/config.yaml").unwrap();
        assert_eq!(1usize, config.velocity);
        assert_eq!(10usize, config.max_iterations);
    }

    #[test]
    fn should_fails_if_no_configuration_is_found() {
        // TODO setup: no configuration file
        if let Err(error) = crate::load_configuration("src/test-res/not_existent.yaml") {
            assert_eq!("No such file or directory (os error 2)", error.to_string());
        } else {
            assert!(false, "The Configuration was not found and should fail")
        }
    }

    #[test]
    fn should_loads_last_save_if_available() {
        // TODO setup with save file
        assert_eq!(Some(42u64), crate::load_save("src/test-res/.wusel"));
    }

    #[test]
    fn should_loads_empty_save_if_not_provided() {
        // TODO setup no save file
        assert_eq!(None, crate::load_save("src/test-res/res/.no_wusel_save"));
    }

    #[test]
    fn should_store_save() {
        if let Err(_) = crate::store_save(42u64) {
            assert!(false, "Stroing the dave failed.");
        }
    }

    #[test]
    fn should_simulate_time_within_the_run() {
        let save = 0u64;
        let simulation_done = crate::run(
            crate::Config { velocity: 1, max_iterations: 11 },
            save,
            |_, _| Ok(()),
        ).unwrap();
        assert_eq!(11u64, simulation_done, "Time Passed within the save on normal time.");
        assert_eq!(0u64, save, "Initial Save is untouched.");
    }
}
