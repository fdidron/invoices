//!!! The main entry point for the application
//!!! The following environment variables must be set:
//!!! - DATABASE_URL - The URL to the Postgres database
//!!! - PORT - The port to bind the server to (defaults to 8080 if not set)
//!!! - SMTP_HOST - The SMTP host to use for sending emails
//!!! - SMTP_USERNAME - The SMTP username to use for sending emails
//!!! - SMTP_PASSWORD - The SMTP password to use for sending emails
//!!! - SMTP_PORT - The SMTP port to use for sending emails
//!!! - SMTP_FROM - The email address to send emails from
pub mod handlers;
pub mod markup;
pub mod models;

use std::sync::Arc;

use axum_session::{SessionConfig, SessionLayer, SessionStore};
use axum_session_sqlx::SessionPgPool;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::net::TcpListener;

pub struct AppState {
    pub db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    // Connect to the database and run pending migrations
    println! {"⌛ Invoice App v{} is starting ...", env!("CARGO_PKG_VERSION")};
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("❌ DATABASE_URL env var must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connected to database");
            pool
        }
        Err(e) => {
            println!("❌ Failed to connect to the database: {}", e);
            std::process::exit(1);
        }
    };

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("❌ Failed to run database migrations");

    // Create the session layer, linked to the database
    let session_config = SessionConfig::default()
        .with_secure(true)
        .with_table_name("sessions");

    let session_store =
        SessionStore::<SessionPgPool>::new(Some(pool.clone().into()), session_config)
            .await
            .expect("❌ Failed to create session store");

    let app = handlers::create_router(Arc::new(AppState { db: pool })).layer(SessionLayer::new(session_store));

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    println!("🚀 Server started successfully at 0.0.0.0:{}", port);

    let listener = match TcpListener::bind("0.0.0.0:8080").await {
        Ok(listener) => listener,
        Err(e) => {
            println!("❌ Failed to bind to port {}: {}", port, e);
            std::process::exit(1);
        }
    };

    match axum::serve(listener, app.into_make_service()).await {
        Ok(_) => println!("🚀 Server started successfully 0.0.0.0:{}", port),
        Err(e) => println!("❌ Server failed to start with error: {}", e),
    }
}
