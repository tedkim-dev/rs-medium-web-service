mod server;
use server::ServerState;

mod database;
use database::create_db_pool;
use worker::start_worker;

mod authentication;
mod router;
mod todos;
mod users;
mod errors;
mod worker;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    // set up connection pool
    let db_pool = create_db_pool().await;
    let jwt_base64 = std::env::var("JWT_SECRET")
        .expect("JWT_BASE64 is not set");

    // start worker
    start_worker(db_pool.clone()).await;

    // start server
    let server_state = ServerState::new(db_pool, jwt_base64);

    server::run_server(server_state).await?;

    Ok(())
}
