use std::marker::PhantomData;
use serde_json::json;
use crate::options::{Axis, AxisType, DataItem, DataValue, DatasetComponent, DatasetTransform, EChartsOption, RegressionConfig, RegressionMethod, Series, SeriesDataSource, SeriesType, Title};

/// Trait to determine an axis type and convert into DataValue
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
            title: None, tooltip: None, legend: None, grid: None, extra: None, dataset: None,
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
            data: SeriesDataSource::Data(data_items),
            extra: None,
        });
        self
    }

    /// Add a dataset with regression transformation
    fn add_regression_dataset(mut self, series_label: &str, data: Vec<(X, Y)>,
                                  method: RegressionMethod, order: Option<u32>, 
                                  series_type: SeriesType) -> Self {
        // Create a dataset vector if it doesn't exist
        if self.option.dataset.is_none() {
            self.option.dataset = Some(Vec::new());
        }
        
        // Convert the data to a format suitable for ECharts
        let raw_data = data.into_iter()
            .map(|(x, y)| [x.into_data_value(), y.into_data_value()])
            .collect::<Vec<_>>();
        
        // Add source dataset
        let datasets = self.option.dataset.as_mut().unwrap();
        let source_index = datasets.len();
        datasets.push(DatasetComponent {
            source: Some(raw_data),
            transform: None,
            extra: None,
        });



        // Add regression transform dataset
        let transform_index = datasets.len();
        let mut regression_config = RegressionConfig {
            method: method.clone(),
            order: None,
            extra: None,

        };

        // Add polynomial order if provided and the method is polynomial
        if method == RegressionMethod::Polynomial {
            regression_config.order = order;
        }

        
        datasets.push(DatasetComponent {
            source: None,
            transform: Some(DatasetTransform {
                r#type: "ecStat:regression".to_string(),
                config: Some(regression_config),
                extra: None,
            }),
            extra: None,
        });
        
        // Add scatter series for original data
        self.option.series.as_mut().unwrap().push(Series {
            r#type: Some(SeriesType::Scatter),
            name: Some(format!("{} (data)", series_label)),
            data: SeriesDataSource::DatasetIndex(source_index),
            extra: None,
        });
        
        // Add line series for regression
        self.option.series.as_mut().unwrap().push(Series {
            r#type: Some(series_type),
            name: Some(format!("{} (regression)", series_label)),
            data: SeriesDataSource::DatasetIndex(transform_index),
            extra: Some(json!({
                "smooth": true,
                "symbolSize": 0.1,
                "symbol": "circle",
                "label": { "show": true, "fontSize": 16 },
                "labelLayout": { "dx": -20 },
                "encode": { "label": 2, "tooltip": 1 }
            })),
        });
        
        self
    }
    
    /// Add a linear regression dataset
    pub fn add_linear_regression(self, series_label: &str, data: Vec<(X, Y)>) -> Self {
        self.add_regression_dataset(series_label, data, RegressionMethod::Linear, None, SeriesType::Line)
    }
    
    /// Add a polynomial regression dataset
    pub fn add_polynomial_regression(self, series_label: &str, data: Vec<(X, Y)>, order: u32) -> Self {
        self.add_regression_dataset(series_label, data, RegressionMethod::Polynomial, Some(order), SeriesType::Line)
    }
    
    /// Add an exponential regression dataset
    pub fn add_exponential_regression(self, series_label: &str, data: Vec<(X, Y)>) -> Self {
        self.add_regression_dataset(series_label, data, RegressionMethod::Exponential, None, SeriesType::Line)
    }
    
    /// Add a logarithmic regression dataset
    pub fn add_logarithmic_regression(self, series_label: &str, data: Vec<(X, Y)>) -> Self {
        self.add_regression_dataset(series_label, data, RegressionMethod::Logarithmic, None, SeriesType::Line)
    }
    
    /// Build final EChartsOption, populating legend
    pub fn build(self) -> EChartsOption {
        self.option
    }
}
