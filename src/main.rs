use std::net::TcpListener;

use oxidize::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8008").expect("faield to bind to port 8008");
    run(listener)?.await
}
