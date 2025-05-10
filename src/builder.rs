use std::marker::PhantomData;
use crate::options::{Axis, AxisType, DataItem, DataValue, EChartsOption, Series, SeriesType, Title};

/// Trait to determine axis type and convert into DataValue
pub trait AxisInfo {
    /// The axis type for this data type
    fn axis_type() -> AxisType;
    /// Convert a value into DataValue
    fn into_data_value(self) -> DataValue;
}

// Implement AxisInfo for common types
impl AxisInfo for i32 {
    fn axis_type() -> AxisType { AxisType::Value }
    fn into_data_value(self) -> DataValue { DataValue::Int(self as i64) }
}
impl AxisInfo for i64 {
    fn axis_type() -> AxisType { AxisType::Value }
    fn into_data_value(self) -> DataValue { DataValue::Int(self) }
}
impl AxisInfo for f64 {
    fn axis_type() -> AxisType { AxisType::Value }
    fn into_data_value(self) -> DataValue { DataValue::Float(self) }
}

impl AxisInfo for f32 {
    fn axis_type() -> AxisType { AxisType::Value }
    fn into_data_value(self) -> DataValue { DataValue::Float(self as f64) }
}

impl AxisInfo for usize{
    fn axis_type() -> AxisType { AxisType::Value }
    fn into_data_value(self) -> DataValue { DataValue::Int(self as i64) }
}

impl AxisInfo for String {
    fn axis_type() -> AxisType { AxisType::Category }
    fn into_data_value(self) -> DataValue { DataValue::String(self) }
}

impl<'a> AxisInfo for &'a str {
    fn axis_type() -> AxisType { AxisType::Category }
    fn into_data_value(self) -> DataValue { DataValue::String(self.to_string()) }
}

/// Typed dataset with homogeneous X and Y types
pub struct Dataset<X, Y> {
    pub label: String,
    pub values: Vec<(X, Y)>,
}

/// Builder for multi-line charts, inferring axis types from X and Y
pub struct ChartBuilder<X: AxisInfo, Y: AxisInfo> {
    option: EChartsOption,
    _marker: PhantomData<(X, Y)>,
}

impl<X: AxisInfo, Y: AxisInfo> ChartBuilder<X, Y> {
    /// Create a builder; axes set according to X::axis_type and Y::axis_type
    pub fn new() -> Self {
        let opt = EChartsOption {
            title: None, tooltip: None, legend: None, grid: None, extra: None,
            x_axis: Some(Axis { r#type: Some(X::axis_type()), name: None, data: None, extra: None }),
            y_axis: Some(Axis { r#type: Some(Y::axis_type()), name: None, data: None, extra: None }),
            series: Some(Vec::new()),
        };
        Self { option: opt, _marker: Default::default() }
    }

    /// Set chart title
    pub fn title_str(mut self, title: &str) -> Self {
        self.option.title = Some(Title::new(title));
        self
    }
    pub fn title(mut self, title: Title) -> Self {
        self.option.title = Some(title);
        self
    }

    pub fn x_axis_label(mut self, x: &str) -> Self {
        self.option.x_axis.get_or_insert_default().name = Some(x.to_string());
        self
    }
    
    pub fn y_axis_label(mut self, y: &str) -> Self {
        self.option.y_axis.get_or_insert_default().name = Some(y.to_string());
        self
    }

    /// Add a dataset; X and Y conversions ensure homogeneous types
    pub fn add_dataset(mut self, series_label:&str, data: Vec<(X, Y)>, series_type: SeriesType) -> Self {
        let data_items: Vec<DataItem> =data.into_iter()
            .map(|(x, y)| DataItem::Pair([x.into_data_value(), y.into_data_value()]))
            .collect();
        self.option.series.as_mut().unwrap().push(Series {
            r#type: Some(series_type),
            name: Some(series_label.to_string()),
            data: Some(data_items),
            extra: None,
        });
        self
    }

    /// Build final EChartsOption, populating legend
    pub fn build(self) -> EChartsOption {
        self.option
    }
}
