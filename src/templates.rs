use crate::options::{EChartsOption};
use sailfish::TemplateSimple;
use crate::common::Size;



#[derive(TemplateSimple)]
#[template(path = "chart.stpl")]
pub struct ScriptTemplate{
    chart_target_id: String,
    width: Size,
    height: Size,
    options: EChartsOption
}

impl ScriptTemplate{
    pub fn new(chart_target_id: String, width: Size, height: Size, options: EChartsOption) -> Self {
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