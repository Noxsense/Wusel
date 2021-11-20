/**
 * main.
 *
 * This is a life simulation game where life is given to multiple wusels whose life can be in your
 * hand, otherwise they will try really hard to keep them alive on their own and you can watch they
 * cute little waddeling and 'wuseln'.
 *
 * @author Nox
 * @version 2021.0.1
 */
use rand;
use std;
use termion;

pub mod life;
pub mod tui;

/** The main method of the wusel world. */
fn main() -> Result<(), std::io::Error> {
    env_logger::init(); // initiate the logger.

    let args: Vec<String> = std::env::args().collect();

    let iterations: usize = match args.get(1) {
        Some(arg_str) => arg_str.parse().unwrap_or(10) as usize,
        None => 10usize,
    };

    let arg_steps_per_second: u64 = match args.get(2) {
        Some(arg_str) => arg_str.parse().unwrap_or(4),
        None => 8,
    };

    let clear_on_exit: bool = match args.get(3) {
        Some(arg_str) => arg_str == "clear",
        None => false,
    };

    //clear on start.
    tui::render_clear_all();

    let (screen_width, screen_height) = match termion::terminal_size() {
        Ok((w, h)) => (w, h),
        Err(e) => return Err(e),
    };

    let width: u32 = screen_width as u32 - (2 * 3);
    let height: u32 = (screen_height as u32) - (2 * 3) - 8; // minus gap for time.

    let mut world: life::World = life::World::new(width, height);
    log::debug!(
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
    );
    world.wusel_new(
        "2nd".to_string(),
        life::WuselGender::Female,
        life::Position::new(20, 0),
    );
    world.wusel_new(
        "3rd".to_string(),
        life::WuselGender::Male,
        life::Position::new(30, 0),
    );
    world.wusel_new(
        "4th".to_string(),
        life::WuselGender::Male,
        life::Position::new(40, 0),
    );

    /* Transportable bibimbap (korean food) */
    let bibimbap = world.food_new("Bibimbap", 10);
    let (bibimbap_id, _bibimbap_index) = bibimbap;

    /* Position. */
    world.object_set_position(bibimbap_id, world.position_random());

    let steps_per_second = arg_steps_per_second;
    let step_sleep = std::time::Duration::from_millis(1000 / steps_per_second);

    /* Draw the field and make some real automation. */
    let (w, h) = (world.get_width() as usize, world.get_depth() as usize);

    let time_position: (u16, u16) = (1u16, h as u16 + 3);
    let timebar_position: (u16, u16) = (w as u16 + 4, 1);
    let need_panel_position: (u16, u16) = (2u16, h as u16 + 6);
    let need_bar_width: u16 = 10;
    let need_panel_show_percentage: bool = true;

    tui::render_clear_all();

    // frame game field
    tui::render_rectangle(
        (1, 1),
        (w as u16 + 2, h as u16 + 2),
        &format!("{}-", termion::color::Fg(termion::color::Rgb(0, 0, 255))),
        &format!("{}|", termion::color::Fg(termion::color::Rgb(0, 255, 0))),
        &format!("{}+", termion::color::Fg(termion::color::Rgb(255, 0, 0))),
    );

    // frame need panel
    tui::render_rectangle(
        (need_panel_position.0 - 1, need_panel_position.1 - 1),
        (
            need_panel_position.0 + 9 + need_bar_width,
            need_panel_position.1 + 7,
        ),
        &format!("{}-", termion::color::Fg(termion::color::Rgb(255, 255, 0))),
        &format!("{}|", termion::color::Fg(termion::color::Rgb(255, 255, 0))),
        &format!("{}+", termion::color::Fg(termion::color::Rgb(255, 255, 0))),
    );

    for i in 0usize..iterations {
        // world.positions_recalculate_grid();
        render_field(w, world.positions_for_grid());

        /* Tick the world, show time. */
        render_time(time_position, i, world.get_time());
        tui::render_progres_bar(timebar_position, h as u16 + 3, false, iterations as u32, i as u32 + 1, None, false);

        /* Draw selected wusel's needs (right position below field). */

        let mut wusel_offset = 0u16;
        for wusel_id in world.wusel_get_all_alive().iter() {
            // TODO

            let next_x = need_panel_position.0 + wusel_offset as u16 * 30;
            wusel_offset += 1;

            if next_x + 30 < screen_width {
                render_wusel_tasklist(
                    (next_x, need_panel_position.1 - 2),
                    world.wusel_get_tasklist(*wusel_id as usize),
                );

                let needs: Vec<(life::Need, u32, u32)> = life::Need::VALUES
                    .iter()
                    .map(|need| {
                        (
                            *need,
                            world.wusel_get_need_full(*need),
                            world.wusel_get_need(*wusel_id, *need),
                        )
                    })
                    .collect();

                render_wusel_need_bar(
                    (next_x, need_panel_position.1),
                    need_bar_width,
                    need_panel_show_percentage,
                    needs,
                );
            }
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

        std::thread::sleep(step_sleep); // wait.

        // cursor to bottom.
        tui::cursor_to((1, screen_height));
    }

    if clear_on_exit {
        tui::render_reset((1u16, 1u16)); // clear whole field.
    }

    Ok(())
}

fn get_render_for_position(
    c: char,
) -> (
    char,
    Option<termion::color::Rgb>,
    Option<termion::color::Rgb>,
    Option<Vec<tui::TextStyle>>,
) {
    return match c {
        '\u{263A}'  => ('\u{263A}', Some(termion::color::Rgb(0, 0, 0)), None, Some(vec![tui::TextStyle::Bold])), // wusel, -- smiley, alternatively or w
        '#'         => ('#', Some(termion::color::Rgb(000, 000, 000)), None, None), // construction, eg. wall
        'm'         => ('m', Some(termion::color::Rgb(099, 067, 014)), None, None), // furniture, eg. chair
        '*'         => ('*', Some(termion::color::Rgb(000, 000, 100)), None, None), // miscellaneous, eg. food
        'ó'         => ('ó', Some(termion::color::Rgb(200, 000, 000)), None, None), // food
        _           => (' ', Some(termion::color::Rgb(000, 100, 000)), Some(termion::color::Rgb(222, 255, 222)), None), // empty
    };
}

/** Clean he view and draw the field, put the cursor, two lines below the field, to write there. */
fn render_field(w: usize, positions: Vec<Vec<(char, usize)>>) {
    /* Draw field. */
    let reset_color_after_draw = false;
    let reset_style_after_draw = true;

    for p in 0..positions.len() {
        /* All things on this position. */
        let on_pos = &positions[p];

        let (x, y): (u16, u16);
        x = (p % w) as u16 + 2;
        y = (p / w) as u16 + 2;

        let on_pos_first = on_pos.get(0).unwrap_or(&('\0', 0usize)).0;

        let render_data = get_render_for_position(on_pos_first);

        let (render_char, render_fg, render_bg, render_styles) = render_data;

        /* Draw position symbol. */
        tui::render_spot(
            (x, y),
            render_char,
            render_fg,
            render_bg,
            render_styles,
            reset_style_after_draw,
            reset_color_after_draw,
        );
    }

    tui::render_reset_colours();
}

fn render_time(position: (u16, u16), tick: usize, time: usize) {
    tui::cursor_to(position);
    print!(
        "{formatted_time}",
        formatted_time = format!("Step Counter: {} => Time: {}", time, time)
    );
}

fn render_wusel_tasklist(position: (u16, u16), tasklist: Vec<String>) {
    tui::cursor_to(position);
    print!("> ");
    for task in tasklist {
        print!("[{task}] ", task = task);
    }
}

/** Render a Need Panel. */
fn render_wusel_need_bar(
    position: (u16, u16),
    panel_width: u16,
    show_percentage: bool,
    needs: Vec<(life::Need, u32, u32)>,
) {
    let mut offset: u16 = 0;
    let (x, y) = position;
    let draw_horizontal = true;

    for (need, need_full, need_now) in needs {
        tui::cursor_to((x, y + offset));
        print!("{title:9}", title = need.name());

        tui::render_progres_bar(
            (x + 10, y + offset),
            panel_width,
            show_percentage,
            need_full,
            need_now,
            Some((
                termion::color::Rgb(0, 255, 0),
                termion::color::Rgb(255, 0, 0),
            )),
            draw_horizontal,
        );
        offset += 1;
    }
}
