use std::sync::{Arc, Mutex};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
    routing::get,
    Router,
};
use futures::stream::TryStreamExt;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[derive(Debug, Clone, PartialEq)]
enum Mark {
    Empty,
    X,
    O,
}

#[derive(Debug, Clone)]
struct Board {
    marks: [[Mark; 3]; 3],
}

impl Board {
    fn place_next_x(&mut self) -> Result<(), String> {
        if self.count(&Mark::X) >= self.count(&Mark::O) {
            return Err("It's O's turn!".to_string());
        }

        for (y, row) in self.marks.iter().enumerate() {
            for (x, mark) in row.iter().enumerate() {
                match mark {
                    Mark::Empty => {
                        self.marks[y][x] = Mark::X;
                        return Ok(());
                    }
                    Mark::O | Mark::X => {}
                }
            }
        }
        Err("Full board".to_string())
    }

    fn place_next_o(&mut self) -> Result<(), String> {
        if self.count(&Mark::O) > self.count(&Mark::X) {
            return Err("It's X's turn!".to_string());
        }

        for (y, row) in self.marks.iter().enumerate() {
            for (x, mark) in row.iter().enumerate() {
                match mark {
                    Mark::Empty => {
                        self.marks[y][x] = Mark::O;
                        return Ok(());
                    }
                    Mark::O | Mark::X => {}
                }
            }
        }
        Err("Full board".to_string())
    }

    fn count(&self, mark: &Mark) -> usize {
        self.marks
            .iter()
            .flat_map(|row| row.iter())
            .filter(|in_board_mark| *in_board_mark == mark)
            .count()
    }

    fn new() -> Self {
        Self {
            marks: [
                [Mark::Empty, Mark::Empty, Mark::Empty],
                [Mark::Empty, Mark::Empty, Mark::Empty],
                [Mark::Empty, Mark::Empty, Mark::Empty],
            ],
        }
    }
}

#[derive(Debug, Clone)]
struct AppState {
    state: Arc<Mutex<Board>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let state = AppState {
        state: Arc::new(Mutex::new(Board::new())),
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
        socket
            .send(Message::Text(process(text, &state).await))
            .await
            .unwrap();
    }
}

async fn process(input: String, state: &AppState) -> String {
    let mut board = state.state.lock().unwrap();
    match input.as_str() {
        "x" | "X" => match board.place_next_x() {
            Ok(()) => {
                format!("{:?}", board)
            }
            Err(err) => err,
        },
        "o" | "O" => match board.place_next_o() {
            Ok(()) => {
                format!("{:?}", board)
            }
            Err(err) => err,
        },
        _ => format!("{:?}", board),
    }
}
