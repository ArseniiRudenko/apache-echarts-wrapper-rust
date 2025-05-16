use crate::common::Size;
use crate::options::{Axis, AxisPointer, AxisPointerType, AxisType, DataPointSymbol, DatasetComponent, DatasetTransform, DatasetTransformType, EChartsOption, Position, PositionKeyword, RegressionConfig, RegressionMethod, Series, SeriesDataSource, SeriesType, Title, Tooltip, TooltipTrigger};
use crate::templates::ScriptTemplate;
use std::marker::PhantomData;
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;


pub trait AxisInfo {
    fn axis_type() -> AxisType;
    
}

///Trait that captures types that have AxisType as Value on the type level.
///That will catch and prevent user from trying to create regression charts on category data
///(which is not supported but will not cause an error, instead the chart just won't render)
pub trait ValueAxis : AxisInfo {}

impl AxisInfo for u128 {
    fn axis_type() -> AxisType { AxisType::Value }
}

impl ValueAxis for u128 {}

impl AxisInfo for i128 {

    fn axis_type() -> AxisType { AxisType::Value }
}

impl ValueAxis for i128 {}


impl AxisInfo for i32 {
    fn axis_type() -> AxisType { AxisType::Value }
}

impl ValueAxis for i32 {}

impl AxisInfo for u32 {
    fn axis_type() -> AxisType { AxisType::Value }
}

impl ValueAxis for u32 {}

impl AxisInfo for i64 {
    fn axis_type() -> AxisType { AxisType::Value }
}

impl ValueAxis for i64 {}

impl AxisInfo for u64 {
    fn axis_type() -> AxisType { AxisType::Value }
}

impl ValueAxis for u64 {}

impl AxisInfo for i16 {
    fn axis_type() -> AxisType { AxisType::Value }
}

impl ValueAxis for i16 {}

impl AxisInfo for u16 {
    fn axis_type() -> AxisType { AxisType::Value }
}

impl ValueAxis for u16 {}

impl AxisInfo for i8 {
    fn axis_type() -> AxisType { AxisType::Value }
}

impl ValueAxis for i8 {}

impl AxisInfo for u8 {
    fn axis_type() -> AxisType { AxisType::Value }
}

impl ValueAxis for u8 {}

impl AxisInfo for f32 {
    fn axis_type() -> AxisType { AxisType::Value }
}

impl ValueAxis for f32 {}

impl AxisInfo for f64 {
    fn axis_type() -> AxisType { AxisType::Value }
}

impl ValueAxis for f64 {}

impl AxisInfo for usize {
    fn axis_type() -> AxisType { AxisType::Value }
}

impl ValueAxis for usize {}

impl AxisInfo for isize {
    fn axis_type() -> AxisType { AxisType::Value }
}

impl ValueAxis for isize {}

impl AxisInfo for String {
    fn axis_type() -> AxisType { AxisType::Category }
}

impl<'a> AxisInfo for &'a str {
    fn axis_type() -> AxisType { AxisType::Category }
}


///trait that provides regression methods that are only supported when both x and y are numeric
pub trait RegressionChartBuilder<X, Y>: ChartBuilder<X, Y>
where X: AxisInfo + ValueAxis + Serialize,
      Y: AxisInfo + ValueAxis + Serialize
{
    
    fn add_linear_regression_dataset(self, data_source_index: usize) -> usize {
        self.add_regression_dataset(data_source_index, RegressionMethod::Linear, None)
    }

    fn add_polynomial_regression_dataset(self, data_source_index: usize, order: u8) -> usize {
        self.add_regression_dataset(data_source_index, RegressionMethod::Polynomial, Some(order))
    }

    fn add_exponential_regression_dataset(self, data_source_index: usize) -> usize {
        self.add_regression_dataset(data_source_index, RegressionMethod::Exponential, None)
    }

    fn add_regression_dataset(mut self, data_source_index: usize, method: RegressionMethod, order: Option<u8>) -> usize {
        let index = self.options().dataset.as_mut().unwrap().len();
        let regression_config = RegressionConfig {
            method: method.clone(),
            order,
            extra: None,
        };
        let dataset = DatasetTransform::regression(regression_config);
        self.options().dataset.as_mut().unwrap().push(DatasetComponent::tr(dataset, data_source_index));
        index
    }


    /// Add a dataset with regression transformation
    fn add_regression_series<TData: Into<DatasetComponent<X,Y>>>(mut self, series_label: &str, data: TData,
                             method: RegressionMethod, order: Option<u8>) -> Self {
        // Create a dataset vector if it doesn't exist
        if self.options().dataset.is_none() {
            self.options().dataset = Some(Vec::new());
        }

        // Add source dataset
        let datasets = self.options().dataset.as_mut().unwrap();
        let source_index = datasets.len();
        datasets.push(data.into());


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

        datasets.push(
            DatasetComponent::tr(
            DatasetTransform::regression(regression_config),
            source_index
        )
        );

        // Add scatter series for original data
        self.options().series.as_mut().unwrap().push(Series {
            r#type: Some(SeriesType::Scatter),
            name: Some(format!("{} (data)", series_label)),
            smooth: None,
            area_style: None,
            data: SeriesDataSource::DatasetIndex(source_index),
            symbol: Some(DataPointSymbol::Circle),
            symbol_size: Some(8),
            extra: None,
        });

        // Add line series for regression
        self.options().series.as_mut().unwrap().push(Series {
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
    fn add_linear_regression_series<TData:Into<DatasetComponent<X,Y>>>(self, series_label: &str, data: TData) -> Self {
        self.add_regression_series(series_label, data, RegressionMethod::Linear, None)
    }

    /// Add a polynomial regression dataset
    fn add_polynomial_regression_series<TData:Into<DatasetComponent<X,Y>>>(self, series_label: &str, data: TData, order: u8) -> Self {
        self.add_regression_series(series_label, data, RegressionMethod::Polynomial, Some(order))
    }

    /// Add an exponential regression dataset
    fn add_exponential_regression_series<TData:Into<DatasetComponent<X,Y>>>(self, series_label: &str, data: TData) -> Self
    {
        self.add_regression_series(series_label, data, RegressionMethod::Exponential, None)
    }

    /// Add a logarithmic regression dataset
    fn add_logarithmic_regression_series<TData:Into<DatasetComponent<X,Y>>>(self, series_label: &str, data: TData) -> Self {
        self.add_regression_series(series_label, data, RegressionMethod::Logarithmic, None)
    }
}


pub trait ChartBuilder<X, Y>: Sized
where X: AxisInfo+Serialize, Y: AxisInfo+Serialize
{

    fn options(&mut self) -> &mut EChartsOption<X,Y>;
    
    /// Set chart title
    fn title_str(mut self, title: &str) -> Self {
        let t =  self.options().title.get_or_insert_default();
        t.left = Some(Position::Keyword(PositionKeyword::Center));
        t.text = Some(title.to_string());
        self
    }

    fn subtitle_str(mut self, subtitle: &str) -> Self {
        self.options().title.get_or_insert_default().sub_text = Some(subtitle.to_string());
        self
    }

    fn title(mut self, title: Title) -> Self {
        self.options().title = Some(title);
        self
    }


    fn x_axis_label(mut self, x: &str) -> Self {
        self.options().x_axis.name = Some(x.to_string());
        self
    }

    fn y_axis_label(mut self, y: &str) -> Self {
        self.options().y_axis.name = Some(y.to_string());
        self
    }

    //add a dataset and get an index
    fn add_dataset<TData:Into<DatasetComponent<X,Y>>>(mut self, data: TData) -> usize {
        let index = self.options().dataset.as_mut().unwrap().len();
        self.options().dataset.as_mut().unwrap().push(data.into());
        index
    }

    /// Add visualization for a dataset.
    /// If no datasets exist, or dataset_index is out of range, no datasets will be added
    fn add_dataset_visualisation(mut self, series_label:&str, series_type: SeriesType, dataset_index: usize) -> Self {
        let datasets = &self.options().dataset;
         if let Some(datasets) = datasets {
           if let Some(dataset) =  datasets.get(dataset_index){
               match dataset {
                   DatasetComponent::Source(_) | DatasetComponent::Transform(_) => {
                       self.options().series.as_mut().unwrap().push(Series {
                           r#type: Some(series_type),
                           name: Some(format!("{}", series_label)),
                           smooth: Some(true),
                           area_style: None,
                           symbol: None,
                           symbol_size: None,
                           data: SeriesDataSource::DatasetIndex(dataset_index),
                           extra: None
                       });
                   }
                   DatasetComponent::LabelledSource(_) => {
                       self.options().series.as_mut().unwrap().push(Series {
                           r#type: Some(series_type),
                           name: Some(format!("{}", series_label)),
                           smooth: Some(true),
                           area_style: None,
                           symbol: None,
                           symbol_size: None,
                           data: SeriesDataSource::DatasetIndex(dataset_index),
                           extra: Some(json!(
                               {"encode": {"tooltip": [2,1], "x": 0, "y": 1 }}
                           ))
                       });
                   }
               }

           }
        }
        self
    }


    fn add_series<TData:Into<SeriesDataSource<X,Y>>>(mut self, series_label:&str, data: TData, series_type: SeriesType) -> Self {
        self.options().series.as_mut().unwrap().push(
            Series::new(series_label,series_type,data.into())
        );
        self
    }



    fn build(self, width: Size, height: Size) -> ScriptTemplate<X,Y>;
}



impl<X, Y>  EChartsOption<X,Y>
where X: AxisInfo, Y: AxisInfo {

    pub fn new() -> Self {
        Self {
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
        }
    }
}


impl<X, Y> ChartBuilder<X, Y> for  EChartsOption<X, Y>
where X: AxisInfo+Serialize , Y: AxisInfo+Serialize {
    fn options(&mut self) -> &mut EChartsOption<X,Y> {
        self
    }

    fn build(self, width: Size, height: Size) -> ScriptTemplate<X,Y>{
        ScriptTemplate::new(Uuid::new_v4().to_string(), width, height, self)
    }
}


impl <X, Y> RegressionChartBuilder<X, Y> for EChartsOption<X, Y>
where X: ValueAxis + AxisInfo+Serialize, Y: ValueAxis + AxisInfo+Serialize{}
