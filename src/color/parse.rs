use std::str::FromStr;

use color::Color;

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Color, ()> {
        match parse_color(s) {
            Some(color) => Ok(color),
            None => Err(()),
        }
    }
}

fn parse_color(s: &str) -> Option<Color> {
    digits(if s.chars().next() == Some('#') {
        &s[1..]
    } else {
        s
    })
}

fn digits(s: &str) -> Option<Color> {
    if s.len() > 6 {
        None
    } else {
        let mut n = 0u32;
        for c in s.chars() {
            let c = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'a' | 'A' => 10,
                'b' | 'B' => 11,
                'c' | 'C' => 12,
                'd' | 'D' => 13,
                'e' | 'E' => 14,
                'f' | 'F' => 15,
                _ => return None,
            };
            n = (n << 4) | c;
        }
        Some(n.into())
    }
}
