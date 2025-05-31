use apache_echarts_wrapper::common::Size;
use apache_echarts_wrapper::options::{EChartOptions, SeriesType};
use sailfish::TemplateSimple;
use time::macros::{datetime, time};
use time::Weekday::{Friday, Monday, Thursday, Tuesday};


mod common;
use crate::common::show_page;
#[cfg(feature = "time_axis")]
#[test]
fn time_test() {
    let chart_week = EChartOptions::<f64, time::Weekday>::default()
        .title_str("Week test".to_string())
        .add_series(
            SeriesType::Line,
            "first_set".to_string(),
            vec![(12.5,Monday),(14.0,Tuesday),(15.0,Thursday),(10.0,Friday)]
        )
        .add_series(
            SeriesType::Line,
            "second_set".to_string(),
            vec![(2.0,Monday),(14.0,Thursday),(15.0,Thursday),(20.0,Monday)]
        )
        .build(Size::pixels(600),Size::pixels(400));
    let chart_date_time = EChartOptions::<f64, time::UtcDateTime>::default()
        .title_str("utc date time test".to_string())
        .add_series(
            SeriesType::Line,
            "fist_set".to_string(),
            vec![
                (12.5,time::UtcDateTime::now()),
                (14.0,datetime!(2025-01-01 12:00 UTC).into()),
                (15.0,datetime!(2024-01-01 12:00 UTC).into()),
                (10.0,datetime!(2023-01-01 12:00 UTC).into())
            ]
        )
        .add_series(
            SeriesType::Line,
            "second_set".to_string(),
            vec![
                (2.0,datetime!(2025-01-01 12:00 UTC).into()),
                (14.0,datetime!(2024-01-01 12:00 UTC).into()),
                (15.0,datetime!(2023-01-01 12:00 UTC).into()),
                (20.0,datetime!(2022-01-01 12:00 UTC).into())
            ]
        )
        .build(Size::pixels(600),Size::pixels(400));


    let chart_time_conf = EChartOptions::<f64, time::Time>::default()
        .title_str("Time test".to_string())
        .add_series(
            SeriesType::Line,
            "fist_set".to_string(),
            vec![
                (12.5,time!(12:00)),
                (14.0,time!(12:30)),
                (15.0,time!(14:00)),
                (10.0,time!(17:00))
            ]
        )
        .add_series(
            SeriesType::Line,
            "second_set".to_string(),
            vec![
                (2.0,time!(12:00)),
                (14.0,time!(10:00)),
                (15.0,time!(11:00)),
                (20.0,time!(00:00))
            ]
        );
    let ser = serde_json::to_string(&chart_time_conf).unwrap();
    println!("{}",ser);
    let chart_time =chart_time_conf.build(Size::pixels(600),Size::pixels(400));
    let mut body = chart_week.render_once().unwrap();
    body.push_str(&chart_date_time.render_once().unwrap());
    body.push_str(&chart_time.render_once().unwrap());
    show_page(body.as_str())
}