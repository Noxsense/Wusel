//! # Wusel - Game
//!
//! This is a life simulation game where life is given to multiple wusels whose life can be in your
//! hand, otherwise they will try really hard to keep them alive on their own and you can watch they
//! cute little waddling and 'wuseln'.
//!
//! ## Author
//! Ngoc (Nox) Le <noxsense@gmail.com>

pub mod life;
pub mod tui;
pub mod util;

/// The main method of the wusel world.
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

    let render: bool = match args.get(3) {
        Some(arg_str) => arg_str != "no-render",
        None => true,
    };

    let clear_on_exit: bool = match args.get(4) {
        Some(arg_str) => arg_str == "clear",
        None => false,
    };

    //clear on start.
    tui::core::render_clear_all();

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

    // Empty world tick.
    world.tick();

    for _ in 0..rand::random::<u8>() % 10 + 2 {
        world.wusel_new_random(util::more_strings::name_gen(
            rand::random::<usize>() % 13 + 2,
        ));
    }

    // Transportable bibimbap (korean food)
    let bibimbap = world.food_new("Bibimbap", 10);
    let bibimbap_id = bibimbap;

    // Position.
    world.object_set_position(bibimbap_id, world.position_random());

    let steps_per_second = arg_steps_per_second;
    let step_sleep = std::time::Duration::from_millis(1000 / steps_per_second);

    // Draw the field and make some real automation.
    let (w, h) = (world.get_width() as usize, world.get_depth() as usize);

    let time_position: &tui::core::ScreenPos = &tui::core::ScreenPos {
        x: 1u16,
        y: h as u16 + 3,
    };
    let timebar_position: &tui::core::ScreenPos = &tui::core::ScreenPos {
        x: w as u16 + 4,
        y: 1,
    };
    let need_panel_position: &tui::core::ScreenPos = &tui::core::ScreenPos {
        x: 2u16,
        y: h as u16 + 6,
    };
    let need_bar_width: u16 = 10;
    let need_panel_show_percentage: bool = true;

    tui::core::render_clear_all();

    // frame game field
    if render {
        tui::core::render_rectangle(
            &tui::core::ScreenPos { x: 1, y: 1 },
            &tui::core::ScreenPos {
                x: w as u16 + 2,
                y: h as u16 + 2,
            },
            &format!("{}-", termion::color::Fg(termion::color::Rgb(0, 0, 255))),
            &format!("{}|", termion::color::Fg(termion::color::Rgb(0, 255, 0))),
            &format!("{}+", termion::color::Fg(termion::color::Rgb(255, 0, 0))),
        );

        // frame need panel
        tui::core::render_rectangle(
            &tui::core::ScreenPos {
                x: need_panel_position.x - 1,
                y: need_panel_position.y - 1,
            },
            &tui::core::ScreenPos {
                x: need_panel_position.x + 9 + need_bar_width,
                y: need_panel_position.y + 7,
            },
            &format!("{}-", termion::color::Fg(termion::color::Rgb(255, 255, 0))),
            &format!("{}|", termion::color::Fg(termion::color::Rgb(255, 255, 0))),
            &format!("{}+", termion::color::Fg(termion::color::Rgb(255, 255, 0))),
        );
    }

    for i in 0usize..iterations {
        if render {
            // world.positions_recalculate_grid();
            tui::world_view::render_field(w, world.positions_for_all_placetakers());

            // Tick the world, show time.
            tui::world_view::render_time(time_position, i, world.get_time());
            tui::core::render_progres_bar(
                timebar_position,
                h as u16 + 3,
                false,
                iterations as u32,
                i as u32 + 1,
                None,
                false,
            );

            // Draw selected wusel's needs (right position below field).

            for (wusel_offset, &wusel_id) in world.wusel_get_all_alive().iter().enumerate() {
                // TODO

                let x_offset = wusel_offset as u16 * 23;

                if need_panel_position.x + x_offset + 20 < screen_width {
                    tui::core::cursor_to(&need_panel_position.right_by(x_offset).up_by(2));
                    print!(
                        "| {} ({})",
                        world
                            .wusel_get_name(wusel_id as usize)
                            .unwrap_or_else(|| "No Name".to_string()),
                        world
                            .wusel_get_gender(wusel_id as usize)
                            .unwrap_or(life::wusel::WuselGender::Female)
                            .to_char(),
                    );

                    tui::world_view::render_wusel_tasklist(
                        need_panel_position.right_by(x_offset).up_by(1),
                        world.wusel_get_tasklist_names(wusel_id as usize),
                    );

                    let needs: Vec<(life::wusel::Need, u32, u32)> = life::wusel::Need::VALUES
                        .iter()
                        .map(|need| {
                            (
                                *need,
                                need.get_full(),
                                world.wusel_get_need(wusel_id, *need),
                            )
                        })
                        .collect();

                    tui::world_view::render_wusel_need_bar(
                        need_panel_position.right_by(x_offset),
                        need_bar_width,
                        need_panel_show_percentage,
                        needs,
                    );
                }
            }
        }

        world.tick();

        // Give some unbusy wusels the task to move around.
        let unbusy = world.wusel_get_all_unbusy();
        let wusel_len = world.wusel_count();
        for widx in unbusy {
            let r = rand::random::<usize>() % (4 * wusel_len);
            match r {
                i if i < wusel_len && i != widx => {
                    // Meet randomly with someone: Let [widx] meet [i], if i in [0..|w|).
                    world.wusel_assign_to_task(
                        widx,
                        life::tasks::TaskBuilder::meet_with(i, true, true).set_duration(10),
                    );
                }
                i if i >= wusel_len && i < 2 * wusel_len => {
                    // Walk randomly somewhere, if i not an wusel index.
                    world.wusel_assign_to_task(
                        widx,
                        life::tasks::TaskBuilder::move_to(world.position_random()),
                    );
                }
                i if i >= 2 * wusel_len && i < 3 * wusel_len => {
                    // Interact with the object.
                    world.wusel_assign_to_task(
                        widx,
                        life::tasks::TaskBuilder::use_object(bibimbap_id, 0), // view
                    );
                }
                _ => {} // do nothing randomly.
            }
        }

        std::thread::sleep(step_sleep); // wait.

        // cursor to bottom.
        tui::core::cursor_to(&tui::core::ScreenPos {
            x: 1,
            y: screen_height,
        });
    }

    if clear_on_exit {
        tui::core::render_reset(&tui::core::ScreenPos::START); // clear whole field.
    }

    tui::core::cursor_to(&tui::core::ScreenPos::START.down_by(screen_height));
    Ok(())
}
