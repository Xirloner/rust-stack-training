use anyhow::Result;
use serde_json::{json, Value};
use deepseek_api::{Client, request::MessageRequest, response::ModelType};
use std::io::{stdin, stdout, Write};
use std::vec;
use axum::{
    routing::get,
    response::Json,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(dummy_post));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn query_ds() -> Result<()> {
    let client = Client::new("");
    println!("In query_ds");
    let mut history = vec![];
    let mut completions = client.chat();
    let builder = completions.chat_builder(vec![]).append_user_message("Hi Deepseek!");
    let resp = completions.create(builder).await?.must_response();

    let mut resp_words = vec![];
    for msg in resp.choices.iter() {
        let resp_msg =
            MessageRequest::from_message(msg.message.as_ref().expect("message"))?;
        history.push(resp_msg);
        resp_words.push(msg.message.as_ref().expect("message").content.clone());
    }
    for msg in resp_words.iter() {
        msg.split("\n").for_each(|x| println!("{}", x));
    }
    Ok(())
}

async fn dummy_post() ->Json<Value> {
    let _ = query_ds().await;
    Json(json!({ "response": 200 }))
}