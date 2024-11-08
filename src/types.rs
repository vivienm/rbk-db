use std::{fmt, str::FromStr};

use serde::Deserialize;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl FromStr for Rgb {
    type Err = ParseRgbError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix('#').unwrap_or(s);
        if s.len() != 6 {
            return Err(ParseRgbError(()));
        }
        let r = u8::from_str_radix(&s[0..2], 16).map_err(|_| ParseRgbError(()))?;
        let g = u8::from_str_radix(&s[2..4], 16).map_err(|_| ParseRgbError(()))?;
        let b = u8::from_str_radix(&s[4..6], 16).map_err(|_| ParseRgbError(()))?;
        Ok(Rgb { r, g, b })
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl<'de> Deserialize<'de> for Rgb {
    fn deserialize<D>(deserializer: D) -> Result<Rgb, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Rgb::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ParseRgbError(());

impl fmt::Display for ParseRgbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("invalid RGB color")
    }
}

impl std::error::Error for ParseRgbError {}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum PartRelationType {
    Print,
    Pair,
    SubPart,
    Mold,
    Pattern,
    Alternate,
}

impl fmt::Display for PartRelationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Print => "print",
            Self::Pair => "pair",
            Self::SubPart => "subpart",
            Self::Mold => "mold",
            Self::Pattern => "pattern",
            Self::Alternate => "alternate",
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum PartMaterial {
    CardboardPaper,
    Cloth,
    FlexiblePlastic,
    Foam,
    Metal,
    Plastic,
    Rubber,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Rgb;

    #[test]
    fn from_str() -> anyhow::Result<()> {
        assert_eq!(
            Rgb::from_str("#ff00ff")?,
            Rgb {
                r: 0xff,
                g: 0x00,
                b: 0xff
            }
        );
        assert!(Rgb::from_str("#ff00f").is_err());
        assert!(Rgb::from_str("#ff00ff0").is_err());
        assert!(Rgb::from_str("#ff00fg").is_err());
        Ok(())
    }
}
