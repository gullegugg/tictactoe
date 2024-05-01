use std::sync::Arc;

use axum::{
    extract::{ws::WebSocket, State, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};
use futures::stream::TryStreamExt;
use tokio::{net::TcpListener, sync::Mutex};
use tower_http::services::ServeDir;

#[derive(Debug)]
enum Mark {
    Empty,
    X,
    O,
}

#[derive(Debug)]
struct Row(Mark, Mark, Mark);

#[derive(Debug)]
struct Board(Mark, Mark, Mark);

#[derive(Debug)]
struct GameState {
    board: Board,
}

#[derive(Debug, Clone)]
struct AppState {
    state: Arc<Mutex<Option<GameState>>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = AppState {
        state: Arc::new(Mutex::new(None)),
    };
    let router = Router::new()
        .route("/ws", get(ws_handler))
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(state);

    let listener = TcpListener::bind("localhost:8080").await?;

    axum::serve(listener, router).await?;

    Ok(())
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| socket_callback(socket, state))
}

async fn socket_callback(mut socket: WebSocket, state: AppState) {
    while let Some(msg) = socket.try_next().await.unwrap() {
        let text = msg.into_text().unwrap();
        println!("Message: {}", text);
    }
}
