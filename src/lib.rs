use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

mod routes;

use routes::greet::greet;
use routes::health_check::health_check;

pub fn run_server() -> std::io::Result<Server> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    println!("Server started on: {}", listener.local_addr().unwrap());
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
