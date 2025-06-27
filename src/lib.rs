pub mod options;
mod builder;
mod macros;
pub mod templates;
pub mod common;

#[cfg(feature = "time_axis")]
pub mod time_axis;
mod axis_typing;

#[cfg(feature = "chrono_axis")]
mod chrono_axis;

pub use builder::RegressionChartBuilder;
pub use axis_typing::{AxisKindMarker,TimeAxis,CategoryAxis,ValueAxis};
pub use options::EChartOptions;

