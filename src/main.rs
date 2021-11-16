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

    println!(
        "{clear}{hide}",
        clear = termion::clear::All,
        hide = termion::cursor::Hide
    ); //clear on start.

    // use terminal_size::{Width, Height, terminal_size};

    let width: u32;
    let height: u32;
    let screen_height: u16;

    let size = terminal_size::terminal_size();
    if let Some((terminal_size::Width(w), terminal_size::Height(h))) = size {
        width = w as u32 - (2 * 3);
        height = (h as u32) - (2 * 3) - 6; // minus gap for time.
        screen_height = h;
    } else {
        width = 80;
        height = 30;
        screen_height = 0;
    }

    if (screen_height < 2) {
        assert!(screen_height > 1);
    }

    let mut world: life::World = life::World::new(width, height);
    println!(
        "Created a new world: w:{w}, h:{h}",
        w = world.get_width(),
        h = world.get_depth()
    );

    /* Empty world tick. */
    world.tick();

    world.wusel_new(
        "1st".to_string(),
        life::WuselGender::Female,
        life::Position::new(0, 0),
    ); // female
    world.wusel_new(
        "2nd".to_string(),
        life::WuselGender::Female,
        life::Position::new(20, 0),
    ); // female
    world.wusel_new(
        "3rd".to_string(),
        life::WuselGender::Male,
        life::Position::new(30, 0),
    ); // male
    world.wusel_new(
        "4th".to_string(),
        life::WuselGender::Male,
        life::Position::new(40, 0),
    ); // male

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

    /* Draw the field and make some real automation. */
    let (w, h) = (world.get_width() as usize, world.get_depth() as usize);

    let time_position: (u16, u16) = (1u16, h as u16 + 3);
    let need_panel_position: (u16, u16) = (1u16, h as u16 + 5);
    let need_panel_width: u16 = 10;
    let need_panel_show_percentage: bool = true;

    let iterations: u32 = 100;

    // render field frame.
    render_rectangle(
        (1, 1),
        (w as u16 + 2, h as u16 + 2),
        &format!("{}-", termion::color::Fg(termion::color::Rgb(0, 0, 255))),
        &format!("{}|", termion::color::Fg(termion::color::Rgb(0, 255, 0))),
        &format!("{}+", termion::color::Fg(termion::color::Rgb(255, 0, 0))),
    );

    for _ in 0..iterations {
        // world.positions_recalculate_grid();
        render_field(w, h, world.positions_for_grid());

        /* Tick the world, maybe print the ongoing tasks. */
        render_time(time_position, world.get_time());

        /* Draw selected wusel's needs (right position below field). */

        for wusel_id in 0u16..4u16 {
            // TODO
            render_wusel_tasklist(
                (
                    need_panel_position.0 + wusel_id * 30,
                    need_panel_position.1 - 1,
                ),
                world.wusel_get_tasklist(wusel_id as usize),
            );

            render_wusel_need_bar(
                (need_panel_position.0 + wusel_id * 30, need_panel_position.1),
                need_panel_width,
                need_panel_show_percentage,
                &mut world,
                wusel_id as usize,
            );
        }

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

        // print!("{}", termion::cursor::Goto(1u16, height as u16 - 1));
        std::thread::sleep(duration); // wait.

        print!("{}", termion::cursor::Goto(1, screen_height))
    }

    render_reset((1u16, 1u16)); // clear whole field.
    Ok(())
}

fn render_reset(end_position: (u16, u16)) {
    /* Position to below field, clear everything below. */
    print!(
        "{pos_clear}{colour_reset}{clear}{pos_then}",
        pos_clear = termion::cursor::Goto(end_position.0, end_position.1),
        colour_reset = termion::color::Fg(termion::color::Reset),
        pos_then = termion::cursor::Goto(end_position.0, end_position.1 + 1), // continue here.
        clear = termion::clear::AfterCursor
    );
}

fn render_reset_colours() {
    print!(
        "{}{}",
        termion::color::Bg(termion::color::Reset),
        termion::color::Fg(termion::color::Reset)
    );
}

/** Draw a rectangle between the spanning postions, do not fill/overwrite the area. */
fn render_rectangle(
    a: (u16, u16),
    b: (u16, u16),
    horizontal_border_symbol: &String,
    vertical_border_symbol: &String,
    corner_symbol: &String,
) {
    let (x0, y0): (u16, u16) = (u16::min(a.0, b.0), u16::min(a.1, b.1)); // top left
    let (x1, y1): (u16, u16) = (u16::max(a.0, b.0), u16::max(a.1, b.1)); // bottom right

    /* Draw horizontal lines. */
    for x in (x0 + 1)..(x1) {
        print!(
            "{}{}",
            termion::cursor::Goto(x, y0),
            horizontal_border_symbol
        );
        print!(
            "{}{}",
            termion::cursor::Goto(x, y1),
            horizontal_border_symbol
        );
    }

    /* Draw vertical lines. */
    for y in (y0 + 1)..(y1) {
        print!("{}{}", termion::cursor::Goto(x0, y), vertical_border_symbol);
        print!("{}{}", termion::cursor::Goto(x1, y), vertical_border_symbol);
    }

    /* Draw Corners. */
    for x in [x0, x1] {
        for y in [y0, y1] {
            print!("{}{}", termion::cursor::Goto(x, y), corner_symbol);
        }
    }

    render_reset_colours();
}

/** Draw a rectangle between the spanning positions, filling the area. */
fn render_rectangle_filled(a: (u16, u16), b: (u16, u16), fill: &String) {
    let (x0, y0): (u16, u16) = (u16::min(a.0, b.0), u16::min(a.1, b.1)); // top left
    let (x1, y1): (u16, u16) = (u16::max(a.0, b.0), u16::max(a.1, b.1)); // bottom right

    for x in x0..x1 {
        for y in y0..y1 {
            print!("{}{}", termion::cursor::Goto(x, y), fill);
        }
    }

    render_reset_colours();
}

/** Draw a line with a certain symbol, this grows either to the right or down. */
fn render_rectangle_by_offset(pos: (u16, u16), offset: (u16, u16), fill: &String) {
    render_rectangle_filled(pos, (pos.0 + offset.0, pos.1 + offset.1), fill);
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

    render_reset_colours();
}

fn render_time(position: (u16, u16), time: usize) {
    print!(
        "{goto}{formatted_time}",
        goto = termion::cursor::Goto(position.0, position.1),
        formatted_time = format!("Time: {}", time)
    );
}

fn render_wusel_tasklist(position: (u16, u16), tasklist: Vec<String>) {
    print!(
        "{goto}",
        goto = termion::cursor::Goto(position.0, position.1)
    );
    for task in tasklist {
        print!("[{task}] ", task = task);
    }
}

fn render_wusel_need_bar(
    position: (u16, u16),
    panel_width: u16,
    show_percentage: bool,
    world: &mut life::World,
    wusel_index: usize,
) {
    let mut offset: u16 = 0;
    let (x, y) = position;
    for need in life::Need::VALUES {
        print!(
            "{goto}{title:9}",
            goto = termion::cursor::Goto(x, y + offset),
            title = need.name()
        );

        render_default_bar(
            (x + 10, y + offset),
            panel_width,
            show_percentage,
            world.wusel_get_need_full(need),
            world.wusel_get_need(wusel_index, need),
            Some(format!(
                "{}o",
                termion::color::Bg(termion::color::Rgb(0, 255, 0))
            )),
            Some(format!(
                "{}-",
                termion::color::Bg(termion::color::Rgb(255, 0, 0))
            )),
        );
        offset += 1;
    }
}

/** Render a bar at a given position. If min value is not null, try to balance the zero value in the center. */
fn render_default_bar(
    position: (u16, u16),
    panel_width: u16,
    show_percentage: bool,
    max_value: u32,
    current_value: u32,
    optional_render_filled: Option<String>,
    optional_render_rest: Option<String>,
) {
    // max width of that actual bar.
    let bar_width: u16 = panel_width - 2;

    let percentage: f32 = current_value as f32 / max_value as f32;
    let percentual: u16 = (percentage * bar_width as f32) as u16;

    let filled_bar: u16 = u16::min(percentual, bar_width);

    let render_filled: String = optional_render_filled.unwrap_or(format!("o"));
    let render_rest: String = optional_render_rest.unwrap_or(format!("."));

    render_rectangle_by_offset(position, (bar_width as u16, 1), &render_rest);
    render_rectangle_by_offset(position, (filled_bar as u16, 1), &render_filled); // overwrite.

    print!(
        "{bar_start}[{bar_end}]",
        bar_start = termion::cursor::Goto(position.0, position.1),
        bar_end = termion::cursor::Goto(position.0 + panel_width as u16 - 2, position.1)
    );

    if show_percentage {
        print!(
            "{goto}{bar_bit}{percentage:3}% ",
            goto = termion::cursor::Goto(position.0 + 1, position.1),
            bar_bit = if filled_bar > 0 {
                render_filled
            } else {
                render_rest
            },
            percentage = (percentage * 100.0) as u16
        );
    };

    render_reset_colours();
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

        test_world.wusel_new(
            "Eater".to_string(),
            life::WuselGender::Female,
            life::Position::new(1, 0),
        ); // female
        test_world.wusel_new(
            "Starver".to_string(),
            life::WuselGender::Male,
            life::Position::new(2, 0),
        ); // male
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
            // test_world.wusel_show_tasklist(i); // tasks
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
        ); // clear the test screen

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
                                                   // test_world.wusel_show_tasklist(i); // tasks
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

        test_world.wusel_new(
            "1st".to_string(),
            life::WuselGender::Female,
            life::Position { x: 1, y: 0 },
        ); // female
        test_world.wusel_new(
            "2nd".to_string(),
            life::WuselGender::Female,
            life::Position { x: 3, y: 0 },
        ); // female
        test_world.wusel_new(
            "3rd".to_string(),
            life::WuselGender::Male,
            life::Position { x: 5, y: 0 },
        ); // male
        test_world.wusel_new(
            "4th".to_string(),
            life::WuselGender::Male,
            life::Position { x: 9, y: 0 },
        ); // male

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
