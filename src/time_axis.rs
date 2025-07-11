use crate::axis_typing::{AxisKindMarker, CategoryAxis, DefaultSerialisation, SerializeFormat, TimeAxis};
use serde::ser::Error;
use serde::{Serialize, Serializer};
use time::macros::format_description;
use time::{OffsetDateTime, UtcDateTime};
use crate::impl_default_marker;
use crate::impl_default_marker_self;

impl_default_marker!(CategoryAxis for  time::Month);
impl_default_marker_self!(CategoryAxis for  time::Weekday time::Time);
impl_default_marker_self!(TimeAxis for  OffsetDateTime time::PrimitiveDateTime UtcDateTime time::Date);


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

impl SerializeFormat<OffsetDateTime> for OffsetDateTime{
    fn serialize<S>(value: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        (value.unix_timestamp_nanos()/ 1000000).serialize(serializer)
    }
}


impl SerializeFormat<time::PrimitiveDateTime> for time::PrimitiveDateTime{
    fn serialize<S>(value: &time::PrimitiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        (value.as_utc().unix_timestamp_nanos() / 1000000).serialize(serializer)
    }
}


impl SerializeFormat<UtcDateTime> for UtcDateTime{
    fn serialize<S>(value: &UtcDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        (value.unix_timestamp_nanos()/ 1000000).serialize(serializer)
    }
}


impl SerializeFormat<time::Date> for time::Date{
    fn serialize<S>(value: &time::Date, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        (value.midnight().as_utc().unix_timestamp_nanos() / 1000000).serialize(serializer)
    }
}


impl SerializeFormat<time::Time> for time::Time{
    fn serialize<S>(value: &time::Time, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match value.format(format_description!("[hour]:[minute]:[second]")) {
            Ok(format) => format.serialize(serializer),
            Err(err) => Err(Error::custom(err.to_string()))
        }
    }
}

