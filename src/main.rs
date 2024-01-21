//! # Wusel - Game
//!
//! This is a life simulation game where life is given to multiple wusels whose life can be in your
//! hand, otherwise they will try really hard to keep them alive on their own and you can watch
//! their cute little waddling and 'wuseln'.
//!
//! ## Author
//! Ngoc (Nox) Le <noxsense@gmail.com>

type Config = usize;
type Save = u64;

/// The main method of the wusel world.
fn main() -> Result<(), std::io::Error> {
    env_logger::init();
    let config = load_configuration("res/config.yaml").unwrap();
    let save = load_save().unwrap_or_else(new_save);
    let simulation_done = run(config, save).unwrap();
    store_save(simulation_done)
}


fn load_configuration(config_file_name: &str) -> Result<Config, std::io::Error> {
    let file = std::fs::File::open(config_file_name);
    if let Err(error) = file {
        return Err(error);
    }
    Ok(42usize)
}

fn load_save() -> Option<Save> {
    Some(42u64)
}

fn new_save() -> Save {
    21u64
}

fn store_save(to_be_saved: Save) -> Result<(), std::io::Error> {
    Ok(())
}

fn run(config: Config, save: Save) -> Result<Save, std::io::Error> {
    println!("Configuration: {}", config);
    println!("Save:          {}", save);
    Ok(save)
}

//////

#[cfg(test)]
mod main_test {
    #[test]
    fn should_loads_default_configloading() {
        // TODO setup: with configuration file
        assert_eq!(42usize, crate::load_configuration("res/config.yaml").unwrap());
    }

    #[test]
    fn should_fails_if_no_configuration_is_found() {
        // TODO setup: no configuration file
        if let Err(error) = crate::load_configuration("/xxx/not_existent.yaml") {
            assert_eq!("No such file or directory (os error 2)", error.to_string());
        } else {
            assert!(false, "The Configuration was not found and should fail")
        }
    }

    #[test]
    fn should_loads_last_save_if_available() {
        // TODO setup with save file
        assert_eq!(Some(42u64), crate::load_save());
    }

    #[test]
    fn should_loads_empty_save_if_not_provided() {
        // TODO setup no save file
        assert_eq!(None, crate::load_save());
    }

    #[test]
    fn should_store_save() {
        if let Err(_) = crate::store_save(42u64) {
            assert!(false, "Stroing the dave failed.");
        }
    }
}
