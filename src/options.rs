//! Auto-generated Rust bindings for Apache ECharts `option` JSON configuration.
//! This module provides strongly-typed structs for commonly used configuration options.
//! Extend as needed for additional ECharts features.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::common::Percent;

/// Root object for ECharts configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EChartsOption {
    /// Chart title options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,

    /// Grid positioning and style options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid: Option<Grid>,

    /// Tooltip options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<Tooltip>,

    /// Legend options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<Legend>,

    /// Dataset component for providing data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<Vec<DatasetComponent>>,

    /// X-axis options (Cartesian charts)
    pub x_axis: Axis,

    /// Y-axis options (Cartesian charts)
    pub y_axis: Axis,

    /// Series data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<Series>>,

    /// Additional raw options not covered by this binding
    #[serde(flatten)]
    pub extra: Option<Value>,
}

/// Keyword positions supported by ECharts
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PositionKeyword {
    Left,
    Right,
    Top,
    Bottom,
    Middle,
    Center,
    Auto
}




/// Position enum supporting keyword, numeric px, percent, or other strings
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Position {
    /// Predefined keyword position
    Keyword(PositionKeyword),
    /// Numeric pixel value
    Number(f64),
    /// Percentage value (e.g., 50 => "50%")
    Percent(Percent)
}

/// Title component
#[derive(Serialize, Deserialize, Debug, Clone,Default)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    /// Main title text
    pub text: Option<String>,

    /// Subtitle text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_text: Option<String>,

    /// Link for title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,

    /// Left position (Keyword, numeric px, percent, or other)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<Position>,

    /// Top position (Keyword, numeric px, percent, or other)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<Position>,

    /// Right position (Keyword, numeric px, percent, or other)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<Position>,

    /// Bottom position (Keyword, numeric px, percent, or other)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<Position>,

    /// Additional raw title options
    #[serde(flatten)]
    pub extra: Option<Value>,
}

impl Title {
    pub(crate) fn new(text: &str) -> Title {
        Self{
            text: Some(text.to_string()),
            sub_text: None,
            link: None,
            left: None,
            top: None,
            right: None,
            bottom: None,
            extra: None,
        }
    }
}

/// Grid component
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Grid {
    /// Distance between grid and left side (Keyword, numeric px, percent, or other)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<Position>,

    /// Distance between grid and right side (Keyword, numeric px, percent, or other)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<Position>,

    /// Distance between grid and top side (Keyword, numeric px, percent, or other)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<Position>,

    /// Distance between grid and bottom side (Keyword, numeric px, percent, or other)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<Position>,

    /// Whether the grid area contain the axis labels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contain_label: Option<bool>,

    /// Background color of the grid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<Value>,

    /// Border color of the grid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<Value>,

    /// Border width of the grid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_width: Option<f64>,

    /// Show the border of the grid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,

    /// z-index of the grid component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z: Option<i32>,

    /// z-level of the grid component
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zlevel: Option<i32>,

    /// Additional raw grid options
    #[serde(flatten)]
    pub extra: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum TooltipTrigger {
    Item,
    Axis,
    None
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AxisPointerType {
    Cross,
    Line,
    Shadow,
    None
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AxisPointerTargetAxis{
    Auto,
    X,
    Y,
    Radius,
    Angle
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AxisPointer{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AxisPointerType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub snap : Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation : Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis: Option<AxisPointerTargetAxis>


}

/// Tooltip component
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tooltip {
    pub show: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_delay: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_delay: Option<i32>,

    /// Trigger mode: item, axis
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger: Option<TooltipTrigger>,

    /// Tooltip formatter template or callback
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_pointer: Option<AxisPointer>,

}

/// Legend component
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Legend {
    /// Data items in the legend
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,

    /// Legend orientation: vertical or horizontal
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orient: Option<String>,

    /// Left position (Keyword, numeric px, percent, or other)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<Position>,

    /// Additional raw legend options
    #[serde(flatten)]
    pub extra: Option<Value>,
}

/// Axis types supported by ECharts
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AxisType {
    Value,
    Category,
    Time,
    Log,
    /// For any unrecognized axis type
    #[serde(other)]
    Unknown,
}

/// Primitive values allowed in data (string, int, float)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum DataValue {
    Isize(isize),
    I128(i128),
    I64(i64),
    I32(i32),
    I16(i16),
    I8(i8),
    Usize(usize),
    U128(u128),
    U64(u64),
    U32(u32),
    U16(u16),
    U8(u8),
    F32(f32),
    F64(f64),
    String(String),
}



/// A single data item: either a single value or a tuple [x, y]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum DataItem {
    /// Single dimension data (string/int/float)
    Single(DataValue),
    /// Two-dimensional data: [x, y]
    Pair([DataValue; 2]),
}


/// Axis (cartesian)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Axis {
    /// Axis type: value, category, time, log
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AxisType>,

    /// Axis name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Data for category axis
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<DataValue>>,

    /// Additional raw axis options
    #[serde(flatten)]
    pub extra: Option<Value>,
}

/// Available series types in ECharts
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SeriesType {
    Line,
    Bar,
    Pie,
    Scatter,
    EffectScatter,
    Radar,
    Tree,
    Treemap,
    Sunburst,
    Boxplot,
    Candlestick,
    Heatmap,
    Map,
    Parallel,
    Lines,
    Graph,
    Sankey,
    Funnel,
    Gauge,
    PictorialBar,
    ThemeRiver,
    Custom,
    /// For any unrecognized series type
    #[serde(other)]
    Unknown,
}

/// Internal enum to represent the data source for a series
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SeriesDataSource {
    /// Direct data items
    Data(Vec<DataItem>),
    /// Reference to a dataset by index
    DatasetIndex(usize),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DataPointSymbol {
    Circle,
    Rect,
    RoundRect,
    Triangle,
    Diamond,
    Pin,
    Arrow,
    None
}

/// Series definition
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    /// Chart type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SeriesType>,

    /// Series name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub smooth: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_style: Option<AreaStyle>,

    /// Data array
    #[serde(flatten)]
    pub data: SeriesDataSource,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<DataPointSymbol>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_size: Option<usize>,

    /// Additional raw series options
    #[serde(flatten)]
    pub extra: Option<Value>,
}

impl Series {
    pub(crate) fn new(name:&str, r#type: SeriesType, data: SeriesDataSource) -> Series {
        Self{
            r#type: Some(r#type),
            name: Some(name.to_string()),
            smooth: None,
            area_style: None,
            data,
            symbol: None,
            symbol_size: None,
            extra: None,
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AxisFillOrigin{
    Auto,
    Start,
    End,
    Number
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AreaStyle{
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    origin: Option<AxisFillOrigin>
}



#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transform{
    pub transform: DatasetTransform,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_dataset_index: Option<usize>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Source{
    pub source: Vec<[DataValue; 2]>,
}


/// Dataset component for providing and transforming data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum DatasetComponent {
    Source(Source),
    Transform(Transform)
}

impl DatasetComponent {
    pub fn tr(transform: DatasetTransform,index: usize) -> Self{
        Self::Transform(
            Transform {
                transform,
                from_dataset_index: Some(index),
            }
        )
    }

    pub fn src(source: Vec<[DataValue; 2]>) -> Self {
        Self::Source(
            Source {
                source
            }
        )
    }

}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DatasetTransformType {
    Filter,
    Sort,
    #[serde(rename = "ecStat:regression")]
    Regression
}

/// Transform applied to a dataset
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DatasetTransform {
    /// Transform type
    pub r#type: DatasetTransformType,

    /// Transform configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<RegressionConfig>,

    /// Additional raw transform options
    #[serde(flatten)]
    pub extra: Option<Value>,
}

/// Regression methods supported by ecStat
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RegressionMethod {
    Linear,
    Exponential,
    Logarithmic,
    Polynomial,
}

/// Configuration for regression transforms
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegressionConfig {
    /// Regression method
    pub method: RegressionMethod,

    /// Polynomial order (only used when method is Polynomial)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,

    /// Additional raw regression config options
    #[serde(flatten)]
    pub extra: Option<Value>,
}
