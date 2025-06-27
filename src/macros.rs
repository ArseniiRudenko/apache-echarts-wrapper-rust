#[macro_export]
macro_rules! impl_default_marker {
    ($type:ident for $($t:ty)*) => ($(
        impl AxisKindMarker for $t {
            type AxisType = $type;
            type Serialization = DefaultSerialisation;
        }
    )*)
}

#[macro_export]
macro_rules! impl_default_marker_self {
    ($type:ident for $($t:ty)*) => ($(
        impl AxisKindMarker for $t {
            type AxisType = $type;
            type Serialization = Self;
        }
    )*)
}