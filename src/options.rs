use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::builder::AxisKindMarker;
use crate::builder::AxisInfo;
use crate::common::Percent;

/// Root object for ECharts configuration
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EChartsOption<X:AxisKindMarker,Y:AxisKindMarker> {
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
    pub dataset: Option<Vec<DatasetComponent<X,Y>>>,

    /// X-axis options (Cartesian charts)
    pub x_axis: Axis<X>,

    /// Y-axis options (Cartesian charts)
    pub y_axis: Axis<Y>,

    /// Series data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<Series<X,Y>>>,

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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AxisType {
    Value,
    Category,
    Time,
    Log
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct  NamedValuePair<X,Y>{
    value: (X,Y),
    name: String,
}

impl<X,Y> NamedValuePair<X,Y> {
    pub fn new(x: X, y: Y, name: String) -> Self {
        Self{
            value: (x, y),
            name,
        }
    }

}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct  NamedValue<X>{
    value: X,
    name: String,
}


/// Axis (cartesian)
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Axis<T:AxisKindMarker> {
    /// Axis type: value, category, time, log
    pub r#type: AxisType,
    
    pub inverse: Option<bool>,

    /// Axis name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Data for category axis
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<T>>,

    /// Additional raw axis options
    #[serde(flatten)]
    pub extra: Option<Value>,
}

impl<T:AxisKindMarker> Default for Axis<T> {
    fn default() -> Self {
        Self{
            r#type: T::AxisType::AXIS_TYPE,
            inverse: None,
            name: None,
            data: None,
            extra: None,
        }
    }
}


impl<T:AxisKindMarker> Axis<T>{
    pub fn new(name: &str, is_log:bool)-> Self{
        if T::AxisType::AXIS_TYPE == AxisType::Value && is_log {
            Self{
                r#type:  AxisType::Log,
                name: Some(name.to_string()),
                inverse: None,
                data: None,
                extra: None,
            }
        }else {
            Self{
                r#type:  T::AxisType::AXIS_TYPE,
                name: Some(name.to_string()),
                inverse: None,
                data: None,
                extra: None,
            }
        }
    }
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
pub enum DataVariant<X,Y>{
    /// Single dimension data (string/int/float)
    Data(Vec<X>),

    /// Two-dimensional data: [x, y]
    Pair(Vec<(X,Y)>),

    /// Single dimension data as an object with a name field
    Named(Vec<NamedValue<X>>),

    /// Two-dimensional data with the name: { value: [x, y], name: "str" }
    NamedPair(Vec<NamedValuePair<X,Y>>),
}

/// Internal enum to represent the data source for a series
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SeriesDataSource<X,Y>{
    /// Direct data
    Data(DataVariant<X,Y>),
    /// Reference to a dataset by index
    DatasetIndex(usize)
}

impl<X,Y> Into<SeriesDataSource<X,Y>> for Vec<(X, Y)>
{
    fn into(self) -> SeriesDataSource<X,Y> {
        SeriesDataSource::from_pairs(self)
    }

}

impl<X,Y> Into<SeriesDataSource<X,Y>> for Vec<(X, Y, String)>
{
    fn into(self) -> SeriesDataSource<X,Y> {
        SeriesDataSource::from_tuples_with_label(self)
    }
}

impl<X,Y> Into<SeriesDataSource<X,Y>> for Vec<X>
{
    fn into(self) -> SeriesDataSource<X,Y> {
        SeriesDataSource::from_values(self)
    }
}

impl<X,Y> SeriesDataSource<X,Y> {
    /// Creates a new SeriesDataSource that references a dataset by index
    pub fn from_dataset_index(index: usize) -> Self {
        Self::DatasetIndex(index)
    }

    /// Creates a SeriesDataSource with a simple array of values
    pub fn from_values(values: Vec<X>) -> Self {
        Self::Data(DataVariant::Data(values))
    }

    /// Creates a SeriesDataSource with an array of [x,y] pairs
    pub fn from_pairs(pairs: Vec<(X,Y)>) -> Self {
        Self::Data(DataVariant::Pair(pairs))
    }

   pub fn from_named_value_pairs(named_pairs: Vec<NamedValuePair<X,Y>>) -> Self{
       Self::Data(DataVariant::NamedPair(named_pairs))
   }


    /// Creates a SeriesDataSource with named values
    pub fn with_named_values(named_values: Vec<NamedValue<X>>) -> Self {
        Self::Data(DataVariant::Named(named_values))
    }

    pub fn from_labeled_values(values: Vec<(X, String)>) -> Self
    {
        let data_named_values = values.into_iter().map(|(x,label)| NamedValue{
            value: x.into(),
            name: label
        }).collect();
        Self::with_named_values(data_named_values)
    }

    /// Creates a SeriesDataSource with named pairs
    pub fn from_tuples_with_label(values: Vec<(X, Y, String)>) -> Self
    {
        let data_named_pairs = values.into_iter()
            .map(|(x, y,label) | NamedValuePair::new(x.into(), y.into(),label))
            .collect();
        Self::from_named_value_pairs(data_named_pairs)
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
pub struct Series<X,Y> {
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
    pub data: SeriesDataSource<X,Y>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<DataPointSymbol>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol_size: Option<usize>,

    /// Additional raw series options
    #[serde(flatten)]
    pub extra: Option<Value>,
}

impl<X,Y> Series <X,Y>{
    pub fn new(name:&str, r#type: SeriesType, data: SeriesDataSource<X,Y>) -> Series<X,Y> {
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
pub struct Source<X,Y>{
    pub source: Vec<(X,Y)>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LabelledSource<X,Y>{
    pub source: Vec<(X,Y,String)>,
}


/// Dataset component for providing and transforming data
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum DatasetComponent<X,Y> {
    Source(Source<X,Y>),
    LabelledSource(LabelledSource<X,Y>),
    Transform(Vec<Transform>)
}


impl<X,Y> Into<DatasetComponent<X,Y>> for Vec<(X,Y)>
{
    fn into(self) -> DatasetComponent<X,Y> {
        DatasetComponent::src(self)
    }
}

impl<X,Y> Into<DatasetComponent<X,Y>> for Vec<(X,Y,String)>
{
    fn into(self) -> DatasetComponent<X,Y> {
        DatasetComponent::labelled_source(self)
    }
}



impl<X,Y> DatasetComponent<X,Y> {
    pub fn tr(transform: DatasetTransform, index: usize) -> Self{
        Self::Transform(vec![Transform {
            transform,
            from_dataset_index: Some(index),
        }])
    }

    pub fn trs(transform: Vec<DatasetTransform>, index: usize) -> Self{
        Self::Transform(
            transform
                .into_iter()
                .map(|tr|
                    Transform {
                        transform:tr,
                        from_dataset_index: Some(index)
                    }
                )
            .collect()
        )
    }

    pub fn src(source: Vec<(X,Y)>) -> Self {
        Self::Source(
            Source {
                source
            }
        )
    }

    /// Constructor for the labeled source. Always put the label last
    pub fn labelled_source(source: Vec<(X,Y,String)>) -> Self {
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
    Boxplot,
    #[serde(rename = "ecStat:regression")]
    Regression,
    #[serde(rename = "ecStat:clustering")]
    Clustering
}

/// Transform applied to a dataset
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DatasetTransform {
    /// Transform type
    r#type: DatasetTransformType,

    /// Transform configuration
    config: DatasetTransformConfig,

}


impl DatasetTransform {

    pub fn regression(config: RegressionConfig) -> Self {
        Self{
            r#type: DatasetTransformType::Regression,
            config: DatasetTransformConfig::Regression(config)
        }
    }

    pub fn clustering(clustering_config: ClusteringConfig)->Self{
        Self{
            r#type: DatasetTransformType::Clustering,
            config: DatasetTransformConfig::Clustering(clustering_config)
        }
    }

    pub fn sort(sort_config: SortConfig) -> Self {
        Self{
            r#type: DatasetTransformType::Sort,
            config: DatasetTransformConfig::Sort(sort_config)
        }
    }

}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
enum DatasetTransformConfig {
    Regression(RegressionConfig),
    Clustering(ClusteringConfig),
    Sort(SortConfig)
}


/// regression methods supported by ecStat
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RegressionMethod {
    Linear,
    Exponential,
    Logarithmic,
    Polynomial,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SortConfig {
    dimension: u8,
    order : Order
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Order {
    Asc,
    Desc
}


/// Configuration for regression transforms
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegressionConfig {
    /// Regression method
    pub method: RegressionMethod,

    /// Polynomial order (only used when method is Polynomial)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u8>,

    /// Additional raw regression config options
    #[serde(flatten)]
    pub extra: Option<Value>,
}



/// Configuration for clustering transforms
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClusteringConfig {

    /// The number of clusters to generate (must be > 1).
    pub cluster_count: u8,

    ///dimension to which the cluster index will be written
    pub output_cluster_index_dimension: u8,

    /// dimensions of source data that will be used in calculation of a cluster
    pub dimensions : Option<Vec<usize>>,

}
