use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // log::info!("starting http server at port 8008");
    let server = HttpServer::new(|| {
        App::new()
            // .wrap(middleware::Logger::default())
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
