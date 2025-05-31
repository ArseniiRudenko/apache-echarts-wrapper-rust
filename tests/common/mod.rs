use std::net::TcpListener;
use std::thread;
use std::time::Duration;
use sailfish::TemplateSimple;
use tiny_http::{Header, Server};
use apache_echarts_wrapper::templates::OnePage;

pub fn show_page(body: &str) {
    // Generate your HTML string here
    let html = OnePage::new("Test",body).render_once().unwrap();

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