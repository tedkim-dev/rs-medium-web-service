# rs-medium-web-service

A practical example of building scalable web services in Rust using modern practices and patterns.

## Project Structure

```
cmd/
└──server/
    ├── migrations/
    │   └──0001.sql
    ├── main.rs
    ├── server.rs
    ├── database.rs
    └── todos/
        ├── mod.rs
        ├── handler.rs
        └── routes.rs

services/
└── todos/
    ├── service.rs
    ├── model.rs
    └── repository.rs
```

### TODO
- add authentication
- add scheduler
- add worker

### Project Structure Reference
```
cmd/
    my-server/
        migrations/
            0001.sql
        Cargo.toml
        main.rs
        server.rs
        scheduler.rs
        worker.rs
        webapp.rs
libs/
    mailer/
        Cargo.toml
        mailer.rs
    queue/
        Cargo.toml
        queue.rs
    stripe/
        Cargo.toml
        stripe.rs
services/
    users/
        repository/
            users.rs
            sessions.rs
        service/
            get_user.rs
            get_session.rs
        Cargo.toml
        errors.rs
        model.rs
        repository.rs
        service.rs
Cargo.toml <- Cargo.toml for the workspace
Dockerfile.my-server
Makefile
README.md
```

#### References
- https://kerkour.com/rust-web-services-axum-sqlx-postgresql