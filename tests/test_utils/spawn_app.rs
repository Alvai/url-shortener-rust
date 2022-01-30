use std::net::TcpListener;

// Launch our application in the background
pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let adress = listener.local_addr().unwrap();

    // Launch the server as a background task
    let server = url_shortener::run_server(listener).expect("Failed to bind address");
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    tokio::spawn(server);

    format!("http://{}", adress)
}
