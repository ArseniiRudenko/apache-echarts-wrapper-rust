use crate::common::Size;
use crate::options::{Axis, AxisPointer, AxisPointerType, AxisType, DataItem, DataPointSymbol, DataValue, DatasetComponent, DatasetTransform, DatasetTransformType, EChartsOption, Position, PositionKeyword, RegressionConfig, RegressionMethod, Series, SeriesDataSource, SeriesType, Title, Tooltip, TooltipTrigger};
use crate::templates::ScriptTemplate;
use std::marker::PhantomData;
use uuid::Uuid;


pub trait TypedAxis{
    fn axis_type()-> AxisType;
}

pub trait ValueAxis :TypedAxis {
    fn axis_type() -> AxisType { AxisType::Value }
}

impl TypedAxis for i32 {
    fn axis_type() -> AxisType { AxisType::Value }
}

impl ValueAxis for i32 {}

impl TypedAxis for u32 {
    fn axis_type() -> AxisType { AxisType::Value }
}

impl ValueAxis for u32 {}

impl TypedAxis for i64 {
    fn axis_type() -> AxisType { AxisType::Value }
}

impl ValueAxis for i64 {}

impl TypedAxis for f64 {
    fn axis_type() -> AxisType { AxisType::Value }
}

impl ValueAxis for f64 {}

impl TypedAxis for usize {
    fn axis_type() -> AxisType { AxisType::Value }
}

impl ValueAxis for usize {}

impl TypedAxis for String {
    fn axis_type() -> AxisType { AxisType::Category }
}

impl<'a> TypedAxis for &'a str {
    fn axis_type() -> AxisType { AxisType::Category }
}



/// Trait to determine an axis type and convert into DataValue
pub trait AxisInfo {
  
    /// Convert a value into DataValue
    fn into_data_value(self) -> DataValue;
}

// Implement AxisInfo for common types
impl AxisInfo for i32 {
    fn into_data_value(self) -> DataValue { DataValue::Int(self as i64) }
}

impl AxisInfo for u32 {
    fn into_data_value(self) -> DataValue { DataValue::Int(self as i64) }
}

impl AxisInfo for u16 {
    fn into_data_value(self) -> DataValue { DataValue::Int(self as i64) }
}

impl AxisInfo for i64 {
    fn into_data_value(self) -> DataValue { DataValue::Int(self) }
}
impl AxisInfo for f64 {
    fn into_data_value(self) -> DataValue { DataValue::Float(self) }
}

impl AxisInfo for f32 {
    fn into_data_value(self) -> DataValue { DataValue::Float(self as f64) }
}

impl AxisInfo for usize{
    fn into_data_value(self) -> DataValue { DataValue::Int(self as i64) }
}

impl AxisInfo for String {
    fn into_data_value(self) -> DataValue { DataValue::String(self) }
}

impl<'a> AxisInfo for &'a str {
    fn into_data_value(self) -> DataValue { DataValue::String(self.to_string()) }
}


/// Builder for multi-line charts, inferring axis types from X and Y
pub struct ChartBuilder<X: AxisInfo, Y: AxisInfo>
where X: TypedAxis, Y: TypedAxis{
    option: EChartsOption,
    _marker: PhantomData<(X, Y)>,
}


pub trait RegressionChartBuilderExt<X: AxisInfo, Y: AxisInfo>: ChartBuilderExt<X, Y>
where X: ValueAxis, Y: ValueAxis {
    
    fn add_linear_regression_dataset(self, data_source_index: usize) -> usize {
        self.add_regression_dataset(data_source_index, RegressionMethod::Linear, None)
    }

    fn add_polynomial_regression_dataset(self, data_source_index: usize, order: u32) -> usize {
        self.add_regression_dataset(data_source_index, RegressionMethod::Polynomial, Some(order))
    }

    fn add_exponential_regression_dataset(self, data_source_index: usize) -> usize {
        self.add_regression_dataset(data_source_index, RegressionMethod::Exponential, None)
    }

    fn add_regression_dataset(mut self, data_source_index: usize, method: RegressionMethod, order: Option<u32>) -> usize {
        let index = self.option().dataset.as_mut().unwrap().len();
        let regression_config = RegressionConfig {
            method: method.clone(),
            order,
            extra: None,
        };
        let dataset = DatasetTransform{
            r#type: DatasetTransformType::Regression,
            config: Some(regression_config),
            extra: None,
        };
        self.option().dataset.as_mut().unwrap().push(DatasetComponent::tr(dataset, data_source_index));
        index
    }


    /// Add a dataset with regression transformation
    fn add_regression_series(mut self, series_label: &str, data: Vec<(X, Y)>,
                             method: RegressionMethod, order: Option<u32>) -> Self {
        // Create a dataset vector if it doesn't exist
        if self.option().dataset.is_none() {
            self.option().dataset = Some(Vec::new());
        }

        // Convert the data to a format suitable for ECharts
        let raw_data = data.into_iter()
            .map(|(x, y)| [x.into_data_value(), y.into_data_value()])
            .collect::<Vec<_>>();

        // Add source dataset
        let datasets = self.option().dataset.as_mut().unwrap();
        let source_index = datasets.len();
        datasets.push(DatasetComponent::src(raw_data));


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


        datasets.push(DatasetComponent::tr(
            DatasetTransform {
                r#type: DatasetTransformType::Regression,
                config: Some(regression_config),
                extra: None
            },
            source_index
        )
        );

        // Add scatter series for original data
        self.option().series.as_mut().unwrap().push(Series {
            r#type: Some(SeriesType::Scatter),
            name: Some(format!("{} (data)", series_label)),
            smooth: None,
            area_style: None,
            data: SeriesDataSource::DatasetIndex(source_index),
            symbol: Some(DataPointSymbol::Circle),
            symbol_size: Some(15),
            extra: None,
        });

        // Add line series for regression
        self.option().series.as_mut().unwrap().push(Series {
            r#type: Some(SeriesType::Line),
            name: Some(format!("{} (regression)", series_label)),
            smooth: Some(true),
            area_style: None,
            data: SeriesDataSource::DatasetIndex(transform_index),
            symbol: Some(DataPointSymbol::None),
            symbol_size: None,
            extra: None
        });

        self
    }

    /// Add a linear regression dataset
    fn add_linear_regression_series(self, series_label: &str, data: Vec<(X, Y)>) -> Self {
        self.add_regression_series(series_label, data, RegressionMethod::Linear, None)
    }

    /// Add a polynomial regression dataset
    fn add_polynomial_regression_series(self, series_label: &str, data: Vec<(X, Y)>, order: u32) -> Self {
        self.add_regression_series(series_label, data, RegressionMethod::Polynomial, Some(order))
    }

    /// Add an exponential regression dataset
    fn add_exponential_regression_series(self, series_label: &str, data: Vec<(X, Y)>) -> Self {
        self.add_regression_series(series_label, data, RegressionMethod::Exponential, None)
    }

    /// Add a logarithmic regression dataset
    fn add_logarithmic_regression_series(self, series_label: &str, data: Vec<(X, Y)>) -> Self {
        self.add_regression_series(series_label, data, RegressionMethod::Logarithmic, None)
    }
    
    
}


pub trait ChartBuilderExt<X: AxisInfo, Y: AxisInfo>: Sized
where X: TypedAxis, Y: TypedAxis{

    fn option(&mut self) -> &mut EChartsOption;
    
    /// Set chart title
    fn title_str(mut self, title: &str) -> Self {
        let t =  self.option().title.get_or_insert_default();
        t.left = Some(Position::Keyword(PositionKeyword::Center));
        t.text = Some(title.to_string());
        self
    }

    fn subtitle_str(mut self, subtitle: &str) -> Self {
        self.option().title.get_or_insert_default().sub_text = Some(subtitle.to_string());
        self
    }

    fn title(mut self, title: Title) -> Self {
        self.option().title = Some(title);
        self
    }


    fn x_axis_label(mut self, x: &str) -> Self {
        self.option().x_axis.name = Some(x.to_string());
        self
    }

    fn y_axis_label(mut self, y: &str) -> Self {
        self.option().y_axis.name = Some(y.to_string());
        self
    }

    //add a dataset and get an index
    fn add_dataset(mut self, data: Vec<(X, Y)>) -> usize {
        let index = self.option().dataset.as_mut().unwrap().len();
        self.option().dataset.as_mut().unwrap().push(DatasetComponent::src(
            data.into_iter()
                .map(|(x, y)| [x.into_data_value(), y.into_data_value()])
                .collect::<Vec<_>>()
        ));
        index
    }

    fn add_dataset_visualisation(mut self, series_label:&str,series_type: SeriesType, dataset_index: usize) -> Self {
        self.option().series.as_mut().unwrap().push(Series {
            r#type: Some(series_type),
            name: Some(format!("{}", series_label)),
            smooth: Some(true),
            area_style: None,
            symbol: None,
            symbol_size: None,
            data: SeriesDataSource::DatasetIndex(dataset_index),
            extra: None
        });
        self
    }

    /// Add a series; X and Y conversions ensure homogeneous types
    fn add_series(mut self, series_label:&str, data: Vec<(X, Y)>, series_type: SeriesType) -> Self {
        let data_items: Vec<DataItem> =data.into_iter()
            .map(|(x, y)| DataItem::Pair([x.into_data_value(), y.into_data_value()]))
            .collect();
        self.option().series.as_mut().unwrap().push(
            Series::new(series_label,series_type,SeriesDataSource::Data(data_items))
        );
        self
    }

    fn build(self, width: Size, height: Size) -> ScriptTemplate;
}



impl<X: AxisInfo + TypedAxis, Y: AxisInfo+TypedAxis>  ChartBuilder<X,Y> {

    pub fn new() -> Self {
        let opt = EChartsOption {
            title: None,
            tooltip: Some(Tooltip {
                show: true,
                show_delay: None,
                hide_delay: None,
                trigger: Some(TooltipTrigger::Item),
                formatter: None,
                axis_pointer: Some(AxisPointer {
                    r#type: Some(AxisPointerType::Cross),
                    snap: Some(false),
                    animation: None,
                    axis: None,
                })
            }),
            legend: None, grid: None, extra: None, dataset: None,
            x_axis: Axis { r#type: Some(X::axis_type()), name: None, data: None, extra: None },
            y_axis: Axis { r#type: Some(Y::axis_type()), name: None, data: None, extra: None },
            series: Some(Vec::new()),
        };
        Self { option: opt, _marker: Default::default() }
    }
    
}


impl<X: AxisInfo + TypedAxis, Y: AxisInfo+TypedAxis> ChartBuilderExt<X, Y> for  ChartBuilder<X, Y> {
    fn option(&mut self) -> &mut EChartsOption {
        &mut self.option
    }

    fn build(self, width: Size, height: Size) -> ScriptTemplate{
        ScriptTemplate::new(Uuid::new_v4().to_string(), width, height, self.option)
    }
}


impl <X: AxisInfo + ValueAxis + TypedAxis, Y: AxisInfo + ValueAxis + TypedAxis> RegressionChartBuilderExt<X, Y> for ChartBuilder<X, Y> {}
