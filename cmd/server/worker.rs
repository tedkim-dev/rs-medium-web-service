use std::{sync::Arc, time::Duration};

use futures::StreamExt;
use queue::{postgres_queue::PostgresQueue, Job, Message, Queue};
use sqlx::PgPool;
use tokio::sync::watch;

const CONCURRENCY: usize = 50;

pub async fn start_worker(db_pool: PgPool, shutdown_rx: watch::Receiver<bool>) {
    let queue = Arc::new(PostgresQueue::new(db_pool));

    println!("worker: starting worker");
    // run worker
    let worker_queue = queue.clone(); // queue is an Arc pointer, so we only copy the reference

    run_worker(worker_queue, shutdown_rx).await;
}

async fn run_worker(queue: Arc<dyn Queue>, shutdown_rx: watch::Receiver<bool>) {
    while !shutdown_rx.has_changed().unwrap() {
        let jobs = match queue.pull(CONCURRENCY as u32).await {
            Ok(jobs) => jobs,
            Err(err) => {
                println!("run_worker: pulling jobs: {}", err);
                tokio::time::sleep(Duration::from_millis(500)).await;
                Vec::new()
            }
        };

        let numbers_of_jobs = jobs.len();
        if numbers_of_jobs > 0 {
            println!("Fetched {} jobs", numbers_of_jobs);
        }

        futures::stream::iter(jobs)
            .for_each_concurrent(CONCURRENCY, |job| async {
                let job_id = job.id;

                let res = match handle_job(job).await {
                    Ok(_) => queue.delete_job(job_id).await,
                    Err(err) => {
                        println!("run_worker: handling job({}): {}", job_id, &err);
                        queue.fail_job(job_id).await
                    }
                };

                match res {
                    Ok(_) => {}
                    Err(err) => {
                        println!("run_worker: deleting / failing job: {}", &err);
                    }
                }
            })
            .await;

        // sleep not to overload our database
        tokio::time::sleep(Duration::from_millis(125)).await;
    }
    println!("worker: received shutdown signal and exiting");
}

async fn handle_job(job: Job) -> Result<(), queue::Error> {
    match job.message {
        // when you seed queue with this message, it will send a sign in email to the user
        // message: {
        //   "SendSignInEmail": {
        //     "email": "user@example.com",
        //     "name": "John Doe",
        //     "code": "123456"
        //   }
        // }
        message @ Message::SendSignInEmail { .. } => {
            println!("Sending sign in email: {:?}", message);
        },
        _ => {
            println!("Unknown job: {:?}", &job);
        },
        // TODO: add logic to handle the job here
    }
    Ok(())
}
