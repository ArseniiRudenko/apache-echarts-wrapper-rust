use crate::axis_typing::{AxisKindMarker, SerializeFormat, TimeAxis};
use chrono::{DateTime, TimeZone};
use serde::{Serialize, Serializer};


impl<T:TimeZone> SerializeFormat<DateTime<T>> for DateTime<T>{
    fn serialize<S>(value: &DateTime<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        value.timestamp_millis().serialize(serializer)
    }
}

impl<T: TimeZone> AxisKindMarker for DateTime<T> {
    type AxisType = TimeAxis;
    type Serialization  = Self;
}


impl SerializeFormat<chrono::NaiveDateTime> for chrono::NaiveDateTime{
    fn serialize<S>(value: &chrono::NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        value.and_utc().timestamp_millis().serialize(serializer)
    }
}

impl AxisKindMarker for chrono::NaiveDateTime {
    type AxisType = TimeAxis;
    type Serialization  = Self;
}

