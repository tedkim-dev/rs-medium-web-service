use std::str::FromStr;

use cron::Schedule;
use sqlx::{pool::PoolConnection, PgPool, Postgres};
use tokio::{sync::watch, time::timeout};

async fn try_to_become_leader(
    db: PgPool,
    advisory_lock_id: i64,
    timeout_duration: std::time::Duration,
) -> Result<Option<PoolConnection<Postgres>>, SchedulerError> {
    let mut connection = db.acquire().await.map_err(SchedulerError::Connection)?;

    let is_leader = match timeout(
        timeout_duration,
        sqlx::query_scalar("SELECT pg_try_advisory_lock($1)")
            .bind(advisory_lock_id)
            .fetch_one(&mut *connection),
    )
    .await
    {
        Ok(Ok(is_leader)) => is_leader,
        Ok(Err(err)) => {
            eprintln!("scheduler: error querying advisory lock: {}", err);
            return Err(SchedulerError::AdvisoryLock(err));
        }
        Err(_) => {
            eprintln!("scheduler: timeout while querying advisory lock");
            return Err(SchedulerError::Timeout);
        }
    };

    if is_leader {
        Ok(Some(connection))
    } else {
        Ok(None)
    }
}

#[derive(Debug, thiserror::Error)]
enum SchedulerError {
    #[error("scheduler: db advisory lock error: {0}")]
    AdvisoryLock(sqlx::Error),
    #[error("scheduler: timeout error")]
    Timeout,
    #[error("scheduler: db connection error: {0}")]
    Connection(sqlx::Error),
}

async fn run_scheduler(db: PgPool, mut shutdown_rx: watch::Receiver<bool>) {
    let advisory_lock_id = 1;
    let timeout_duration = std::time::Duration::from_secs(10);
    let leader_connection = tokio::select! {
        _ = shutdown_rx.changed() => {
            println!("scheduler: received shutdown signal");
            return;
        },
        leader_conn = try_to_become_leader(db.clone(), advisory_lock_id, timeout_duration) => {
            match leader_conn {
                Ok(leader_conn) => leader_conn,
                Err(err) => {
                    eprintln!("scheduler: error: {}", err);
                    return;
                }
            }
        }
    };

    if leader_connection.is_some() {
        eprintln!("scheduler: this instance became leader");
    } else {
        eprintln!("scheduler: this instance did not become leader");
        return;
    }

    // add new cron jobs here
    // https://www.twilio.com/en-us/blog/run-cron-jobs-rust
    let mut cron_jobs = vec![];

    let cron_job_hello = tokio::spawn(run_hello(shutdown_rx.clone()));
    cron_jobs.push(cron_job_hello);

    let cron_job_hello_from_db = tokio::spawn({
        let schedule = Schedule::from_str("10 * * * * *").expect("failed to parse CRON expression");
        let mut shutdown_rx = shutdown_rx.clone();
        let db = db.clone();
        async move {
            loop {
                if let Some(next) = schedule.upcoming(chrono::Utc).take(1).next() {
                    let now = chrono::Utc::now();
                    let until_next = next - now;
                    tokio::select! {
                        _ = shutdown_rx.changed() => {
                            break;
                        }
                        _ = tokio::time::sleep(until_next.to_std().unwrap()) => {
                            let row = sqlx::query("SELECT now();").fetch_one(&db).await.unwrap();
                            println!("{:?} from another cron job", row);
                        }
                    }
                }
            }
        }
    });
    cron_jobs.push(cron_job_hello_from_db);

    println!("scheduler: cron jobs started");

    tokio::select! {
        _ = shutdown_rx.changed() => {
            println!("scheduler: received shutdown signal");
            for handle in cron_jobs {
                handle.await.unwrap();
            }
        }
    }
}

async fn run_hello(mut shutdown_rx: watch::Receiver<bool>) {
    let schedule = Schedule::from_str("5 * * * * *").expect("failed to parse CRON expression");

    loop {
        if let Some(next) = schedule.upcoming(chrono::Utc).take(1).next() {
            let now = chrono::Utc::now();
            let until_next = next - now;
            tokio::select! {
                _ = shutdown_rx.changed() => {
                    break;
                }
                _ = tokio::time::sleep(until_next.to_std().unwrap()) => {
                    println!("Hello, cron job from scheduler!");
                }
            }
        }
    }
}

pub async fn start_scheduler(db_pool: PgPool, shutdown_rx: watch::Receiver<bool>) -> tokio::task::JoinHandle<()> {
    println!("scheduler: starting scheduler");
    tokio::spawn(async move {
        run_scheduler(db_pool, shutdown_rx).await;
    })
}
