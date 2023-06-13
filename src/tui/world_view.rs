//! # TUI: World Rendering
//!
//! Here, functions to tui::render the world view.
//!
//! ## Author
//! Ngoc (Nox) Le <noxsense@gmail.com>

use crate::life;
use crate::tui::screen;

/// Get a styled char for a placeholder in the world.
fn get_render_for_position(
    c: Option<&life::world::PlaceTaker>,
) -> (
    char,
    Option<screen::Rgb>,
    Option<screen::Rgb>,
    Option<Vec<screen::TextStyle>>,
) {
    match c {
        Some(life::world::PlaceTaker::Wusel(_)) => (
            'O',
            Some(screen::Rgb(0, 0, 0)),
            None,
            Some(vec![screen::TextStyle::Bold]),
        ),

        Some(life::world::PlaceTaker::Construction(
            life::world::ConstructionType::Wall(false, _),
            _,
        )) => ('#', None, Some(screen::hash_color_to_rgb(0xa04c1f)), None),

        Some(life::world::PlaceTaker::Construction(
            life::world::ConstructionType::Wall(true, _),
            _,
        )) => ('#', None, Some(screen::hash_color_to_rgb(0xa04c1f)), None),

        Some(life::world::PlaceTaker::Construction(
            life::world::ConstructionType::Door(true),
            _,
        )) => ('-', None, None, None),

        Some(life::world::PlaceTaker::Construction(
            life::world::ConstructionType::Door(false),
            _,
        )) => ('+', None, None, None), // construction, eg. wall

        Some(life::world::PlaceTaker::Object(_, life::objects::ObjectType::Furniture(_))) => {
            ('m', Some(screen::Rgb(99, 67, 14)), None, None)
        }

        Some(life::world::PlaceTaker::Object(_, life::objects::ObjectType::Miscellaneous(_))) => {
            ('*', Some(screen::Rgb(000, 000, 100)), None, None)
        }

        Some(life::world::PlaceTaker::Object(_, life::objects::ObjectType::Food(_))) => {
            ('ó', Some(screen::Rgb(200, 000, 000)), None, None)
        }

        _ => (
            ' ',
            Some(screen::Rgb(000, 100, 000)),
            Some(screen::Rgb(222, 255, 222)),
            None,
        ),
    }
}

/// Clean he view and draw the field, put the cursor, two lines below the field, to write there.
pub fn render_field(w: usize, positions: Vec<Vec<life::world::PlaceTaker>>) {
    // Draw field.
    let reset_color_after_draw = false;
    let reset_style_after_draw = true;

    for (p, on_pos) in positions.iter().enumerate() {
        // All things on this position.

        let (x, y): (u16, u16);
        x = (p % w) as u16 + 2;
        y = (p / w) as u16 + 2;

        let most_important = on_pos
            .iter()
            .find(|&p| {
                matches!(
                    p,
                    &life::world::PlaceTaker::Construction(
                        life::world::ConstructionType::Door(_),
                        _
                    )
                )
            })
            .or_else(|| on_pos.get(0));

        let render_data = get_render_for_position(most_important);

        let (render_char, render_fg, render_bg, render_styles) = render_data;

        // Draw position symbol.
        screen::render::spot(
            &screen::Pos { x, y },
            render_char,
            render_fg,
            render_bg,
            render_styles,
            reset_style_after_draw,
            reset_color_after_draw,
        );
    }

    screen::render::reset_colours();
}

/// Show time
/// tick .. time units the session is running
/// time .. time of the world.
pub fn render_time(position: &screen::Pos, tick: usize, time: usize) {
    screen::render::cursor_set(position);
    print!(
        "{formatted_time}",
        formatted_time = format!("Step Counter: {} => Time: {}", tick, time)
    );
}

/// Render the task list of a wusel, given by string names.
/// This also crops longer texts.
pub fn render_wusel_tasklist(position: screen::Pos, tasklist: Vec<String>) {
    screen::render::cursor_set(&position);
    print!("{:23}", ""); // clear field.
    screen::render::cursor_set(&position);
    print!("|> ");

    let mut length = 3;

    for task in tasklist {
        length += task.chars().count() + 2;
        if length < 23 {
            print!("[{task}] ", task = task);
        }
    }
}

/// Render a Need Panel.
pub fn render_wusel_need_bar(
    position: screen::Pos,
    panel_width: u16,
    show_percentage: bool,
    needs: Vec<(crate::life::wusels::needs::Need, u32, u32)>,
) {
    let draw_horizontal = true;

    let bar_start = position + (10u16, 0u16);

    for (offset, (need, need_full, need_now)) in needs.iter().enumerate() {
        let offset_u16 = offset as u16;
        screen::render::cursor_set(&(position + (0u16, offset_u16)));
        print!("{title:9}", title = need.get_name());

        screen::render::progres_bar(
            &(bar_start + (0u16, offset_u16)),
            panel_width,
            show_percentage,
            *need_full,
            *need_now,
            Some((screen::Rgb(0, 255, 0), screen::Rgb(255, 0, 0))),
            draw_horizontal,
        );
    }
}
