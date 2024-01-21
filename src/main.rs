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
    let config = load_configuration().unwrap();
    let save = load_save();
    run(config, save)
}

fn load_configuration() -> Result<usize, std::io::Error> {
    Ok(42usize)
}

fn load_save() -> u64 {
    42u64
}

fn run(
    config: usize,
    save: u64
    ) -> Result<(), std::io::Error> {
    println!("Configuration: {}", config);
    println!("Save:          {}", save);
    Ok(())
}


//////

#[cfg(test)]
mod main_test {
    #[test]
    fn should_loads_default_configloading() {
        // TODO setup: with configuration file
        assert_eq!(42usize, crate::load_configuration().unwrap());
    }

    #[test]
    fn should_fails_if_no_configuration_is_found() {
        // TODO setup: no configuration file
        if let Err(error) = crate::load_configuration() {
            assert_eq!("Config Not Found", error.to_string());
        } else {
            assert!(false, "The Configuration was not found and should fail")
        }
    }

    #[test]
    fn should_loads_last_save_if_available() {
        // TODO setup with save file
        assert_eq!(42u64, crate::load_save());
    }

    #[test]
    fn should_loads_empty_save_if_not_provided() {
        // TODO setup no save file
        assert_eq!(42u64, crate::load_save());
    }
}
