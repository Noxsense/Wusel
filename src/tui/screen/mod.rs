//! # TUI Core
//!
//! Here, functions to render on the terminal user interface are provided.
//!
//! ## Author
//! Ngoc (Nox) Le <noxsense@gmail.com>

use termion;
pub use termion::color::Rgb;


pub mod render;


/// Position on the Screen.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Pos {
    pub x: u16,
    pub y: u16,
}

impl Pos {
    pub const START: Self = Pos { x: 1, y: 1 };
}


impl std::ops::Add<Pos> for Pos {
    type Output = Self;
    fn add(self, _rhs: Self) -> Self {
        Self {
            x: self.x.saturating_add(_rhs.x),
            y: self.y.saturating_add(_rhs.y),
        }
    }
}

impl std::ops::Add<(u16, u16)> for Pos {
    type Output = Self;
    fn add(self, _rhs: (u16, u16)) -> Self {
        Self {
            x: self.x.saturating_add(_rhs.0),
            y: self.y.saturating_add(_rhs.1),
        }
    }
}

impl std::ops::Sub<Pos> for Pos {
    type Output = Self;
    fn sub(self, _rhs: Self) -> Self {
        Self {
            x: self.x.saturating_sub(_rhs.x),
            y: self.y.saturating_sub(_rhs.y),
        }
    }
}

impl std::ops::Sub<(u16, u16)> for Pos {
    type Output = Self;
    fn sub(self, _rhs: (u16, u16)) -> Self {
        Self {
            x: self.x.saturating_sub(_rhs.0),
            y: self.y.saturating_sub(_rhs.1),
        }
    }
}

impl std::convert::From<(u16, u16)> for Pos {
    /// Create a new Screen Position from a tuple.
    fn from(position: (u16, u16)) -> Self {
        Self {
            x: position.0,
            y: position.1,
        }
    }
}

impl std::convert::Into<(u16, u16)> for Pos {
    /// Create a new Screen Position from a tuple.
    fn into(self) -> (u16, u16) {
        (self.x, self.y)
    }
}

/// Wrapping TextStyle simulating termion Screen Styles.
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

/// Transform a colour given as `#0066ff` to as Rgb(0, 102, 255) type.
pub fn hash_color_to_rgb(color_hash: u32) -> Rgb {
    let r: u8 = (color_hash >> 4) as u8;
    let g: u8 = ((color_hash >> 2) % 256) as u8;
    let b: u8 = (color_hash % 256) as u8;
    Rgb(r, g, b)
}

/// Darken a colour value.
/// Decreasing all fields value with the same amount.
pub fn darken_rgb(colour: Rgb, darker_value: u8) -> Rgb {
    let Rgb(r, g, b) = colour;

    let r1 = r.saturating_sub(darker_value);
    let g1: u8 = g.saturating_sub(darker_value);
    let b1: u8 = b.saturating_sub(darker_value);
    Rgb(r1, g1, b1)
}
