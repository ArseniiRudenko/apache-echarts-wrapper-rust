use crate::axis_typing::{AxisKindMarker, ValueAxis};
use crate::common::Size;
use crate::options::*;
use crate::templates::ScriptTemplate;
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;
use crate::common;
use crate::options::Position::Percent;

///trait that provides regression methods that are only supported when both x and y are numeric
impl<X, Y>  EChartOptions<X,Y>
where X: AxisKindMarker<AxisType=ValueAxis>+ Serialize,
      Y: AxisKindMarker<AxisType=ValueAxis>+ Serialize,
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
        let index = self.dataset.as_mut().unwrap().len();
        let regression_config = RegressionConfig {
            method: method.clone(),
            order,
            extra: None,
        };
        let dataset = DatasetTransform::regression(regression_config);
        self.dataset.as_mut().unwrap().push(DatasetComponent::tr(dataset, data_source_index));
        index
    }


    /// Add a dataset with regression transformation
    fn add_regression_series<TData: Into<DatasetComponent<X,Y>>>(mut self, series_label: &str, data: TData,
                             method: RegressionMethod, order: Option<u8>) -> Self {
        // Create a dataset vector if it doesn't exist
        if self.dataset.is_none() {
            self.dataset = Some(Vec::new());
        }

        // Add source dataset
        let datasets = self.dataset.as_mut().unwrap();
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
        self.series.as_mut().unwrap().push(Series {
            r#type: Some(SeriesType::Scatter),
            name: Some(format!("{} (data)", series_label)),
            smooth: None,
            area_style: None,
            data: SeriesDataSource::DatasetIndex(source_index),
            show_symbol: None,
            symbol: Some(DataPointSymbol::Circle),
            symbol_size: Some(8),
            extra: None,
        });

        // Add line series for regression
        self.series.as_mut().unwrap().push(Series {
            r#type: Some(SeriesType::Line),
            name: Some(format!("{} (regression)", series_label)),
            smooth: Some(true),
            area_style: None,
            data: SeriesDataSource::DatasetIndex(transform_index),
            show_symbol: None,
            symbol: Some(DataPointSymbol::None),
            symbol_size: None,
            extra: None
        });

        self
    }

    /// Add a linear regression dataset
    pub fn add_linear_regression_series<TData:Into<DatasetComponent<X,Y>>>(self, series_label: &str, data: TData) -> Self {
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


impl<X, Y> Default for EChartOptions<X,Y>
where X: AxisKindMarker, Y: AxisKindMarker, EChartOptions<X,Y>:Serialize{
    fn default() -> Self {
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
            x_axis: Axis::default(),
            y_axis: Axis::default(),
            series: Some(Vec::new()),
        }
    }
}

impl<X, Y>  EChartOptions<X,Y>
where X: AxisKindMarker, Y: AxisKindMarker, EChartOptions<X,Y>:Serialize {
    pub fn new(x_axis:Axis<X>,y_axis: Axis<Y>) -> Self {
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
            x_axis,
            y_axis,
            series: Some(Vec::new()),
        }
    }
    
    
    pub fn enable_legend(mut self) -> Self{
        if self.legend.is_none(){
            self.legend = Some(Legend{
                data: None,
                orient: Some(LegendOrient::Vertical),
                left: None,
                right: Some(Percent(common::Percent(0.0))),
                top: Some(Percent(common::Percent(20.0))),
                bottom: Some(Percent(common::Percent(20.0)))
            })
        }else {
            let mut legend = self.legend.unwrap();
            legend.right = Some(Percent(common::Percent(0.0)));
            legend.top = Some(Percent(common::Percent(20.0)));
            legend.bottom = Some(Percent(common::Percent(20.0)));
            legend.orient = Some(LegendOrient::Vertical);
            self.legend = Some(legend);
        }
        if self.grid.is_none(){
            self.grid = Some( Grid{
                left: None,
                right: Some(Percent(common::Percent(20.0))),
                top: None,
                bottom: None,
                contain_label: None,
                background_color: None,
                border_color: None,
                border_width: None,
                show: None,
                z: None,
                zlevel: None,
                extra: None,
            })
        }else {
            let mut grid = self.grid.unwrap();
            grid.right = Some(Percent(common::Percent(20.0)));
            self.grid = Some(grid);
        }
        self
    }

    /// Set chart title
    pub fn title_str(mut self, title: String) -> Self {
        let t =  self.title.get_or_insert_default();
        t.left = Some(Position::Keyword(PositionKeyword::Center));
        t.text = Some(title);
        self
    }

    pub  fn subtitle_str(mut self, subtitle: String) -> Self {
        self.title.get_or_insert_default().sub_text = Some(subtitle);
        self
    }

    pub  fn title(mut self, title: Title) -> Self {
        self.title = Some(title);
        self
    }


    pub  fn x_axis_label(mut self, x: String) -> Self {
        self.x_axis.name = Some(x);
        self
    }

    pub fn y_axis_label(mut self, y: String) -> Self {
        self.y_axis.name = Some(y);
        self
    }

    //add a dataset and get an index
    pub fn add_dataset<TData:Into<DatasetComponent<X,Y>>>(mut self, data: TData) -> usize {
        let index = self.dataset.as_mut().unwrap().len();
        self.dataset.as_mut().unwrap().push(data.into());
        index
    }

    /// Add visualization for a dataset.
    /// If no datasets exist, or dataset_index is out of range, no datasets will be added
    pub fn add_dataset_visualisation(mut self, series_label:String, series_type: SeriesType, dataset_index: usize) -> Self {
        let datasets = &self.dataset;
        if let Some(datasets) = datasets {
            if let Some(dataset) =  datasets.get(dataset_index){
                match dataset {
                    DatasetComponent::Source(_) | DatasetComponent::Transform(_) => {
                        self.series.as_mut().unwrap().push(Series {
                            r#type: Some(series_type),
                            name: Some(series_label),
                            smooth: Some(true),
                            area_style: None,
                            symbol: None,
                            symbol_size: None,
                            data: SeriesDataSource::DatasetIndex(dataset_index),
                            extra: None,
                            show_symbol: None,
                        });
                    }
                    DatasetComponent::LabelledSource(_) => {
                        self.series.as_mut().unwrap().push(Series {
                            r#type: Some(series_type),
                            name: Some(series_label),
                            smooth: Some(true),
                            area_style: None,
                            symbol: None,
                            symbol_size: None,
                            data: SeriesDataSource::DatasetIndex(dataset_index),
                            extra: Some(json!(
                               {"encode": {"tooltip": [2,1], "x": 0, "y": 1 }}
                           )),
                            show_symbol: None,
                        });
                    }
                }

            }
        }
        self
    }

    pub fn add_series_direct(mut self, series:Series<X,Y>) -> Self {
        self.series.as_mut().unwrap().push(series);
        self
    }



    pub fn add_series<TData:Into<SeriesDataSource<X,Y>>>(mut self, series_type: SeriesType, series_label:String, data: TData) -> Self {
        self.series.as_mut().unwrap().push(
            Series::new(series_label,series_type,data.into())
        );
        self
    }

    pub fn build(self, width: Size, height: Size) -> ScriptTemplate<X,Y>{
        ScriptTemplate::new(Uuid::new_v4().to_string(), width, height, self)
    }
}
