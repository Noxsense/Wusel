use crate::model::creature::*;
use crate::model::world::*;

pub type Save = World;

pub fn load_save(wusel_save_file: &str) -> Option<Save> {
    let file = std::fs::File::open(wusel_save_file);

    if file.is_err() {
        return None;
    }

    let time = 42u64;
    let wusels = vec![Wusel::new(WuselId::generate())];

    Some(World::from(time, wusels))
}

pub fn new_save() -> Save {
    let time = 0u64;
    let wusels = vec![Wusel::new(WuselId::generate())];

    World::from(time, wusels)
}

pub fn store_save(_to_be_saved: Save) -> Result<(), std::io::Error> {
    // TODO write save to file.
    Ok(())
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_loads_last_save_if_available() {
        // TODO setup with save file
        let save = load_save("src/test-res/.wusel");

        let expected_time = 42;
        let expected_wusel_id = WuselId::from(0);
        let exepcted_wusel = Wusel::new(expected_wusel_id);
        // expected_wusel.position: Position { x: 0, y: 0, z: 0 },
        // expected_wusel.age: 0,
        // expected_wusel.nourishment: 10,
        // expected_wusel.wakefulness: 10,
        // expected_wusel.digestion: 0,
        // expected_wusel.tidiness: 10,
        let expected_world = World::from(expected_time, vec![exepcted_wusel]);
        assert_eq!(Some(expected_world), save);
    }

    #[test]
    fn should_loads_empty_save_if_not_provided() {
        // TODO setup no save file
        assert_eq!(None, load_save("src/test-res/res/.no_wusel_save"));
    }

    #[test]
    fn should_store_save() {
        let time = 2;
        let wusel = Wusel::new(WuselId::generate());
        // wusel.position: Position { x: 0, y: 0, z: 0 },
        // wusel.age: 0,
        // wusel.nourishment: 10,
        // wusel.wakefulness: 10,
        // wusel.digestion: 0,
        // wusel.tidiness: 10,
        let save = World::from(time, vec![wusel]);

        if let Err(_) = store_save(save) {
            assert!(false, "Storing the dave failed.");
        }
    }
}
