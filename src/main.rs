use oxidize::configuration::get_configuration;
use oxidize::startup::run;
use oxidize::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("oxidize".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration().expect("failed to read configuration.");
    let connection_pool = PgPool::connect(&config.database.connection_string().expose_secret())
        .await
        .expect("failed to connect to postgresql server.");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address).expect("faield to bind to port 8008");

    run(listener, connection_pool)?.await
}
