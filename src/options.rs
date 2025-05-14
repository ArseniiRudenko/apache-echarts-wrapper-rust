//! Auto-generated Rust bindings for Apache ECharts `option` JSON configuration.
//! This module provides strongly-typed structs for commonly used configuration options.
//! Extend as needed for additional ECharts features.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_axis: Option<Axis>,

    /// Y-axis options (Cartesian charts)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_axis: Option<Axis>,

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
    Center,
}

/// Newtype for percentage values, serialized as "{value}%"
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percent(pub f64);

impl Serialize for Percent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        let s = format!("{}%", self.0);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Percent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        let trimmed = s.trim_end_matches('%');
        trimmed.parse::<f64>()
            .map(Percent)
            .map_err(serde::de::Error::custom)
    }
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
    Percent(Percent),
    /// Arbitrary string (e.g., expressions)
    Other(String),
}

/// Title component
#[derive(Serialize, Deserialize, Debug, Clone)]
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
enum TooltipTrigger {
    Item,
    Axis,
    None
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

    /// Additional raw tooltip options
    #[serde(flatten)]
    pub extra: Option<Value>,
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
    Int(i64),
    Float(f64),
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
#[serde(rename_all= "snake_case")]
pub enum SeriesDataSource {
    /// Direct data items
    Data(Vec<DataItem>),
    /// Reference to a dataset by index
    DatasetIndex(usize),
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




/// Dataset component for providing and transforming data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DatasetComponent {
    /// Raw data source (array of arrays or objects)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Vec<[DataValue; 2]>>,

    /// Transform applied to this dataset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<DatasetTransform>,

    /// Additional raw dataset options
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
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
