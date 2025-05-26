use crate::axis_typing::{AxisKindMarker, CategoryAxis, DefaultSerialisation, SerializeFormat, TimeAxis};
use serde::{Serialize, Serializer};


impl<T: serde::Serialize + ?Sized + ToString> SerializeFormat<T> for time::Weekday{
    fn serialize<S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        value.to_string().serialize(serializer)
    }
}

impl<T: serde::Serialize + ?Sized + ToString> SerializeFormat<T> for time::Month{
    fn serialize<S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        value.to_string().serialize(serializer)
    }
}


impl AxisKindMarker for time::Weekday{
    type AxisType = CategoryAxis;
    type Serialization = Self;
    
}

impl AxisKindMarker for time::Month{
    type AxisType = CategoryAxis;
    type Serialization = DefaultSerialisation;
}

impl AxisKindMarker for time::OffsetDateTime{
    type AxisType = TimeAxis;
    type Serialization = DefaultSerialisation;
}

impl AxisKindMarker for time::PrimitiveDateTime{
    type AxisType = TimeAxis;
    type Serialization = DefaultSerialisation;
}

impl AxisKindMarker for time::UtcDateTime{
    type AxisType = TimeAxis;
    type Serialization = DefaultSerialisation;
}

impl AxisKindMarker for time::Date {
    type AxisType = TimeAxis;
    type Serialization = DefaultSerialisation;
}

impl AxisKindMarker for time::Time{
    type AxisType = TimeAxis;
    type Serialization = DefaultSerialisation;
    
}
