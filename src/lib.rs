use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

mod routes;

use routes::greet::greet;
use routes::health_check::health_check;

pub fn run_server(tcp_listener: TcpListener) -> std::io::Result<Server> {
    let adress = tcp_listener.local_addr().unwrap();

    println!("Server started on: http://{}", adress);

    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}
