extern crate rand;

use std;
use terminal_size;
use termion;

pub mod life;

use std::io;
// use std::io::{Read, Write, stdout, stdin};
// use termion::raw::IntoRawMode;

/** The main method of the wusel world. */
fn main() -> Result<(), io::Error> {
    env_logger::init(); // initiate the logger.

    let args: Vec<String> = std::env::args().collect();

    let iterations: u32 = match args.get(1) {
        Some(arg_str) => arg_str.parse().unwrap_or(10) as u32,
        None => 10u32,
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
    render_clear_all();

    // use terminal_size::{Width, Height, terminal_size};

    let width: u32;
    let height: u32;
    let (screen_width, screen_height): (u16, u16);

    let size = terminal_size::terminal_size();
    if let Some((terminal_size::Width(w), terminal_size::Height(h))) = size {
        width = w as u32 - (2 * 3);
        height = (h as u32) - (2 * 3) - 8; // minus gap for time.
        screen_width = w;
        screen_height = h;
    } else {
        width = 80;
        height = 30;
        screen_width = 0;
        screen_height = 0;
    }

    if screen_height < 2 || screen_width < 2 {
        assert!(screen_height > 1);
    }

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

    // TODO (2021-11-17) make mutable to change during the game.
    let steps_per_second = arg_steps_per_second;
    let step_sleep = std::time::Duration::from_millis(1000 / steps_per_second);

    /* Draw the field and make some real automation. */
    let (w, h) = (world.get_width() as usize, world.get_depth() as usize);

    let time_position: (u16, u16) = (1u16, h as u16 + 3);
    let need_panel_position: (u16, u16) = (2u16, h as u16 + 6);
    let need_bar_width: u16 = 10;
    let need_panel_show_percentage: bool = true;

    render_clear_all();

    // frame game field
    render_rectangle(
        (1, 1),
        (w as u16 + 2, h as u16 + 2),
        &format!("{}-", termion::color::Fg(termion::color::Rgb(0, 0, 255))),
        &format!("{}|", termion::color::Fg(termion::color::Rgb(0, 255, 0))),
        &format!("{}+", termion::color::Fg(termion::color::Rgb(255, 0, 0))),
    );

    // frame need panel
    render_rectangle(
        (need_panel_position.0 - 1, need_panel_position.1 - 1),
        (
            need_panel_position.0 + 9 + need_bar_width,
            need_panel_position.1 + 7,
        ),
        &format!("{}-", termion::color::Fg(termion::color::Rgb(255, 255, 0))),
        &format!("{}|", termion::color::Fg(termion::color::Rgb(255, 255, 0))),
        &format!("{}+", termion::color::Fg(termion::color::Rgb(255, 255, 0))),
    );

    for _ in 0..iterations {
        // world.positions_recalculate_grid();
        render_field(w, h, world.positions_for_grid());

        /* Tick the world, show time. */
        render_time(time_position, world.get_time());

        /* Draw selected wusel's needs (right position below field). */

        for wusel_id in 0u16..4u16 {
            // TODO

            let next_x = need_panel_position.0 + wusel_id * 30;

            if next_x + 30 < screen_width {
                render_wusel_tasklist(
                    (next_x, need_panel_position.1 - 2),
                    world.wusel_get_tasklist(wusel_id as usize),
                );

                render_wusel_need_bar(
                    (next_x, need_panel_position.1),
                    need_bar_width,
                    need_panel_show_percentage,
                    &mut world,
                    wusel_id as usize,
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
        print!("{}", termion::cursor::Goto(1, screen_height));
    }

    if clear_on_exit {
        render_reset((1u16, 1u16)); // clear whole field.
    }

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

fn render_clear_all() {
    println!("{}{}", termion::clear::All, termion::cursor::Hide);
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
        formatted_time = format!("Step Counter: {} => Time: {}", time, time)
    );
}

fn render_wusel_tasklist(position: (u16, u16), tasklist: Vec<String>) {
    print!(
        "{goto}> ",
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
