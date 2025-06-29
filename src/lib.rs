pub mod options;
mod builder;
mod macros;
pub mod templates;
pub mod common;

mod axis_typing;

#[cfg(feature = "time_axis")]
pub mod time_axis;


#[cfg(feature = "chrono_axis")]
pub mod chrono_axis;

pub use axis_typing::{AxisKindMarker,TimeAxis,CategoryAxis,ValueAxis};
pub use options::EChartOptions;

