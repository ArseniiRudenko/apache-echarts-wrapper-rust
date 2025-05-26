use crate::axis_typing::{AxisKindMarker, DefaultSerialisation, TimeAxis};

impl AxisKindMarker for chrono::DateTime<T: chrono::TimeZone> {
    type AxisType = TimeAxis;
    type Serialization where Self: Sized = DefaultSerialisation;
}

impl AxisKindMarker for chrono::NaiveDateTime {
    type AxisType = TimeAxis;
    type Serialization where Self: Sized = DefaultSerialisation;   
}

