pub mod models;
pub mod routes;

use dropshot::{ApiDescription, ConfigDropshot, ConfigLogging, ConfigLoggingLevel, HttpServer};
use std::net::{Ipv4Addr, SocketAddr};

use models::context::Context;

#[tokio::main]
async fn main() -> Result<(), String> {
    let port = std::env::var("PORT").expect("Must set PORT environment variable");
    let port = port.parse::<u16>().expect("Unable to parse port to u16");
    let redis_url = std::env::var("REDIS").expect("Must set REDIS environment variable");

    let address = SocketAddr::from((Ipv4Addr::LOCALHOST, port));

    let log = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Info,
    }
    .to_logger("bee-api")
    .map_err(|error| format!("failed to create logger: {}", error))?;

    let mut api = ApiDescription::new();
    api.register(routes::puzzle::create_puzzle).unwrap();
    api.register(routes::puzzle::read_puzzle).unwrap();
    api.register(routes::puzzle::update_puzzle).unwrap();
    api.register(routes::puzzle::delete_puzzle).unwrap();
    api.register(routes::puzzle::read_puzzles).unwrap();

    let openapi = api.openapi("Bee", "0.0.0");
    let context = Context::new(&redis_url, openapi).await;

    api.register(routes::openapi::read_openapi).unwrap();

    let mut server = HttpServer::new(
        &ConfigDropshot {
            bind_address: address,
            ..Default::default()
        },
        api,
        context,
        &log,
    )
    .map_err(|error| format!("failed to create server: {}", error))?;
    let server_task = server.run();

    server.wait_for_shutdown(server_task).await
}
