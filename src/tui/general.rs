/**
 * module tui.
 * - Here, functions to render on the terminal user interface are provided.
 * @author Nox
 * @version 2021.0.1
 */
use termion;

pub struct ScreenPos {
    pub x: u16,
    pub y: u16,
}

impl ScreenPos {
    pub const START: Self = ScreenPos { x: 1, y: 1 };

    pub fn from(position: (u16, u16)) -> Self {
        Self {
            x: position.0,
            y: position.1,
        }
    }

    pub fn left_by(&self, offset: u16) -> Self {
        Self {
            x: self.x.saturating_sub(offset),
            y: self.y,
        }
    }

    pub fn left(&self) -> Self {
        self.left_by(1)
    }

    pub fn right_by(&self, offset: u16) -> Self {
        Self {
            x: self.x.saturating_add(offset),
            y: self.y,
        }
    }

    pub fn right(&self) -> Self {
        self.right_by(1)
    }

    pub fn up_by(&self, offset: u16) -> Self {
        Self {
            x: self.x,
            y: self.y.saturating_sub(offset),
        }
    }

    pub fn up(&self) -> Self {
        self.up_by(1)
    }

    pub fn down_by(&self, offset: u16) -> Self {
        Self {
            x: self.x,
            y: self.y.saturating_add(offset),
        }
    }

    pub fn down(&self) -> Self {
        self.down_by(1)
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum TextStyle {
    Blink,      // blinking text (not widely supported)
    CrossedOut, // (not widely supported)
    Framed,     // framed text (not widely supported)
    Bold,       // Bold text.
    Invert,     // Inverted colors (negative mode).
    Italic,     // Italic text.
    Underline,  // Underlined text.
}

pub fn hash_color_to_rgb(color_hash: u32) -> termion::color::Rgb {
    let r: u8 = (color_hash >> 4) as u8;
    let g: u8 = ((color_hash >> 2) % 256) as u8;
    let b: u8 = (color_hash % 256) as u8;
    termion::color::Rgb(r, g, b)
}

pub fn darken_rgb(colour: termion::color::Rgb, darker_value: u8) -> termion::color::Rgb {
    let termion::color::Rgb(r, g, b) = colour;

    let r1 = r.saturating_sub(darker_value);
    let g1: u8 = g.saturating_sub(darker_value);
    let b1: u8 = b.saturating_sub(darker_value);
    termion::color::Rgb(r1, g1, b1)
}

fn cursor_to_u16(x: u16, y: u16) {
    print!("{}", termion::cursor::Goto(x, y));
}

pub fn cursor_to(position: &ScreenPos) {
    cursor_to_u16(position.x, position.y);
}

pub fn render_reset(end_position: &ScreenPos) {
    /* Position to below field, clear everything below. */
    cursor_to(end_position);
    render_reset_colours();
    print!("{}", termion::clear::AfterCursor);
    cursor_to(end_position);
}

pub fn render_clear_all() {
    print!("{}{}", termion::clear::All, termion::cursor::Hide);
}

pub fn render_reset_style() {
    // also uncolours.
    print!("{}", termion::style::Reset);
}

pub fn render_reset_colours() {
    print!(
        "{}{}",
        termion::color::Bg(termion::color::Reset),
        termion::color::Fg(termion::color::Reset),
    );
}

pub fn render_spot(
    postion: &ScreenPos,
    character: char,
    color_fg: Option<termion::color::Rgb>,
    color_bg: Option<termion::color::Rgb>,
    styles: Option<Vec<TextStyle>>,
    reset_style: bool,
    reset_color: bool,
) {
    cursor_to(postion);

    if let Some(color_bg_rgb) = color_bg {
        print!("{bg}", bg = termion::color::Bg(color_bg_rgb));
    }

    if let Some(color_fg_rgb) = color_fg {
        print!("{fg}", fg = termion::color::Fg(color_fg_rgb));
    }

    if let Some(text_styles) = styles {
        for style in text_styles {
            match style {
                TextStyle::Blink => print!("{}", termion::style::Blink),
                TextStyle::Bold => print!("{}", termion::style::Bold),
                TextStyle::CrossedOut => print!("{}", termion::style::CrossedOut),
                TextStyle::Framed => print!("{}", termion::style::Framed),
                TextStyle::Invert => print!("{}", termion::style::Invert),
                TextStyle::Italic => print!("{}", termion::style::Italic),
                TextStyle::Underline => print!("{}", termion::style::Underline),
            }
        }
    }

    print!("{}", character);

    if reset_style {
        render_reset_style();

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
        render_reset_colours();
    }
}

/** Draw a rectangle between the spanning postions, do not fill/overwrite the area. */
pub fn render_rectangle(
    a: &ScreenPos,
    b: &ScreenPos,
    horizontal_border_symbol: &str,
    vertical_border_symbol: &str,
    corner_symbol: &str,
) {
    let (x0, y0): (u16, u16) = (u16::min(a.x, b.x), u16::min(a.y, b.y)); // top left
    let (x1, y1): (u16, u16) = (u16::max(a.x, b.x), u16::max(a.y, b.y)); // bottom right

    /* Draw horizontal lines. */
    for x in (x0 + 1)..(x1) {
        cursor_to_u16(x, y0);
        print!("{}", horizontal_border_symbol);

        cursor_to_u16(x, y1);
        print!("{}", horizontal_border_symbol);
    }

    /* Draw vertical lines. */
    for y in (y0 + 1)..(y1) {
        cursor_to_u16(x0, y);
        print!("{}", vertical_border_symbol);

        cursor_to_u16(x1, y);
        print!("{}", vertical_border_symbol);
    }

    /* Draw Corners. */
    for x in [x0, x1] {
        for y in [y0, y1] {
            cursor_to_u16(x, y);
            print!("{}", corner_symbol);
        }
    }

    render_reset_colours();
}

/** Draw a rectangle between the spanning positions, filling the area. */
pub fn render_rectangle_filled(a: &ScreenPos, b: &ScreenPos, fill: &str) {
    let (x0, y0): (u16, u16) = (u16::min(a.x, b.x), u16::min(a.y, b.y)); // top left
    let (x1, y1): (u16, u16) = (u16::max(a.x, b.x), u16::max(a.y, b.y)); // bottom right

    for x in x0..x1 {
        for y in y0..y1 {
            cursor_to_u16(x, y);
            print!("{}", fill);
        }
    }

    render_reset_colours();
}

pub fn render_progres_bar_from_percent(
    position: &ScreenPos,
    panel_size: u16,
    show_percentage: bool,
    percentage: f32,
    optipnal_colors: Option<(termion::color::Rgb, termion::color::Rgb)>,
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
        /* Draw something like: [#######----] */

        // start bar border.
        render_spot(position, '[', None, None, None, false, false);

        // draw bar content.
        if let Some((full_color, rest_color)) = optipnal_colors {
            // horizontal, colourful.
            let mut bar_colour: termion::color::Rgb;

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
                        bar_colour = darken_rgb(bar_colour, 50u8);
                        bar_character = percentage_word[word_index];
                    }
                }
                print!("{}{}", termion::color::Fg(bar_colour), bar_character);
            }
            render_reset_colours();
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
                        print!("{}", termion::color::Fg(termion::color::Rgb(0, 0, 0)));
                        bar_character = percentage_word[word_index];
                    }
                }
                print!("{}", bar_character);
            }
        }
        // end bar border.
        render_spot(&position.right_by(bar_max + 1), ']', None, None, None, false, false);
    } else {
        /* Draw vertical bar (down to up). */
        render_spot(position, '^', None, None, None, false, false);

        if let Some((full_color, rest_color)) = optipnal_colors {
            // vertical, colourful
            let mut bar_colour: termion::color::Rgb;

            for i in 1u16..bar_max + 1 {
                // change colour.
                if bar_max - i <= bar_now {
                    bar_colour = full_color;
                    bar_character = '#';
                } else {
                    bar_colour = rest_color;
                    bar_character = ':';
                }
                cursor_to(&position.down_by(i));
                print!("{}{}", termion::color::Fg(bar_colour), bar_character);
            }
            render_reset_colours();
        } else {
            // vertical, plain.
            for i in 1u16..bar_max + 1 {
                if bar_max - i <= bar_now {
                    bar_character = '#';
                } else {
                    bar_character = ':';
                }
                cursor_to(&position.down_by(i));
                print!("{}", bar_character);
            }
            render_reset_colours();
        }

        // end bar border.
        render_spot(&position.down_by(bar_max + 1), 'v', None, None, None, false, false);
    }

    render_reset_colours();
}

/** Render a bar at a given position.
 * If min value is not null, try to balance the zero value in the center. */
pub fn render_progres_bar(
    position: &ScreenPos,
    panel_size: u16,
    show_percentage: bool,
    max_value: u32,
    current_value: u32,
    optipnal_colors: Option<(termion::color::Rgb, termion::color::Rgb)>,
    draw_horizontal: bool,
) -> f32 {
    let percentage_pre: f32 = current_value as f32 / max_value as f32 * 100f32;
    let percentage: f32 = f32::min(100.0, f32::max(0.0, percentage_pre));

    render_progres_bar_from_percent(
        position,
        panel_size,
        show_percentage,
        percentage,
        optipnal_colors,
        draw_horizontal,
    );

    percentage
}
