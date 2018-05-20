mod de;
mod parse;
mod ser;

use std::fmt::{Display, Formatter, Result as FmtResult};

/// A color.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Color(pub u8, pub u8, pub u8);

impl Display for Color {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let mut n = self.2 as u32;
        n |= (self.1 as u32) << 8;
        n |= (self.0 as u32) << 16;
        write!(fmt, "#{:06x}", n)
    }
}

impl From<u32> for Color {
    fn from(n: u32) -> Color {
        let r = (n >> 16) as u8;
        let g = (n >> 8) as u8;
        let b = n as u8;
        Color(r, g, b)
    }
}
