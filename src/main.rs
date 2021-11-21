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
// use rand;
// use std;
// use termion;

pub mod life;
pub mod tui;
pub mod util;

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
    tui::general::render_clear_all();

    let (screen_width, screen_height) = match termion::terminal_size() {
        Ok((w, h)) => (w, h),
        Err(e) => return Err(e),
    };

    let width: u32 = screen_width as u32 - (2 * 3);
    let height: u32 = (screen_height as u32) - (2 * 3) - 8; // minus gap for time.

    let mut world: life::world::World = life::world::World::new(width, height);
    log::debug!(
        "Created a new world: w:{w}, h:{h}",
        w = world.get_width(),
        h = world.get_depth()
    );

    /* Empty world tick. */
    world.tick();

    for _ in 0..rand::random::<u8>() % 10 + 2 {
        world.wusel_new_random(util::more_strings::name_gen(rand::random::<usize>() % 13 + 2));
    }

    /* Transportable bibimbap (korean food) */
    let bibimbap = world.food_new("Bibimbap", 10);
    let (bibimbap_id, _bibimbap_index) = bibimbap;

    /* Position. */
    world.object_set_position(bibimbap_id, world.position_random());

    let steps_per_second = arg_steps_per_second;
    let step_sleep = std::time::Duration::from_millis(1000 / steps_per_second);

    /* Draw the field and make some real automation. */
    let (w, h) = (world.get_width() as usize, world.get_depth() as usize);

    let time_position: &tui::general::ScreenPos = &tui::general::ScreenPos {
        x: 1u16,
        y: h as u16 + 3,
    };
    let timebar_position: &tui::general::ScreenPos = &tui::general::ScreenPos {
        x: w as u16 + 4,
        y: 1,
    };
    let need_panel_position: &tui::general::ScreenPos = &tui::general::ScreenPos {
        x: 2u16,
        y: h as u16 + 6,
    };
    let need_bar_width: u16 = 10;
    let need_panel_show_percentage: bool = true;

    tui::general::render_clear_all();

    // frame game field
    tui::general::render_rectangle(
        &tui::general::ScreenPos { x: 1, y: 1 },
        &tui::general::ScreenPos {
            x: w as u16 + 2,
            y: h as u16 + 2,
        },
        &format!("{}-", termion::color::Fg(termion::color::Rgb(0, 0, 255))),
        &format!("{}|", termion::color::Fg(termion::color::Rgb(0, 255, 0))),
        &format!("{}+", termion::color::Fg(termion::color::Rgb(255, 0, 0))),
    );

    // frame need panel
    tui::general::render_rectangle(
        &tui::general::ScreenPos {
            x: need_panel_position.x - 1,
            y: need_panel_position.y - 1,
        },
        &tui::general::ScreenPos {
            x: need_panel_position.x + 9 + need_bar_width,
            y: need_panel_position.y + 7,
        },
        &format!("{}-", termion::color::Fg(termion::color::Rgb(255, 255, 0))),
        &format!("{}|", termion::color::Fg(termion::color::Rgb(255, 255, 0))),
        &format!("{}+", termion::color::Fg(termion::color::Rgb(255, 255, 0))),
    );

    for i in 0usize..iterations {
        // world.positions_recalculate_grid();
        render_field(w, world.positions_for_grid());

        /* Tick the world, show time. */
        render_time(time_position, i, world.get_time());
        tui::general::render_progres_bar(
            timebar_position,
            h as u16 + 3,
            false,
            iterations as u32,
            i as u32 + 1,
            None,
            false,
        );

        /* Draw selected wusel's needs (right position below field). */

        for (wusel_offset, wusel_id) in world.wusel_get_all_alive().iter().enumerate() {
            // TODO

            let x_offset = wusel_offset as u16 * 23;

            if need_panel_position.x + x_offset + 20 < screen_width {
                tui::general::cursor_to(&need_panel_position.right_by(x_offset).up_by(2));
                print!(
                    "{} ({})",
                    world
                        .wusel_get_name(*wusel_id as usize)
                        .unwrap_or_else(|| "No Name".to_string()),
                    world
                        .wusel_get_gender(*wusel_id as usize)
                        .unwrap_or(life::world::WuselGender::Female)
                        .to_char(),
                );
                // render_wusel_tasklist(
                //    need_panel_position.right_by(x_offset).up_by(2),
                //     world.wusel_get_tasklist(*wusel_id as usize),
                // );

                let needs: Vec<(life::world::Need, u32, u32)> = life::world::Need::VALUES
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
                    need_panel_position.right_by(x_offset),
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
                        life::world::TaskBuilder::meet_with(i, true, true).set_duration(10),
                    );
                }
                i if i >= wusel_len && i < 2 * wusel_len => {
                    /* Walk randomly somewhere, if i not an wusel index. */
                    world.wusel_assign_task(
                        widx,
                        life::world::TaskBuilder::move_to(world.position_random()),
                    );
                }
                i if i >= 2 * wusel_len && i < 3 * wusel_len => {
                    /* Interact with the object. */
                    world.wusel_assign_task(
                        widx,
                        life::world::TaskBuilder::use_object(bibimbap_id, 0), // view
                    );
                }
                _ => {} // do nothing randomly.
            }
        }

        std::thread::sleep(step_sleep); // wait.

        // cursor to bottom.
        tui::general::cursor_to(&tui::general::ScreenPos {
            x: 1,
            y: screen_height,
        });
    }

    if clear_on_exit {
        tui::general::render_reset(&tui::general::ScreenPos::START); // clear whole field.
    }

    tui::general::cursor_to(&tui::general::ScreenPos::START.down_by(screen_height));
    Ok(())
}

fn get_render_for_position(
    c: char,
) -> (
    char,
    Option<termion::color::Rgb>,
    Option<termion::color::Rgb>,
    Option<Vec<tui::general::TextStyle>>,
) {
    match c {
        '\u{263A}'  => ('\u{263A}', Some(termion::color::Rgb(0, 0, 0)), None, Some(vec![tui::general::TextStyle::Bold])), // wusel, -- smiley, alternatively or w
        '#'         => ('#', Some(termion::color::Rgb(000, 000, 000)), None, None), // construction, eg. wall
        'm'         => ('m', Some(termion::color::Rgb(99, 67, 14)), None, None), // furniture, eg. chair
        '*'         => ('*', Some(termion::color::Rgb(000, 000, 100)), None, None), // miscellaneous, eg. food
        'ó'         => ('ó', Some(termion::color::Rgb(200, 000, 000)), None, None), // food
        _           => (' ', Some(termion::color::Rgb(000, 100, 000)), Some(termion::color::Rgb(222, 255, 222)), None), // empty
    }
}

/** Clean he view and draw the field, put the cursor, two lines below the field, to write there. */
fn render_field(w: usize, positions: Vec<Vec<(char, usize)>>) {
    /* Draw field. */
    let reset_color_after_draw = false;
    let reset_style_after_draw = true;

    for (p, on_pos) in positions.iter().enumerate() {
        /* All things on this position. */

        let (x, y): (u16, u16);
        x = (p % w) as u16 + 2;
        y = (p / w) as u16 + 2;

        let on_pos_first = on_pos.get(0).unwrap_or(&('\0', 0usize)).0;

        let render_data = get_render_for_position(on_pos_first);

        let (render_char, render_fg, render_bg, render_styles) = render_data;

        /* Draw position symbol. */
        tui::general::render_spot(
            &tui::general::ScreenPos { x, y },
            render_char,
            render_fg,
            render_bg,
            render_styles,
            reset_style_after_draw,
            reset_color_after_draw,
        );
    }

    tui::general::render_reset_colours();
}

fn render_time(position: &tui::general::ScreenPos, tick: usize, time: usize) {
    tui::general::cursor_to(position);
    print!(
        "{formatted_time}",
        formatted_time = format!("Step Counter: {} => Time: {}", tick, time)
    );
}

fn render_wusel_tasklist(position: tui::general::ScreenPos, tasklist: Vec<String>) {
    tui::general::cursor_to(&position);
    print!("> ");
    for task in tasklist {
        print!("[{task}] ", task = task);
    }
}

/** Render a Need Panel. */
fn render_wusel_need_bar(
    position: tui::general::ScreenPos,
    panel_width: u16,
    show_percentage: bool,
    needs: Vec<(life::world::Need, u32, u32)>,
) {
    let draw_horizontal = true;

    let bar_start = position.right_by(10);

    for (offset, (need, need_full, need_now)) in needs.iter().enumerate() {
        let offset_u16 = offset as u16;
        tui::general::cursor_to(&position.down_by(offset_u16));
        print!("{title:9}", title = need.name());

        tui::general::render_progres_bar(
            &bar_start.down_by(offset_u16),
            panel_width,
            show_percentage,
            *need_full,
            *need_now,
            Some((
                termion::color::Rgb(0, 255, 0),
                termion::color::Rgb(255, 0, 0),
            )),
            draw_horizontal,
        );
    }
}
