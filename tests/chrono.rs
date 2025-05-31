mod common;
use chrono::Month::{August, February, January, September};
use chrono::TimeZone;
use sailfish::TemplateSimple;
use apache_echarts_wrapper::common::Size;
use apache_echarts_wrapper::EChartOptions;
use apache_echarts_wrapper::options::SeriesType;
use crate::common::show_page;

#[cfg(feature = "chrono_axis")]
#[test]
fn test_chrono() {
    let chart_month = EChartOptions::<chrono::Month,f64>::default()
        .title_str("Week test".to_string())
        .add_series(
            SeriesType::Bar,
            "lbl".to_string(),
            [
                (January, 10.0),
                (February, 12.0),
                (August,5.0),
                (September,300.0)
            ]
        ).build(Size::pixels(800),Size::pixels(600));
    
    let chart_datetime = EChartOptions::<chrono::DateTime<chrono::Utc>,f64>::default()
    .title_str("datetime test".to_string())
        .add_series(
            SeriesType::Line,
            "f".to_string(),
            [
                (chrono::Utc.with_ymd_and_hms(2021,1,1,0,0,0).unwrap(), 10.0),
                (chrono::Utc.with_ymd_and_hms(2021,1,2,0,0,0).unwrap(), 12.0),
                (chrono::Utc.with_ymd_and_hms(2021,1,3,0,0,0).unwrap(),5.0),
                (chrono::Utc.with_ymd_and_hms(2021,1,4,0,0,0).unwrap(),8.0)
            ]
        ).add_series(
        SeriesType::Bar,
        "lbl".to_string(),
        [
            (chrono::Utc.with_ymd_and_hms(2021,1,1,0,0,0).unwrap(), 10.0),
            (chrono::Utc.with_ymd_and_hms(2021,1,2,0,0,0).unwrap(), 12.0),
        ]
        ).build(Size::pixels(800),Size::pixels(600));   
    
    let mut body =  chart_month.render_once().unwrap();
    body.push_str(&chart_datetime.render_once().unwrap());
    show_page(&body);
}