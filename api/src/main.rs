mod config;
mod database;
mod models;
mod state;

use crate::config::AppConfig;
use anyhow::Result;
use axum::extract::State;
use axum::{Router, response::Json, routing::get};
use database::connect_to_database;
use deepseek_api::{Client, request::MessageRequest};
use serde_json::{Value, json};
use state::AppState;
use std::sync::Arc;
use std::vec;
use tracing::error;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let cfg = AppConfig::from_env().expect("Failed to load configuration");
    let db = match connect_to_database(cfg.database_url).await {
        Ok(db) => db,
        Err(e) => {
            error!("Failed to connect to database: {}", e);
            std::process::exit(1); // Exit with error
        }
    };
    // Build shared app state
    let app_state = AppState::new(Arc::new(db));

    // build our application with a single route
    let app = Router::new()
        .route("/", get(dummy_post))
        .route("/home", get(home))
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn query_ds() -> Result<()> {
    let client = Client::new("");
    println!("In query_ds");
    let mut history = vec![];
    let mut completions = client.chat();
    let builder = completions
        .chat_builder(vec![])
        .append_user_message("Hi Deepseek!");
    let resp = completions.create(builder).await?.must_response();

    let mut resp_words = vec![];
    for msg in resp.choices.iter() {
        let resp_msg = MessageRequest::from_message(msg.message.as_ref().expect("message"))?;
        history.push(resp_msg);
        resp_words.push(msg.message.as_ref().expect("message").content.clone());
    }
    for msg in resp_words.iter() {
        msg.split("\n").for_each(|x| println!("{}", x));
    }
    Ok(())
}

async fn dummy_post() -> Json<Value> {
    let _ = query_ds().await;
    Json(json!({ "response": 200 }))
}

async fn home(State(state): State<AppState>) -> Json<Value> {
    let _db_connection = state.db_arc();
    Json(json!({ "response": 200 }))
}
