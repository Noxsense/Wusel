//! # Wusel - Game
//!
//! This is a life simulation game where life is given to multiple wusels whose life can be in your
//! hand, otherwise they will try really hard to keep them alive on their own and you can watch
//! their cute little waddling and 'wuseln'.
//!
//! ## Author
//! Ngoc (Nox) Le <noxsense@gmail.com>

use wusel::*;

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
