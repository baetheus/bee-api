pub mod models;

use dropshot::{ApiDescription, ConfigDropshot, ConfigLogging, ConfigLoggingLevel, HttpServer};
use std::env;
use std::fs::File;
use std::io::prelude::*;

use models::context::Context;

#[tokio::main]
async fn main() -> Result<(), String> {
    let port = env::var("PORT").expect("Required env variable PORT was not found");
    let redis_url = env::var("REDIS_URL").expect("Required env variable REDIS_URL was not found");

    let config_dropshot = ConfigDropshot {
        bind_address: format!("127.0.0.1:{}", port).parse().unwrap(),
    };
    let context = Context::new(&redis_url).await;

    let log = ConfigLogging::StderrTerminal {
        level: ConfigLoggingLevel::Info,
    }
    .to_logger("bee-api")
    .map_err(|error| format!("failed to create logger: {}", error))?;

    let mut api = ApiDescription::new();
    api.register(routes::puzzle::get_puzzle_by_id).unwrap();

    let mut spec = vec![];
    api.print_openapi(
        &mut spec,
        &"Pet Shop",
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        &"",
    )
    .map_err(|e| e.to_string())?;

    let mut file = File::create("spec.json").expect("Could not create spec.json file");
    file.write_all(&spec).expect("Could not write to spec.json");

    let mut server = HttpServer::new(&config_dropshot, api, context, &log)
        .map_err(|error| format!("failed to create server: {}", error))?;
    let server_task = server.run();

    server.wait_for_shutdown(server_task).await
}
