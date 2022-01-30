// Launch our application in the background
pub fn spawn_app() -> () {
    // Launch the server as a background task
    let server = url_shortener::run_server().expect("Failed to bind address");
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    tokio::spawn(server);

    return;
}
