use crate::options::{EChartsOption};
use sailfish::TemplateSimple;
use serde::{Serialize};
use crate::axis_typing::AxisKindMarker;
use crate::common::Size;



#[derive(TemplateSimple)]
#[template(path = "chart.stpl")]
pub struct ScriptTemplate<X:AxisKindMarker,Y:AxisKindMarker>
where EChartsOption<X,Y>: Serialize{
    chart_target_id: String,
    width: Size,
    height: Size,
    options: EChartsOption<X,Y>
}

impl<X,Y> ScriptTemplate<X,Y>
where X: AxisKindMarker, Y: AxisKindMarker, EChartsOption<X,Y>: Serialize{

    pub fn new(chart_target_id: String, width: Size, height: Size, options: EChartsOption<X,Y>) -> Self {
        Self {
            chart_target_id,
            width,
            height,
            options
        }
    }
}


#[derive(TemplateSimple)]
#[template(path = "one_page_chart.stpl")]
pub struct OnePage<'a>{
   title: &'a str,
   body: &'a str
}

impl<'a> OnePage<'a> {
    pub fn new(title: &'a str, body: &'a str) -> Self {
        Self {
            title,
            body
        }
    }
}