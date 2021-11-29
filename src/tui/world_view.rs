use crate::life;
/**
 * module tui::world_view.
 * - Here, functions to tui::render the world view.
 * @author Nox
 * @version 2021.0.1
 */
use crate::tui::core;

fn get_render_for_position(
    c: Option<&life::world::PlaceTaker>,
) -> (
    char,
    Option<core::Rgb>,
    Option<core::Rgb>,
    Option<Vec<core::TextStyle>>,
) {
    match c {
       Some(life::world::PlaceTaker::Wusel(_))                                                 => ('O', Some(core::Rgb(0, 0, 0)), None, Some(vec![core::TextStyle::Bold])), // wusel, -- smiley, alternatively or w
       Some(life::world::PlaceTaker::Construction(_))                                          => ('#', Some(core::Rgb(000, 000, 000)), None, None), // construction, eg. wall
       Some(life::world::PlaceTaker::Object((life::objects::ObjectType::Furniture, _, _)))     => ('m', Some(core::Rgb(99, 67, 14)), None, None), // furniture, eg. chair
       Some(life::world::PlaceTaker::Object((life::objects::ObjectType::Miscellaneous, _, _))) => ('*', Some(core::Rgb(000, 000, 100)), None, None), // miscellaneous, eg. food
       Some(life::world::PlaceTaker::Object((life::objects::ObjectType::Food, _, _)))          => ('ó', Some(core::Rgb(200, 000, 000)), None, None), // food
        _                                                                                      => (' ', Some(core::Rgb(000, 100, 000)), Some(core::Rgb(222, 255, 222)), None), // empty
    }
}

/** Clean he view and draw the field, put the cursor, two lines below the field, to write there. */
pub fn render_field(w: usize, positions: Vec<Vec<life::world::PlaceTaker>>) {
    /* Draw field. */
    let reset_color_after_draw = false;
    let reset_style_after_draw = true;

    for (p, on_pos) in positions.iter().enumerate() {
        /* All things on this position. */

        let (x, y): (u16, u16);
        x = (p % w) as u16 + 2;
        y = (p / w) as u16 + 2;

        let render_data = get_render_for_position(on_pos.get(0));

        let (render_char, render_fg, render_bg, render_styles) = render_data;

        /* Draw position symbol. */
        core::render_spot(
            &core::ScreenPos { x, y },
            render_char,
            render_fg,
            render_bg,
            render_styles,
            reset_style_after_draw,
            reset_color_after_draw,
        );
    }

    core::render_reset_colours();
}

pub fn render_time(position: &core::ScreenPos, tick: usize, time: usize) {
    core::cursor_to(position);
    print!(
        "{formatted_time}",
        formatted_time = format!("Step Counter: {} => Time: {}", tick, time)
    );
}

pub fn render_wusel_tasklist(position: core::ScreenPos, tasklist: Vec<String>) {
    core::cursor_to(&position);
    print!("{:23}", ""); // clear field.
    core::cursor_to(&position);
    print!("|> ");

    let mut length = 3;

    for task in tasklist {
        length += task.chars().count() + 2;
        if length < 23 {
            print!("[{task}] ", task = task);
        }
    }
}

/** Render a Need Panel. */
pub fn render_wusel_need_bar(
    position: core::ScreenPos,
    panel_width: u16,
    show_percentage: bool,
    needs: Vec<(crate::life::wusel::Need, u32, u32)>,
) {
    let draw_horizontal = true;

    let bar_start = position.right_by(10);

    for (offset, (need, need_full, need_now)) in needs.iter().enumerate() {
        let offset_u16 = offset as u16;
        core::cursor_to(&position.down_by(offset_u16));
        print!("{title:9}", title = need.get_name());

        core::render_progres_bar(
            &bar_start.down_by(offset_u16),
            panel_width,
            show_percentage,
            *need_full,
            *need_now,
            Some((core::Rgb(0, 255, 0), core::Rgb(255, 0, 0))),
            draw_horizontal,
        );
    }
}
