use crate::axis_typing::{AxisKindMarker, CategoryAxis, SerializeFormat, TimeAxis};
use chrono::{DateTime, TimeZone, Weekday};
use serde::{Serialize, Serializer};


use crate::impl_default_marker_self;

impl_default_marker_self!(CategoryAxis for Weekday chrono::Month);


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

impl SerializeFormat<chrono::Month> for chrono::Month{
    fn serialize<S>(value: &chrono::Month, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        value.name().serialize(serializer)
    }
}

impl SerializeFormat<Weekday> for Weekday{
    fn serialize<S>(value: &Weekday, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        value.to_string().serialize(serializer)
    }
}

