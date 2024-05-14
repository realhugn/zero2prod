use std::net::TcpListener;

use zero2prod::{
    configuration::{self, get_config},
    run,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_config().expect("Failed to read config");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
