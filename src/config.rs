/// Configuration of the game start.
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct Config {
    /// how many simulated time units are played withim one real time unit. (normal: 1)
    velocity: usize,

    /// max iterations (debug): how many iterations should the simulation run
    max_iterations: usize,

    /// renderer used for the programm.
    renderer: u8,
}

pub fn load_configuration(config_file_name: &str) -> Result<Config, std::io::Error> {
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

///////////////////////////////////////////////////////////////////////////////

impl Config {
    pub fn new(velocity: usize, max_iterations: usize, renderer: u8) -> Self {
        Self {
            velocity,
            max_iterations,
            renderer,
        }
    }

    pub fn velocity(&self) -> usize {
        self.velocity
    }

    pub fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    pub fn renderer(&self) -> u8 {
        self.renderer
    }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_loads_default_configloading() {
        // TODO setup: with configuration file
        let config = load_configuration("src/test-res/config.yaml").unwrap();
        assert_eq!(1usize, config.velocity);
        assert_eq!(10usize, config.max_iterations);
    }

    #[test]
    fn should_fails_if_no_configuration_is_found() {
        // TODO setup: no configuration file
        if let Err(error) = load_configuration("src/test-res/not_existent.yaml") {
            assert_eq!("No such file or directory (os error 2)", error.to_string());
        } else {
            assert!(false, "The Configuration was not found and should fail")
        }
    }
}
