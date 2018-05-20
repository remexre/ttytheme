use std::iter::once;

use color::Color;

/// A theme.
#[derive(Clone, Debug, Deserialize)]
pub struct Theme {
    pub normal: Pallete,
    pub bright: Pallete,
}

impl Theme {
    /// Iterates over the colors in the theme.
    pub fn iter(&self) -> impl Iterator<Item = Color> {
        self.normal.iter().chain(self.bright.iter())
    }
}

/// One half of a theme.
#[derive(Clone, Debug, Deserialize)]
pub struct Pallete {
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub white: Color,
}

impl Pallete {
    /// Iterates over the colors in the pallete.
    pub fn iter(&self) -> impl Iterator<Item = Color> {
        // TODO: Check how slow this is...
        once(self.black)
            .chain(once(self.red))
            .chain(once(self.green))
            .chain(once(self.yellow))
            .chain(once(self.blue))
            .chain(once(self.magenta))
            .chain(once(self.cyan))
            .chain(once(self.white))
    }
}
