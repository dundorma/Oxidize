use oxidize::configuration::get_configuration;
use oxidize::startup::run;
use oxidize::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("oxidize".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("failed to read configuration.");
    let connection_pool = PgPoolOptions::new().connect_lazy_with(config.database.with_db());

    let address = format!("{}:{}", config.application.host, config.application.port);
    let listener = TcpListener::bind(address).expect("faield to bind to port 8008");

    run(listener, connection_pool)?.await
}
