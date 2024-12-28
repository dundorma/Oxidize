use std::net::TcpListener;

use oxidize::configuration::get_configuration;
use oxidize::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("failed to read configuration.");
    let address = format!("127.0.0.1{}", config.application_port);
    let listener = TcpListener::bind(address).expect("faield to bind to port 8008");
    run(listener)?.await
}
