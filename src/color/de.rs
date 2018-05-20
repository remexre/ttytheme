use std::fmt::{Formatter, Result as FmtResult};
use std::u32;

use serde::de::{Deserialize, Deserializer, Error, SeqAccess, Visitor};

use color::Color;

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Color, D::Error> {
        de.deserialize_any(ColorVisitor)
    }
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "a 24-bit color")
    }

    fn visit_i8<E: Error>(self, value: i8) -> Result<Color, E> {
        if value < 0 {
            Err(E::custom(format!("color value out of range: {}", value)))
        } else {
            self.visit_u8(value as u8)
        }
    }

    fn visit_u8<E: Error>(self, value: u8) -> Result<Color, E> {
        self.visit_u32(value as u32)
    }

    fn visit_i16<E: Error>(self, value: i16) -> Result<Color, E> {
        if value < 0 {
            Err(E::custom(format!("color value out of range: {}", value)))
        } else {
            self.visit_u16(value as u16)
        }
    }

    fn visit_u16<E: Error>(self, value: u16) -> Result<Color, E> {
        self.visit_u32(value as u32)
    }

    fn visit_i32<E: Error>(self, value: i32) -> Result<Color, E> {
        if value < 0 {
            Err(E::custom(format!("color value out of range: {}", value)))
        } else {
            self.visit_u32(value as u32)
        }
    }

    fn visit_u32<E: Error>(self, value: u32) -> Result<Color, E> {
        Ok(value.into())
    }

    fn visit_i64<E: Error>(self, value: i64) -> Result<Color, E> {
        if value < 0 || value > u32::MAX as i64 {
            Err(E::custom(format!("color value out of range: {}", value)))
        } else {
            self.visit_u32(value as u32)
        }
    }

    fn visit_u64<E: Error>(self, value: u64) -> Result<Color, E> {
        if value > u32::MAX as u64 {
            Err(E::custom(format!("color value out of range: {}", value)))
        } else {
            self.visit_u32(value as u32)
        }
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Color, E> {
        value
            .parse()
            .map_err(|()| E::custom(format!("invalid color string: {}", value)))
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Color, A::Error> {
        let r: Option<u8> = seq.next_element()?;
        let g: Option<u8> = seq.next_element()?;
        let b: Option<u8> = seq.next_element()?;
        let end: Option<u8> = seq.next_element()?;
        match (r, g, b, end) {
            (Some(r), Some(g), Some(b), None) => Ok(Color(r, g, b)),
            _ => Err(A::Error::custom("invalid color")),
        }
    }
}
