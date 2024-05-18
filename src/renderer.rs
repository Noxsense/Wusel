use crate::config::*;
use crate::save::*;

pub type UserView = u8;

pub fn get_renderer(config: Config) -> impl Fn(&Save, UserView) -> Result<(), std::io::Error> {
    match config.renderer() {
        // log renderer.
        0u8 => log_renderer,

        // TODO graphical ascii renderer

        // TODO graphical renderer

        // default renderer, muted renderer,  not even log.
        _ => mute_renderer,
    }
}

pub fn log_renderer(save: &Save, view: UserView) -> Result<(), std::io::Error> {
    println!("view: {:?}, save: {:?}", view, save);
    Ok(())
}

pub fn mute_renderer(_: &Save, _: UserView) -> Result<(), std::io::Error> {
    Ok(())
}
