use crate::routes;

use actix_web::dev::Server;
use actix_web::{middleware, web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting http server at port 8008");
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
