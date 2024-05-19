//! # Wusel - Game
//!
//! This is a life simulation game where life is given to multiple wusels whose life can be in your
//! hand, otherwise they will try really hard to keep them alive on their own and you can watch
//! their cute little waddling and 'wuseln'.
//!
//! ## Author
//! Ngoc (Nox) Le <noxsense@gmail.com>

use wusel::app;

/// The main method of the wusel world.
fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    app::run("res/config.yaml", "res/.wusel")
}
