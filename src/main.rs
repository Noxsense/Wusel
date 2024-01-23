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

fn load_configuration(config_file_name: &str) -> Result<Config, std::io::Error> {
    let file = std::fs::File::open(config_file_name);
    if let Err(error) = file {
        return Err(error);
    }
    Ok(Config {
        velocity: 1,
        max_iterations: 10,
        renderer: 0,
    })
}

fn load_save(wusel_save_file: &str) -> Option<Save> {
    let file = std::fs::File::open(wusel_save_file);
    if file.is_err() {
        return None;
    }
    Some(World {
        time: 42u64,
        wusel: Wusel {
            position: Position { x: 0, y: 0, z: 0 },
            age: 0,
            nourishment: 10,
            wakefulness: 10,
            digestion: 0,
            tidiness: 10,
        },
    })
}

fn new_save() -> Save {
    World {
        time: 0,
        wusel: Wusel::new(),
    }
}

fn store_save(to_be_saved: Save) -> Result<(), std::io::Error> {
    Ok(())
}

fn get_renderer(config: Config) -> impl Fn(Save, UserView) -> Result<(), std::io::Error> {
    match config.renderer {
        // log renderer.
        0u8 => |save, view| {
            println!("view: {:?}, save: {:?}", view, save);
            Ok(())
        },

        // TODO graphical ascii renderer

        // TODO graphical renderer

        // default renderer, muted renderer,  not even log.
        _ => |_, _| {
            Ok(())
        },
    }
}

fn run(
    config: Config,
    save: Save,
    renderer: impl Fn(Save, UserView) -> Result<(), std::io::Error>,
) -> Result<Save, std::io::Error> {
    println!("Configuration: {:?}", config);
    println!("Save:          {:?}", save);

    // make clone of initial save.
    let mut simulating = save;

    let mut i = 0;
    while i < config.max_iterations {
        // run simulation.
        simulating = tick(simulating)?;

        // render.
        if let Err(render_error) = renderer(simulating, 0u8) {
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

fn tick(last_save: Save) -> Result<Save, std::io::Error> {
    Ok(World {
        time: last_save.time.saturating_add(1),
        wusel: Wusel {
            position: last_save.wusel.position,
            age: last_save.wusel.age.saturating_add(1),
            nourishment: last_save.wusel.nourishment.saturating_sub(1),
            wakefulness: last_save.wusel.wakefulness.saturating_sub(1),
            digestion: last_save.wusel.digestion.saturating_sub(1),
            tidiness: last_save.wusel.tidiness.saturating_sub(1),
        },
    })
}

/// Configuration of the game start.
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Config {
    /// how many simulated time units are played withim one real time unit. (normal: 1)
    velocity: usize,

    /// max iterations (debug): how many iterations should the simulation run
    max_iterations: usize,

    /// renderer used for the programm.
    renderer: u8,
}

type Save = World;
type UserView = u8;

/// World.
/// View of the World and life.
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct World {
    // TODO: placeholder.
    /// how many simulated time units are played withim one real time unit. (normal: 1)
    time: u64,
    wusel: Wusel,
}

/// Wusel.
/// A living object, "that discovers the wolrd."
/// A bundle of needs position in the world.
/// For Living it has an own drive to survive, explore and sozialize.
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Wusel {
    /// Position of the wusel.
    position: Position,

    /// Age of a wusel.
    /// when it is born, the age starts, if the wusel is too old, it dies.
    age: u64,

    /// Life Need for Sleep; if too low, the Wusel falls asleep and must rest.
    /// while sleepy, a wusel is also slower.
    wakefulness: u64,

    /// Life Need for Food and Water; if too low, the Wusel starves and dies.
    /// while hungry, the wusel is not very focussed, or distractable.
    nourishment: u64,

    /// After taking Food and Water, the digestion starts (progress).
    /// if the digestions is done, the Wusel needs to get rid of digested materials.
    digestion: u64,

    /// Level of Tidiness.
    /// If The Wusel gets too dirty, the immune system is weakened,
    /// and it may feel uncomfortable.
    tidiness: u64,

}

impl Wusel {
    fn new() -> Self {
        Self {
            // root position for new wusel.
            position: Position {
                x: 0,
                y: 0,
                z: 0,
            },

            // just born.
            age: 0,

            // not about to fall asleep.
            wakefulness: 10,

            // not about to starve to dead.
            nourishment: 10,

            // digesting nothing.
            digestion: 0,

            // not that dirty.
            tidiness: 10,
        }
    }
}

/// Position.
/// A point in the world with three coordinates.
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Position {
    /// width (bird's eye: horizontal axis)
    x: u64,

    /// depth (bird's eye: vertical axis)
    y: u64,

    /// height (bird's eye: upper may cover below, up to certsin level)
    z: u64,
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
        let save = crate::load_save("src/test-res/.wusel");
        assert_eq!(
            Some(crate::World {
                time: 42u64,
                wusel: crate::Wusel {
                    position: crate::Position { x: 0, y: 0, z: 0 },
                    age: 0,
                    nourishment: 10,
                    wakefulness: 10,
                    digestion: 0,
                    tidiness: 10,
                },
            }),
            save
        );
    }

    #[test]
    fn should_loads_empty_save_if_not_provided() {
        // TODO setup no save file
        assert_eq!(None, crate::load_save("src/test-res/res/.no_wusel_save"));
    }

    #[test]
    fn should_store_save() {
        let save = crate::World {
            time: 2,
            wusel: crate::Wusel {
                position: crate::Position { x: 0, y: 0, z: 0 },
                age: 0,
                nourishment: 10,
                wakefulness: 10,
                digestion: 0,
                tidiness: 10,
            },
        };
        if let Err(_) = crate::store_save(save) {
            assert!(false, "Storing the dave failed.");
        }
    }

    #[test]
    fn should_simulate_time_within_the_run() {
        let save = crate::World {
            time: 7,
            wusel: crate::Wusel::new(),
        };
        let simulation_done = crate::run(
            crate::Config {
                velocity: 1,
                max_iterations: 11,
                renderer: 0u8,
            },
            save,
            |_, _| Ok(()),
        )
        .unwrap();
        assert_eq!(
            18u64, simulation_done.time,
            "Time Passed within the save on normal time."
        );
        assert_eq!(7u64, save.time, "Initial Save is untouched.");
    }

    #[test]
    fn should_simulate_time_within_the_tick() {
        let save = crate::World {
            time: 7,
            wusel: crate::Wusel::new(),
        };
        let simulation_done = crate::tick(save).unwrap();
        assert_eq!(
            save.time + 1,
            simulation_done.time,
            "Time Passed ticked one time."
        );
    }

    #[test]
    fn should_decrease_fullness_of_wusel_wellbeing_every_tick() {
        let save = crate::World {
            time: 0,
            wusel: crate::Wusel {
                position: crate::Position {x: 0, y: 0, z: 0, },
                age: 0u64,
                wakefulness: 10u64,
                nourishment: 10u64,
                digestion: 10u64,
                tidiness: 10u64,
            },
        };

        let simulation_result = crate::tick(save).unwrap();

        let wusel_ticked = simulation_result.wusel;
        assert!(wusel_ticked.wakefulness < save.wusel.wakefulness);
        assert!(wusel_ticked.nourishment < save.wusel.nourishment);
        assert!(wusel_ticked.digestion < save.wusel.digestion);
        assert!(wusel_ticked.tidiness < save.wusel.tidiness);
    }

    #[test]
    fn should_increase_wusel_age_every_tick() {
        let save = crate::World {
            time: 0,
            wusel: crate::Wusel {
                position: crate::Position {x: 0, y: 0, z: 0, },
                age: 0,
                wakefulness: 0,
                nourishment: 0,
                digestion: 0,
                tidiness: 0,
            },
        };

        let simulation_result = crate::tick(save).unwrap();

        let wusel_ticked = simulation_result.wusel;
        assert!(wusel_ticked.age > save.wusel.age);
    }
}
