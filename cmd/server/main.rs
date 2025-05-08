mod server;
use scheduler::start_scheduler;
use server::ServerState;

mod database;
use database::create_db_pool;
use tokio::{signal, sync::watch};
use worker::start_worker;

mod authentication;
mod errors;
mod router;
mod scheduler;
mod todos;
mod users;
mod worker;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    // set up connection pool
    let db_pool = create_db_pool().await;
    let jwt_base64 = std::env::var("JWT_SECRET").expect("JWT_BASE64 is not set");

    // start server
    let server_state = ServerState::new(db_pool.clone(), jwt_base64);

    let server_shutdown_rx = shutdown_rx.clone();
    let server_task = tokio::spawn(async move {
        server::run_server(server_state, server_shutdown_rx).await;
    });

    // start worker
    let worker_shutdown_rx = shutdown_rx.clone();
    let worker_db_pool = db_pool.clone();
    let worker_task = tokio::spawn(async move {
        start_worker(worker_db_pool, worker_shutdown_rx).await;
    });

    // start scheduler
    let scheduler_task = start_scheduler(db_pool.clone(), shutdown_rx).await;

    // Wait for a Ctrl+C signal
    signal::ctrl_c().await.unwrap();

    // Signal the scheduler and server to stop
    shutdown_tx.send(true).unwrap();

    // Wait for all tasks to complete
    println!("\nWaiting for all tasks to complete");
    _ = tokio::join!(server_task, worker_task, scheduler_task);

    println!("All tasks completed");
    Ok(())
}
