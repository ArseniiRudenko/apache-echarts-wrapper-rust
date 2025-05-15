use std::fmt::Write;
use sailfish::RenderError;
use sailfish::runtime::{Buffer, Render};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Newtype for percentage values, serialized as "{value}%"
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percent(pub f64);

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
        trimmed.parse::<f64>()
            .map(Percent)
            .map_err(serde::de::Error::custom)
    }
}


/// Newtype for percentage values, serialized as "{value}px"
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixels(pub i64);

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
        trimmed.parse::<i64>()
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

    pub fn percent(f: f64) -> Self{
        Size::Percent(Percent(f))
    }

    pub fn pixels(f: i64) -> Self{
        Size::Pixel(Pixels(f))
    }

}