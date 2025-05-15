mod options;
mod builder;
mod templates;
mod common;

#[cfg(test)]
mod tests {
    use crate::builder::{ChartBuilder, ChartBuilderExt, RegressionChartBuilderExt};
    use crate::options::SeriesType;
    use crate::templates::OnePage;
    use sailfish::TemplateSimple;
    use std::net::TcpListener;
    use std::thread;
    use std::time::Duration;
    use tiny_http::{Header, Server};
    use crate::common::Size;

    #[test]
    fn it_works() {
        let chart = ChartBuilder::<f64, &str>::new()
            .title_str("Something interesting")
            .add_series(
                "fist_set",
                vec![(12.5,"First"),(14.0,"Second"),(15.0,"Third"),(10.0,"Fourth")]
                ,SeriesType::Line)
            .add_series(
                "second_set",
                vec![(2.0,"First"),(14.0,"Third"),(15.0,"Third"),(20.0,"First")]
                ,SeriesType::Line
            )
            .add_linear_regression_series(
                "third set",
                vec![(1.0,"First"),(1.0,"Second"),(3.5,"Third"),(4.0,"Fourth"),(4.1,"First"),(4.1,"Third"),(5.0,"Fourth"),(14.0,"Third"),(15.0,"Third"),(20.0,"First")]
                
            )
            .build(Size::pixels(600),Size::pixels(400));

        let body = chart.render_once().unwrap();
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
