//! # TUI Rendering
//!
//! Here we define how to put colour and lines on the screen.

/// Put the cursor on the given position,
/// see [Goto](termion::cursor_set::Goto).
fn cursor_set_u16(x: u16, y: u16) {
    print!("{}", termion::cursor::Goto(x, y));
}

/// Put the cursor on the given position given by a screen::Pos,
/// see [Goto](termion::cursor_set::Goto).
pub fn cursor_set(position: &super::Pos) {
    cursor_set_u16(position.x, position.y);
}

/// Clear everything after the cursor and also unset the colours from there on.
pub fn reset(end_position: &super::Pos) {
    // Position to below field, clear everything below.
    cursor_set(end_position);
    reset_colours();
    print!("{}", termion::clear::AfterCursor);
    cursor_set(end_position);
}

/// Make a blank screen.
pub fn clear_all() {
    print!("{}{}", termion::clear::All, termion::cursor::Hide);
}

/// Reset the style, keep going un-styled from then.
pub fn reset_style() {
    // also uncolours.
    print!("{}", termion::style::Reset);
    // TODO (2021-12-11) reset only style not color stack.
}

/// Reset color, keep going with default terminal colours.
pub fn reset_colours() {
    print!(
        "{}{}",
        termion::color::Bg(termion::color::Reset),
        termion::color::Fg(termion::color::Reset),
    );
}

/// Render on the given position a given char with styles, and colours.
pub fn spot(
    postion: &super::Pos,
    character: char,
    color_fg: Option<super::Rgb>,
    color_bg: Option<super::Rgb>,
    styles: Option<Vec<super::TextStyle>>,
    reset_style_set: bool,
    reset_color: bool,
) {
    cursor_set(postion);

    if let Some(color_bg_rgb) = color_bg {
        print!("{bg}", bg = termion::color::Bg(color_bg_rgb));
    }

    if let Some(color_fg_rgb) = color_fg {
        print!("{fg}", fg = termion::color::Fg(color_fg_rgb));
    }

    if let Some(text_styles) = styles {
        for style in text_styles {
            match style {
                super::TextStyle::Blink => print!("{}", termion::style::Blink),
                super::TextStyle::Bold => print!("{}", termion::style::Bold),
                super::TextStyle::CrossedOut => print!("{}", termion::style::CrossedOut),
                super::TextStyle::Framed => print!("{}", termion::style::Framed),
                super::TextStyle::Invert => print!("{}", termion::style::Invert),
                super::TextStyle::Italic => print!("{}", termion::style::Italic),
                super::TextStyle::Underline => print!("{}", termion::style::Underline),
            }
        }
    }

    print!("{}", character);

    if reset_style_set {
        reset_style();

        // FIXME workaround, restore colours, if not resetted.
        // if colour was none, but it used the background colour of a previous set, this will also not be reset.

        if !reset_color {
            if let Some(color_bg_rgb) = color_bg {
                print!("{bg}", bg = termion::color::Bg(color_bg_rgb));
            }

            if let Some(color_fg_rgb) = color_fg {
                print!("{fg}", fg = termion::color::Fg(color_fg_rgb));
            }
        }
    }

    if reset_color {
        reset_colours();
    }
}

/// Draw a rectangle between the spanning postions, do not fill/overwrite the area.
pub fn rectangle(
    a: &super::Pos,
    b: &super::Pos,
    horizontal_border_symbol: &str,
    vertical_border_symbol: &str,
    corner_symbol: &str,
) {
    let (x0, y0): (u16, u16) = (u16::min(a.x, b.x), u16::min(a.y, b.y)); // top left
    let (x1, y1): (u16, u16) = (u16::max(a.x, b.x), u16::max(a.y, b.y)); // bottom right

    // Draw horizontal lines.
    for x in (x0 + 1)..(x1) {
        cursor_set_u16(x, y0);
        print!("{}", horizontal_border_symbol);

        cursor_set_u16(x, y1);
        print!("{}", horizontal_border_symbol);
    }

    // Draw vertical lines.
    for y in (y0 + 1)..(y1) {
        cursor_set_u16(x0, y);
        print!("{}", vertical_border_symbol);

        cursor_set_u16(x1, y);
        print!("{}", vertical_border_symbol);
    }

    // Draw Corners.
    for x in [x0, x1] {
        for y in [y0, y1] {
            cursor_set_u16(x, y);
            print!("{}", corner_symbol);
        }
    }

    reset_colours();
}

/// Draw a rectangle between the spanning positions, filling the area.
pub fn rectangle_filled(a: &super::Pos, b: &super::Pos, fill: &str) {
    let (x0, y0): (u16, u16) = (u16::min(a.x, b.x), u16::min(a.y, b.y)); // top left
    let (x1, y1): (u16, u16) = (u16::max(a.x, b.x), u16::max(a.y, b.y)); // bottom right

    for x in x0..x1 {
        for y in y0..y1 {
            cursor_set_u16(x, y);
            print!("{}", fill);
        }
    }

    reset_colours();
}

/// Render a progress bar, with the value set by a percentage value.
pub fn progres_bar_from_percent(
    position: &super::Pos,
    panel_size: u16,
    show_percentage: bool,
    percentage: f32,
    optional_colours: Option<(super::Rgb, super::Rgb)>,
    draw_horizontal: bool,
) {
    // max width of that actual bar.
    let bar_max: u16 = panel_size - 3; // minus border.
    let bar_now: u16 = (percentage / 100f32 * bar_max as f32) as u16;

    let percentage_discrete: u8 = (percentage).round() as u8;
    let percentage_word: [char; 4] = [
        if percentage_discrete / 100 == 1 {
            '1'
        } else {
            ' '
        },
        (48u8 + (percentage_discrete / 10) % 10) as char,
        (48u8 + (percentage_discrete % 10)) as char,
        '%',
    ];

    let mut bar_character: char;

    if draw_horizontal {
        // Draw something like: [#######----]

        // start bar border.
        spot(position, '[', None, None, None, false, false);

        // draw bar content.
        if let Some((full_color, rest_color)) = optional_colours {
            // horizontal, colourful.
            let mut bar_colour: super::Rgb;

            for i in 1u16..bar_max + 1 {
                // change colour.
                if i <= bar_now {
                    bar_colour = full_color;
                    bar_character = '#';
                } else {
                    bar_colour = rest_color;
                    bar_character = '-';
                }
                if show_percentage && i > 1 && i < 6 {
                    // render percentage if not empty.
                    let word_index = i as usize - 2;
                    if percentage_word[word_index] != ' ' {
                        bar_colour = super::darken_rgb(bar_colour, 50u8);
                        bar_character = percentage_word[word_index];
                    }
                }
                print!("{}{}", termion::color::Fg(bar_colour), bar_character);
            }
            reset_colours();
        } else {
            // horizontal, plain.
            for i in 1u16..bar_max + 1 {
                // change colour.
                if i <= bar_now {
                    bar_character = '#';
                } else {
                    bar_character = '-';
                }
                if show_percentage && i > 1 && i < 6 {
                    // render percentage if not empty.
                    let word_index = i as usize - 2;
                    if percentage_word[word_index] != ' ' {
                        print!("{}", termion::color::Fg(super::Rgb(0, 0, 0)));
                        bar_character = percentage_word[word_index];
                    }
                }
                print!("{}", bar_character);
            }
        }
        // end bar border.
        spot(
            &(*position + (bar_max + 1, 0u16)),
            ']',
            None,
            None,
            None,
            false,
            false,
        );
    } else {
        // Draw vertical bar (down to up).
        spot(position, '^', None, None, None, false, false);

        if let Some((full_color, rest_color)) = optional_colours {
            // vertical, colourful
            let mut bar_colour: super::Rgb;

            for i in 1u16..bar_max + 1 {
                // change colour.
                if bar_max - i <= bar_now {
                    bar_colour = full_color;
                    bar_character = '#';
                } else {
                    bar_colour = rest_color;
                    bar_character = ':';
                }
                cursor_set(&(*position + (0u16, i)));
                print!("{}{}", termion::color::Fg(bar_colour), bar_character);
            }
            reset_colours();
        } else {
            // vertical, plain.
            for i in 1u16..bar_max + 1 {
                if bar_max - i <= bar_now {
                    bar_character = '#';
                } else {
                    bar_character = ':';
                }
                cursor_set(&(*position + (0u16, i)));
                print!("{}", bar_character);
            }
            reset_colours();
        }

        // end bar border.
        spot(
            &(*position + (0u16, bar_max + 1)),
            'v',
            None,
            None,
            None,
            false,
            false,
        );
    }

    reset_colours();
}

/// Render a bar at a given position.
/// If min value is not null, try to balance the zero value in the center.
pub fn progres_bar(
    position: &super::Pos,
    panel_size: u16,
    show_percentage: bool,
    max_value: u32,
    current_value: u32,
    optional_colours: Option<(super::Rgb, super::Rgb)>,
    draw_horizontal: bool,
) -> f32 {
    let percentage_pre: f32 = current_value as f32 / max_value as f32 * 100f32;
    let percentage: f32 = f32::min(100.0, f32::max(0.0, percentage_pre));

    progres_bar_from_percent(
        position,
        panel_size,
        show_percentage,
        percentage,
        optional_colours,
        draw_horizontal,
    );

    percentage
}
