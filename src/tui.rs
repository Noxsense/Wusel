pub fn render_reset(end_position: (u16, u16)) {
    /* Position to below field, clear everything below. */
    print!(
        "{pos_clear}{colour_reset}{clear}{pos_then}",
        pos_clear = termion::cursor::Goto(end_position.0, end_position.1),
        colour_reset = termion::color::Fg(termion::color::Reset),
        pos_then = termion::cursor::Goto(end_position.0, end_position.1 + 1), // continue here.
        clear = termion::clear::AfterCursor
    );
}

pub fn render_clear_all() {
    println!("{}{}", termion::clear::All, termion::cursor::Hide);
}

pub fn render_reset_colours() {
    print!(
        "{}{}",
        termion::color::Bg(termion::color::Reset),
        termion::color::Fg(termion::color::Reset)
    );
}

/** Draw a rectangle between the spanning postions, do not fill/overwrite the area. */
pub fn render_rectangle(
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
pub fn render_rectangle_filled(a: (u16, u16), b: (u16, u16), fill: &String) {
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
pub fn render_rectangle_by_offset(pos: (u16, u16), offset: (u16, u16), fill: &String) {
    render_rectangle_filled(pos, (pos.0 + offset.0, pos.1 + offset.1), fill);
}

/** Render a bar at a given position. If min value is not null, try to balance the zero value in the center. */
pub fn render_default_bar(
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
