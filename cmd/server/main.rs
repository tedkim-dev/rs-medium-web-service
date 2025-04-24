mod server;
use server::ServerState;

mod database;
use database::create_db_pool;

mod todos;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    // set up connection pool
    let db_pool = create_db_pool().await;

    let server_state = server::ServerState::new(db_pool);

    server::run_server(server_state).await?;

    Ok(())
}
