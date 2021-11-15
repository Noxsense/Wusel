extern crate rand;

use terminal_size;

pub mod life;

use std::io;
// use std::io::{Read, Write, stdout, stdin};
// use termion::raw::IntoRawMode;

/** The main method of the wusel world. */
fn main() -> Result<(), io::Error> {
    // initiate the logger.
    env_logger::init();

    // use terminal_size::{Width, Height, terminal_size};

    let width: u32;
    let height: u32;

    let size = terminal_size::terminal_size();
    if let Some((terminal_size::Width(w), terminal_size::Height(h))) = size {
        width = w as u32 - (2 * 3);
        height = (h as u32) - (2 * 3) - 4; // minus gap for time.
    } else {
        width = 80;
        height = 30;
    }

    let mut world: life::World = life::World::new(width, height);
    println!(
        "Created a new world: w:{w}, h:{h}",
        w = world.get_width(),
        h = world.get_depth()
    );

    /* Empty world tick. */
    world.tick();

    world.wusel_new("1st".to_string(), true, life::Position::new(0, 0)); // female
    world.wusel_new("2nd".to_string(), true, life::Position::new(20, 0)); // female
    world.wusel_new("3rd".to_string(), false, life::Position::new(30, 0)); // male
    world.wusel_new("4th".to_string(), false, life::Position::new(40, 0)); // male

    /* Transportable bibimbap (korean food) */
    let bibimbap = world.food_new("Bibimbap", 10);
    let (bibimbap_id, _bibimbap_index) = bibimbap;

    /* Position. */
    world.object_set_position(bibimbap_id, world.position_random());

    /* Create an easy talk, without any preconditions.
     * => no preconditions.
     * => does 'nothing' for ticks steps. */
    let reading: life::TaskBuilder = life::TaskBuilder::new(String::from("Reading"))
        // .set_passive_part(String::from("Any Book"))
        .set_duration(5 /*ticks*/);

    world.tick();

    // wusel.improve(life::Ability::COOKING);
    // wusel.improve(life::Ability::COMMUNICATION);
    // wusel.improve(life::Ability::FITNESS);
    // wusel.improve(life::Ability::FINESSE);

    println!("World Clock: {}", world.get_time());

    /* Every 500 ms, make a tick. */
    let duration = std::time::Duration::from_millis(125);

    // hide cursor?
    println!(
        "{clear}{hide}",
        clear = termion::clear::All,
        hide = termion::cursor::Hide
    );

    /* Draw the field and make some real automation. */
    let (w, h) = (world.get_width() as usize, world.get_depth() as usize);

    for _ in 0..150 {
        // world.positions_recalculate_grid();
        render_field(w, h, world.positions_for_grid());

        /* Tick the world, maybe print the ongoing tasks. */
        print!("Time: {}\n", world.get_time());
        world.tick();

        /* Give some unbusy wusels the task to move circumference. */
        let unbusy = world.wusel_get_all_unbusy();
        let wusel_len = world.wusel_count();
        for widx in unbusy {
            let r = rand::random::<usize>() % (4 * wusel_len);
            match r {
                i if i < wusel_len && i != widx => {
                    /* Meet randomly with someone: Let [widx] meet [i], if i in [0..|w|). */
                    world.wusel_assign_task(
                        widx,
                        life::TaskBuilder::meet_with(i, true, true).set_duration(10),
                    );
                }
                i if i >= wusel_len && i < 2 * wusel_len => {
                    /* Walk randomly somewhere, if i not an wusel index. */
                    world.wusel_assign_task(
                        widx,
                        life::TaskBuilder::move_to(world.position_random()),
                    );
                }
                i if i >= 2 * wusel_len && i < 3 * wusel_len => {
                    /* Interact with the object. */
                    world.wusel_assign_task(
                        widx,
                        life::TaskBuilder::use_object(bibimbap_id, 0), // view
                    );
                }
                _ => {} // do nothing randomly.
            }
        }

        /* Draw selected wusel's needs (right position below field). */
        // TODO

        std::thread::sleep(duration); // wait.
    }

    Ok(())
}

/** Clean he view and draw the field, put the cursor, two lines below the field, to write there. */
fn render_field(w: usize, h: usize, positions: Vec<Vec<(char, usize)>>) {
    /* Draw field. */
    for p in 0..positions.len() {
        /* All things on this position. */
        let on_pos = &positions[p];

        let (x, y): (u16, u16);
        x = (p % w) as u16 + 2;
        y = (p / w) as u16 + 2;

        let color_bg: termion::color::Rgb;
        let color_fg: termion::color::Rgb;
        let render_symbol: char;

        // TODO (2021-11-15) from position, get form and colour.
        let position_is_free = on_pos.len() < 1;

        if position_is_free {
            color_fg = termion::color::Rgb(0, 255, 0);
            render_symbol = '`';
        } else {
            color_fg = termion::color::Rgb(255, 0, 0);
            render_symbol = on_pos[0].0;
        }

        color_bg = termion::color::Rgb(92, 194, 97);

        /* Draw position symbol. */
        print!(
            "{hide}{pos}{color_fg}{color_bg}{render_symbol}",
            pos = termion::cursor::Goto(x, y),
            color_bg = termion::color::Bg(color_bg),
            color_fg = termion::color::Fg(color_fg),
            render_symbol = render_symbol,
            hide = termion::cursor::Hide,
        );
    }

    /* Draw border. */
    let mut i: u16 = 0;
    let (w2, h2): (u16, u16) = (w as u16 + 2, h as u16 + 2);
    let circumference: u16 = (w2 * h2) as u16;

    let border_top: u16 = 0;
    let border_right: u16 = w2 - 1;
    let border_left: u16 = 0;
    let border_bottom: u16 = h2 - 1;

    let border_symbol_vertical = "|";
    let border_symbol_horizontal = "=";
    let border_symbol_edge = "+";

    let border_colour = termion::color::Rgb(192, 67, 67);

    print!(
        "{bg_reset}{bordercolour}",
        bg_reset = termion::color::Bg(termion::color::Reset),
        bordercolour = termion::color::Fg(border_colour)
    );

    while i < circumference {
        let x: u16 = i % w2;
        let y: u16 = i / w2;

        let is_on_edge: bool =
            (x == border_left || x == border_right) && (y == border_top || y == border_bottom);
        let is_vertical: bool = x == 0 || x == border_right; // most left or most right.

        /* Draw symbol. */
        print!(
            "{pos}{border}",
            pos = termion::cursor::Goto(i % w2 + 1, i / w2 + 1),
            border = match x {
                _ if is_on_edge => border_symbol_edge,
                _ if is_vertical => border_symbol_vertical,
                _ => border_symbol_horizontal,
            }
        );
        /* Go circumference field, next index. */
        let is_drawing_horizotal: bool = i < w2 || i >= circumference - w2 - 1 || i % w2 == w2 - 1;
        i += match is_drawing_horizotal {
            true => 1,       // add one.
            false => w2 - 1, // add width + 2
        };
    }

    /* Position to below field, clear everything below. */
    print!(
        "{pos_clear}{colour_reset}{clear}{pos_then}",
        pos_clear = termion::cursor::Goto(1, h as u16 + 3),
        colour_reset = termion::color::Fg(termion::color::Reset),
        pos_then = termion::cursor::Goto(1, h as u16 + 4),
        clear = termion::clear::AfterCursor
    );
}

/** Test doing tasks. */
#[cfg(test)]
mod test {

    use super::life;

    #[test]
    fn test_consume_bread() {
        // TODO refactor test.

        log::debug!("[TEST] Creating new stuff, let the wusels eat the bread.");
        let mut test_world: life::World = life::World::new(20, 5); // small world.
        log::debug!("Test World created");

        /* Empty test_world tick. */
        test_world.tick();
        log::debug!("Test World ticked");

        test_world.wusel_new("Eater".to_string(), true, life::Position::new(1, 0)); // female
        test_world.wusel_new("Starver".to_string(), false, life::Position::new(2, 0)); // male
        log::debug!("Test World's wusels created.");

        /* Create food: transportable, no storage. */
        let food1 = test_world.food_new("Bread", 100);

        let (food1_id, food1_index) = food1;

        log::debug!("Test World's food created, index: {}.", food1_index);

        let food2 = test_world.object_duplicate(0).unwrap(); // unsafe, but must be true.

        let (food2_id, food2_index) = food2;
        test_world.object_set_position(food2_id, test_world.position_random());

        log::debug!("Test World's food duplicated, index: {}.", food2_index);

        /* Put a copy into the world. */
        test_world.object_set_position(food1_id, test_world.position_random());

        log::debug!("Test World's food put onto a position.");

        /* Get the food and transport it somewhere else. */
        test_world.wusel_assign_task(1, life::TaskBuilder::use_object(food1_id, 1)); // take
        test_world.wusel_assign_task(1, life::TaskBuilder::move_to(test_world.position_random()));
        test_world.wusel_assign_task(1, life::TaskBuilder::use_object(food1_id, 2)); // drop
        test_world.wusel_assign_task(1, life::TaskBuilder::move_to(test_world.position_random()));
        test_world.wusel_assign_task(1, life::TaskBuilder::use_object(food1_id, 1)); // take not exisiting?

        /* Let the other wusel wait, than it's tries to get the food as well, and consume it. */
        test_world.wusel_assign_task(
            0,
            life::TaskBuilder::move_to(life::Position::new(
                test_world.get_width() - 1,
                test_world.get_depth() - 1,
            )),
        );
        test_world.wusel_assign_task(0, life::TaskBuilder::use_object(food1_id, 1)); // take as well.
        test_world.wusel_assign_task(0, life::TaskBuilder::move_to(test_world.position_random()));
        test_world.wusel_assign_task(0, life::TaskBuilder::use_object(food1_id, 3)); // consume.
        test_world.wusel_assign_task(0, life::TaskBuilder::move_to(test_world.position_random()));
        test_world.wusel_assign_task(0, life::TaskBuilder::move_to(test_world.position_random()));
        log::debug!("Test World's task to work at the workbench assigned.");

        // show everyone's stats.
        for i in 0usize..2 {
            test_world.wusel_show_tasklist(i); // tasks
            for n in life::Need::VALUES.iter() {
                test_world.wusel_set_need(i, n, 100);
            }
            test_world.wusel_show_overview(i); // needs
        }
        log::debug!("Test World's wusels' needs artificially reduced.");

        /* Show the grid.. */
        let (_w, _h): (usize, usize) = (
            test_world.get_width() as usize,
            test_world.get_depth() as usize,
        );

        println!(
            "{clear}{hide}",
            clear = termion::clear::All,
            hide = termion::cursor::Hide
        ); // clear the screen

        for _ in 0..300 {
            // render_field(_w, _h, test_world.positions_for_grid());
            println!();
            log::debug!(
                "Test World's current grid, time: {}.",
                test_world.get_time()
            );

            test_world.tick(); // progress time.

            // show everyone's stats.
            for i in 0usize..2 {
                test_world.wusel_show_overview(i); // needs
                test_world.wusel_show_tasklist(i); // tasks
            }

            if test_world.wusel_get_all_unbusy().len() > 1 {
                log::debug!("Test world is done, to be busy.");
                break;
            }

            std::thread::sleep(std::time::Duration::from_millis(100)); // wait.
        }
    }

    /** Test doing tasks. */
    #[test]
    fn test_create_bread() {
        // TODO refactor test.

        // Example: Wusel wants to cook.
        // 1. Go to (free) cooking station: (move)
        // 2. Wait for the Station to be free
        // 3. Work on station.
        // 4. Fetch tomatoes to be cut and prepared (needs Tomatoes)
        // 5. Cut (consume) tomatoes, create sauce
        // 6. Heat up sauce. (> use up cold <? Consumable with extra states?)
        // 7. Creates hot tomato sauce. (can get cold or be eaten.)
        //
        // OPTIONAL
        // Or should tools also be "Consumed" after 1M uses?
        // Knife dull and then .. gone
        // Couch is sit broken after several times?

        /* Cook a meal, that needs a working station, tomatoes, a knife and pot.
         * Or knife and Pot as part of the station.
         * Cut a meal, boil the meal => consumes tomatoes, creates tomato soup. */

        // abort if difficulty is too high
        // walk to station.position, wait until free, block
        // get required ingredients
        // do required steps, eg. station changing, prbly a list of subtasks?

        // using objects may influence the needs and skills.
        // eg.
        // * eating uses energy, but fills water and hunger
        // * sleeping fills energy
        // * doing sports uses energy and water and fills sportivité abilities.
    }

    /** Test mutually meeting, which may cause deadlocks.
     * -----
     * 1at: [Read, Meet 2nd].
     * 2nd: [Meet 3rd]
     * 3rd: [Meet 4th]
     * 4th: [Meet 1st]
     * -----
     * 1at: [Read, Meet 2nd] + [Met by 4th]
     * 2nd: [Meet 3rd]
     * 3rd: [Meet 4th] + [Met by 2nd]
     * 4th: [Meet 1st] + [Met by 3rd]
     * -----
     * 1st done with reading and wants to meet 2nd.
     * -----
     * 1at: [Meet 2nd, Met by 4th]
     * 2nd: [Meet 3rd] + [Met by 1st]
     * 3rd: [Meet 4th, Met by 2nd]
     * 4th: [Meet 1st, Met by 3rd]
     * -----
     * Nothing happens, since everyone waits for the other to be done.
     * 2nd, 3rd and 4th stop meeting. (they waited too long)
     * -----
     * 1at: [Meet 2nd, Met by 4th]
     * 2nd: [Met by 1st]
     * 3rd: [Met by 2nd]
     * 4th: [Met by 3rd]
     * -----
     * The active meeter, they were about to be met by is gone, stop being met.
     * 1at: [Meet 2nd, Met by 4th]
     * 2nd: [Met by 1st]
     * 3rd: []
     * 4th: []
     * -----
     * 1st meets 2nd; 4th is not meeting 1st anymore. No tasks left.
     */
    #[test]
    fn test_mutal_meeting() {
        // TODO refactor test.

        println!("[test] Mutual Meeting, causes for circular deadlocks.");
        let mut test_world: life::World = life::World::new(80, 30);

        /* Empty test_world tick. */
        test_world.tick();

        test_world.wusel_new("1st".to_string(), true, life::Position { x: 1, y: 0 }); // female
        test_world.wusel_new("2nd".to_string(), true, life::Position { x: 3, y: 0 }); // female
        test_world.wusel_new("3rd".to_string(), false, life::Position { x: 5, y: 0 }); // male
        test_world.wusel_new("4th".to_string(), false, life::Position { x: 9, y: 0 }); // male

        // 4 wusels created.
        assert_eq!(4, test_world.wusel_count());

        /* Create an easy talk, without any preconditions.
         * => no preconditions.
         * => does 'nothing' for ticks steps. */
        let reading: life::TaskBuilder =
            life::TaskBuilder::new(String::from("Reading")).set_duration(5 /*ticks*/);

        test_world.tick();

        // first wusel is also doing something else
        test_world.wusel_assign_task(0, reading.clone()); // do reading.

        // scenario: everyone wants too meet the next one.
        test_world.wusel_assign_task(
            0,
            life::TaskBuilder::meet_with(1, true, false).set_duration(7),
        ); // mutual meeting.
        test_world.wusel_assign_task(
            1,
            life::TaskBuilder::meet_with(2, true, false).set_duration(7),
        ); // mutual meeting.
        test_world.wusel_assign_task(
            2,
            life::TaskBuilder::meet_with(3, true, false).set_duration(7),
        ); // mutual meeting.
        test_world.wusel_assign_task(
            3,
            life::TaskBuilder::meet_with(0, true, false).set_duration(7),
        ); // mutual meeting.

        /* 90 ticks later. */
        for _ in 0..90 {
            test_world.tick();
            // println!("\nTasks at time {}:", test_world.get_time());
            // for w in 0..4 { test_world.wusel_show_tasklist(w); }
        }
    }
}
///////////////////////////////////////////////////////////////////////////////
