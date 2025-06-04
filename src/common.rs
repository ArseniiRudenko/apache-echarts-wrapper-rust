use std::fmt;
use std::fmt::Write;
use sailfish::RenderError;
use sailfish::runtime::{Buffer, Render};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Visitor;

/// Newtype for percentage values, serialized as "{value}%"
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percent(pub f32);

impl Serialize for Percent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let s = format!("{}%", self.0);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Percent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        let trimmed = s.trim_end_matches('%');
        trimmed.parse::<f32>()
            .map(Percent)
            .map_err(serde::de::Error::custom)
    }
}


/// Newtype for percentage values, serialized as "{value}px"
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixels(pub usize);

impl Serialize for Pixels {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let s = format!("{}px", self.0);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Pixels {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        let trimmed = s.trim_end_matches("px");
        trimmed.parse::<usize>()
            .map(Pixels)
            .map_err(serde::de::Error::custom)
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Size{
    Percent(Percent),
    Pixel(Pixels)
}


impl Render for Size{
    fn render(&self, b: &mut Buffer) -> Result<(), RenderError> {
        b.write_str(&self.to_string())?;
        Ok(())   
    }
}

impl Size{
    pub fn to_string(&self) -> String{
        match self{
            Size::Percent(p) => format!("{}%", p.0),
            Size::Pixel(p) => format!("{}px", p.0)
        }
    }

    pub fn percent(f: f32) -> Self{
        Size::Percent(Percent(f))
    }

    pub fn pixels(f: usize) -> Self{
        Size::Pixel(Pixels(f))
    }

}

#[derive(Debug,Clone, PartialEq)]
pub struct Rgb(pub u8,pub  u8,pub  u8);

impl Serialize for Rgb {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("rgb({}, {}, {})", self.0, self.1, self.2);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Rgb {
    fn deserialize<D>(deserializer: D) -> Result<Rgb, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RgbVisitor;

        impl<'de> Visitor<'de> for RgbVisitor {
            type Value = Rgb;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(r#"a string in the format "rgb(r, g, b)""#)
            }

            fn visit_str<E>(self, v: &str) -> Result<Rgb, E>
            where
                E: de::Error,
            {
                let v = v.trim();
                if !v.starts_with("rgb(") || !v.ends_with(')') {
                    return Err(E::custom("invalid format"));
                }

                let content = &v[4..v.len() - 1]; // strip "rgb(" and ")"
                let parts: Vec<&str> = content.split(',').map(str::trim).collect();
                if parts.len() != 3 {
                    return Err(E::custom("expected three components"));
                }

                let r = parts[0].parse::<u8>().map_err(E::custom)?;
                let g = parts[1].parse::<u8>().map_err(E::custom)?;
                let b = parts[2].parse::<u8>().map_err(E::custom)?;

                Ok(Rgb(r, g, b))
            }
        }

        deserializer.deserialize_str(RgbVisitor)
    }
}