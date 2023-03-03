use std::net::TcpListener;

use env_logger::Env;
use sqlx::PgPool;
use tracing_subscriber::EnvFilter;

use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let env_filter = EnvFilter::from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
