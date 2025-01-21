use super::{routes::build_routes, FileStorage, StoragePlatform};
use database::{
    self,
    db::{Connection, Sources},
};
use environment::Environment;
use errors::Result;
use redis::Client;
use services::{
    auth::AuthServices, email::EmailServices, feed::FeedServices, gym::GymServices,
    gymseeker::GymSeekerServices, location::LocationServices, post::PostServices,
    trainer::TrainerServices,
};
use state::axum_state::AppState;
use std::sync::Arc;

use repository::{
    feed::FeedRepository, gym::GymRepository, gymseeker::GymSeekerRepository,
    location::LocationRepository, post::PostRepository, trainer::TrainerRepository,
    user::UserRepository,
};

pub async fn run() -> Result<()> {
    let mut surreal_db = database::db::DatabaseSource {
        db_type: database::db::DatabaseType::SurrealDB,
    };

    let cloud_storage = FileStorage {
        platform: StoragePlatform::Google,
    };

    let environment = Environment::new();

    let redis_url = format!(
        "redis://{}:{}@{}:{}",
        environment.redis_username,
        environment.redis_password,
        environment.redis_host,
        environment.redis_port
    );

    let redis_client = match Client::open(redis_url) {
        Ok(client) => {
            println!("✅ Connection to Redis is successful!");
            client
        }
        Err(e) => {
            println!("🔥 Error connecting to Redis: {}", e);
            std::process::exit(1);
        }
    };
    // Connect to the database
    let conn = Arc::new(surreal_db.connect().await?);
    let ping_db = conn.ping();

    if ping_db == *"Pong!" {
        println!("✅ {} from database!", ping_db);
    } else {
        println!("🔥 {} from database!", ping_db);
        std::process::exit(1);
    }


    let environment_cloned = environment.clone();

    let email_services = EmailServices {};
    let app_state = AppState {

    };

    let shared_state = Arc::new(app_state);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &environment.app_port))
        .await
        .unwrap();
    axum::serve(listener, build_routes(shared_state))
        .await
        .unwrap();

    Ok(())
}
