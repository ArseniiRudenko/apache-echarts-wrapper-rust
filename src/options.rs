//! Auto-generated Rust bindings for Apache ECharts `option` JSON configuration.
//! This module provides strongly-typed structs for commonly used configuration options.
//! Extend as needed for additional ECharts features.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::common::Percent;
use crate::options::DataVariant::NamedPair;

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
#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl Into<DataValue> for u128  {
    fn into(self) -> DataValue {
        DataValue::U128(self)
    }
}

impl Into<DataValue> for u64  {
    fn into(self) -> DataValue {
        DataValue::U64(self)
    }
}

impl Into<DataValue> for u32  {
    fn into(self) -> DataValue {
        DataValue::U32(self)
    }
}

impl Into<DataValue> for u16  {
    fn into(self) -> DataValue {
        DataValue::U16(self)
    }
}

impl Into<DataValue> for u8  {
    fn into(self) -> DataValue {
        DataValue::U8(self)
    }
}

impl Into<DataValue> for i128  {
    fn into(self) -> DataValue {
        DataValue::I128(self)
    }
}

impl Into<DataValue> for i64  {
    fn into(self) -> DataValue {
        DataValue::I64(self)
    }
}

impl Into<DataValue> for i32  {
    fn into(self) -> DataValue {
        DataValue::I32(self)
    }
}

impl Into<DataValue> for i16  {
    fn into(self) -> DataValue {
        DataValue::I16(self)
    }
}

impl Into<DataValue> for i8  {
    fn into(self) -> DataValue {
        DataValue::I8(self)
    }
}

impl Into<DataValue> for isize  {
    fn into(self) -> DataValue {
        DataValue::Isize(self)
    }
}

impl Into<DataValue> for f32  {
    fn into(self) -> DataValue {
        DataValue::F32(self)
    }
}

impl Into<DataValue> for f64  {
    fn into(self) -> DataValue {
        DataValue::F64(self)
    }
}

impl Into<DataValue> for String  {
    fn into(self) -> DataValue {
        DataValue::String(self)
    }
}

impl Into<DataValue> for &str  {
    fn into(self) -> DataValue {
        DataValue::String(self.to_string())
    }
}




#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct  NamedValuePair{
    value: [DataValue; 2],
    name: String,
}

impl NamedValuePair {
    pub fn new(x: DataValue, y: DataValue, name: String) -> Self {
        Self{
            value: [x, y],
            name,
        }
    }

}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct  NamedValue{
    value: DataValue,
    name: String,
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


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum DataVariant{
    /// Single dimension data (string/int/float)
    Data(Vec<DataValue>),

    /// Two-dimensional data: [x, y]
    Pair(Vec<[DataValue; 2]>),

    /// Single dimension data as an object with a name field
    Named(Vec<NamedValue>),

    /// Two-dimensional data with the name: { value: [x, y], name: "str" }
    NamedPair(Vec<NamedValuePair>),
}

/// Internal enum to represent the data source for a series
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SeriesDataSource {
    /// Direct data
    Data(DataVariant),
    /// Reference to a dataset by index
    DatasetIndex(usize)
}




impl<X,Y> Into<SeriesDataSource> for Vec<(X, Y)>
where X: Into<DataValue>,
Y: Into<DataValue>
{
    fn into(self) -> SeriesDataSource {
        SeriesDataSource::from_value_pairs(self)
    }

}

impl<X,Y> Into<SeriesDataSource> for Vec<(X, Y, String)>
where X: Into<DataValue>,
Y: Into<DataValue>
{
    fn into(self) -> SeriesDataSource {
        SeriesDataSource::from_tuples_with_label(self)
    }
}

impl<X> Into<SeriesDataSource> for Vec<X>
where X: Into<DataValue>
{
    fn into(self) -> SeriesDataSource {
        SeriesDataSource::from_values(self)
    }
}



impl SeriesDataSource {
    /// Creates a new SeriesDataSource that references a dataset by index
    pub fn from_dataset_index(index: usize) -> Self {
        Self::DatasetIndex(index)
    }

    /// Creates a SeriesDataSource with simple array of values
    pub fn with_values(values: Vec<DataValue>) -> Self {
        Self::Data(DataVariant::Data(values))
    }

    /// Creates a SeriesDataSource with array of [x,y] pairs
    pub fn with_pairs(pairs: Vec<[DataValue; 2]>) -> Self {
        Self::Data(DataVariant::Pair(pairs))
    }

   pub fn from_named_value_pairs(named_pairs: Vec<NamedValuePair>) -> Self{
       Self::Data(NamedPair(named_pairs))
   }


    /// Creates a SeriesDataSource with named values
    pub fn with_named_values(named_values: Vec<NamedValue>) -> Self {
        Self::Data(DataVariant::Named(named_values))
    }

    pub fn from_labeled_values<X>(values: Vec<(X, String)>) -> Self
    where
        X: Into<DataValue>
    {
        let data_named_values = values.into_iter().map(|(x,label)| NamedValue{
            value: x.into(),
            name: label
        }).collect();
        Self::with_named_values(data_named_values)
    }


    /// Creates a SeriesDataSource with named pairs
    pub fn from_tuples_with_label<X, Y>(values: Vec<(X, Y, String)>) -> Self
    where
        X: Into<DataValue>,
        Y: Into<DataValue>
    {
        let data_named_pairs = values.into_iter()
            .map(|(x, y,label) | NamedValuePair::new(x.into(), y.into(),label))
            .collect();
        Self::from_named_value_pairs(data_named_pairs)
    }

    /// Helper method to create a SeriesDataSource from a vector of any type that can be converted to DataValue
    pub fn from_values<T>(values: Vec<T>) -> Self
    where
        T: Into<DataValue>
    {
        Self::with_values(values.into_iter().map(|v| v.into()).collect())
    }

    /// Helper method to create a SeriesDataSource from pairs of values that can be converted to DataValue
    pub fn from_value_pairs<X, Y>(pairs: Vec<(X, Y)>) -> Self
    where
        X: Into<DataValue>,
        Y: Into<DataValue>
    {
        let data_pairs = pairs.into_iter()
            .map(|(x, y)| [x.into(), y.into()])
            .collect();
        Self::with_pairs(data_pairs)
    }
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
    pub fn new(name:&str, r#type: SeriesType, data: SeriesDataSource) -> Series {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LabelledSource{
    pub source: Vec<[DataValue; 3]>,
}


/// Dataset component for providing and transforming data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum DatasetComponent {
    Source(Source),
    LabelledSource(LabelledSource),
    Transform(Transform)
}


impl<X,Y> Into<DatasetComponent> for Vec<(X,Y)>
where X: Into<DataValue>,
      Y: Into<DataValue>
{
    fn into(self) -> DatasetComponent {
        DatasetComponent::src(self.into_iter().map(|(x,y)| [x.into(), y.into()]).collect())
    }
}

impl<X,Y> Into<DatasetComponent> for Vec<(X,Y,String)>
where X: Into<DataValue>,
      Y: Into<DataValue>
{
    fn into(self) -> DatasetComponent {
        DatasetComponent::labelled_source(self.into_iter().map(|(x,y,z)| [x.into(), y.into(), z.into()]).collect())
    }
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

    /// Constructor for the labeled source. Always put the label last
    pub fn labelled_source(source: Vec<[DataValue; 3]>) -> Self {
        Self::LabelledSource(
            LabelledSource{
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
