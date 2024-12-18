use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

// Define the Task structure
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

// App state to store tasks
#[derive(Debug, Default)]
struct AppState {
    tasks: Mutex<Vec<Task>>,
}

#[tokio::main]
async fn main() {
    // Initialize shared state
    let state = Arc::new(AppState::default());

    // Define the app routes
    let app = Router::new()
        .route("/", get(root))
        .route("/tasks", get(get_tasks).post(create_task))
        .with_state(state);

    // Run the app
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Root handler
async fn root() -> &'static str {
    "Welcome to the Rust API with local storage!"
}

// Get tasks handler
async fn get_tasks(State(state): State<Arc<AppState>>) -> Json<Vec<Task>> {
    let tasks = state.tasks.lock().unwrap();
    Json(tasks.clone())
}

// Create task handler
async fn create_task(
    State(state): State<Arc<AppState>>,
    Json(task): Json<Task>,
) -> Json<Task> {
    let mut tasks = state.tasks.lock().unwrap();
    tasks.push(task.clone());
    println!("Task added: {:?}", task);
    Json(task)
}
