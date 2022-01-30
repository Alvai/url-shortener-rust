use actix_web::{dev::Server, web, App, HttpServer};

mod routes;

use routes::greet::greet;
use routes::health_check::health_check;

pub fn run_server() -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8000))?
    .run();

    Ok(server)
}
