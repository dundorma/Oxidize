use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run() -> Result<Server, std::io::Error> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // log::info!("starting http server at port 8008");
    let server = HttpServer::new(|| {
        App::new()
            // .wrap(middleware::Logger::default())
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8008")?
    .run();

    Ok(server)
}
