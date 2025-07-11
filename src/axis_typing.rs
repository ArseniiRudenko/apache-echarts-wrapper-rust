use serde::{Serialize, Serializer};
use crate::options::AxisType;

pub trait AxisInfo{
    const AXIS_TYPE: AxisType;
}
pub struct ValueAxis;

pub struct CategoryAxis;

pub struct TimeAxis;

impl AxisInfo for TimeAxis {
    const AXIS_TYPE: AxisType = AxisType::Time;
}

impl AxisInfo for ValueAxis {
    const AXIS_TYPE: AxisType = AxisType::Value;
}

impl AxisInfo for CategoryAxis {
    const AXIS_TYPE: AxisType = AxisType::Category;
}

pub struct DefaultSerialisation;

pub trait SerializeFormat<T: ?Sized> {
    fn serialize<S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

impl<T: serde::Serialize + ?Sized> SerializeFormat<T> for DefaultSerialisation {
    fn serialize<S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.serialize(serializer)
    }
}

//provides a configurable serialization implementation for type(because we don't have access to with_serializer for generics)
#[derive(Debug,Clone)]
pub struct ValueSerializeWrapper<T: AxisKindMarker>(T);


impl <T: AxisKindMarker> From<T> for ValueSerializeWrapper<T> {
    fn from(value: T) -> Self {
        ValueSerializeWrapper(value)
    }
}

impl<T: AxisKindMarker> Serialize for ValueSerializeWrapper<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        T::Serialization::serialize(&self.0, serializer)
    }
}

pub trait AxisKindMarker {
    type AxisType : AxisInfo;
    type Serialization: SerializeFormat<Self>
    where
        Self: Sized;
}

use crate::impl_default_marker;

impl_default_marker!(ValueAxis for u128 i128 i32 u32 i64 u64 i16 u16 i8 u8 f32 f64 usize isize);

impl_default_marker!(CategoryAxis for String &str);
