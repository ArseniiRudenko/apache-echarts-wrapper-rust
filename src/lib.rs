pub mod options;
mod builder;
pub mod templates;
pub mod common;

#[cfg(feature = "time_axis")]
pub mod time_axis;
mod axis_typing;

#[cfg(feature = "chrono_axis")]
mod chrono_axis;

pub use builder::{
    ChartBuilder,
    RegressionChartBuilder,
};

#[cfg(test)]
mod tests {
    use crate::builder::{ChartBuilder, RegressionChartBuilder};
    use crate::options::{EChartsOption, SeriesType};
    use crate::templates::OnePage;
    use sailfish::TemplateSimple;
    use std::net::TcpListener;
    use std::thread;
    use std::time::Duration;
    use time::Weekday;
    use time::Weekday::{Friday, Monday, Thursday, Tuesday};
    use tiny_http::{Header, Server};
    use crate::common::Size;

    #[test]
    #[cfg(feature = "time_axis")]
    fn it_works() {
        let chart = EChartsOption::<f64, &str>::new()
            .title_str("Something interesting")
            .add_series(
                SeriesType::Line,
                "fist_set",
                vec![(12.5,"First"),(14.0,"Second"),(15.0,"Third"),(10.0,"Fourth")]
            )
            .add_series(
                SeriesType::Line,
                "second_set",
                vec![(2.0,"First"),(14.0,"Third"),(15.0,"Third"),(20.0,"First")]
            )
            .build(Size::pixels(600),Size::pixels(400));

        let numeric_chart = EChartsOption::<f64, f64>::new()
            .title_str("Something completely different")
            .add_linear_regression_series(
                "regression set2",
                vec![
                    (1.0,1.0),
                    (1.0,2.0),
                    (3.5,3.0),
                    (4.0,4.0),
                    (4.1,1.0),
                    (4.1,3.0),
                    (5.0,4.0),
                    (14.0,3.0),
                    (15.0,1.0),
                    (20.0,1.0)
                ]
            )
            .add_linear_regression_series(
                "regression set",
                vec![
                    (1.0,11.0),
                    (1.0,20.0),
                    (3.5,30.0),
                    (4.0,40.0),
                    (4.1,11.0),
                    (4.1,35.0),
                    (5.0,40.0),
                    (14.0,33.0),
                    (15.0,31.0),
                    (20.0,11.0)
                ]
            ) .build(Size::pixels(600),Size::pixels(400));

        let chart_week = EChartsOption::<f64, Weekday>::new()
            .title_str("Something Dated")
            .add_series(
                SeriesType::Line,
                "fist_set",
                vec![(12.5,Monday),(14.0,Tuesday),(15.0,Thursday),(10.0,Friday)]
            )
            .add_series(
                SeriesType::Line,
                "second_set",
                vec![(2.0,Monday),(14.0,Thursday),(15.0,Thursday),(20.0,Monday)]
            )
            .build(Size::pixels(600),Size::pixels(400));


        let mut body = chart.render_once().unwrap();
        body.push_str(numeric_chart.render_once().unwrap().as_str());
        body.push_str(chart_week.render_once().unwrap().as_str());
        // Generate your HTML string here
        let html = OnePage::new("Test",body.as_str()).render_once().unwrap();

        // Start a minimal web server in a separate thread
        let listener = TcpListener::bind("127.0.0.1:0").unwrap(); // Bind to any free port
        let addr = listener.local_addr().unwrap();
        let server = Server::from_listener(listener,None).unwrap();

        // Optionally open in a browser
        let url = format!("http://{}", addr);
        println!("Serving at {}", url);
        let _ = open::that(&url); // Just ignore errors here

        thread::spawn(move || {
            for request in server.incoming_requests() {
                let response = tiny_http::Response::from_string(html.clone())
                    .with_header("Content-Type: text/html".parse::<Header>().unwrap());
                let _ = request.respond(response);
            }
        });

        // Keep the test alive a bit so you can view it
        thread::sleep(Duration::from_secs(5));
    }
}
