use crate::options::EChartsOption;
use sailfish::TemplateSimple;

#[derive(TemplateSimple)]
#[template(path = "script.stpl")]
pub struct ScriptTemplate{
    chart_target_id: String,
    options: EChartsOption
}

impl ScriptTemplate{
    pub fn new(chart_target_id: String, options: EChartsOption) -> Self {
        Self {
            chart_target_id,
            options,
        }
    }
}