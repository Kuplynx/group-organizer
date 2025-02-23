use crate::colors::CLEAR;
use crate::version::VERSION;
use axum::response::Html;
use axum::{
    extract::{Json, Path},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use flow::{cover_to_json, on_billion_attempts, on_startup};
use tokio::task;
use types::{ExactCover, InputData, OutputData};

mod colors;
mod exact_cover;
mod flow;
mod permutations;
mod types;
mod version;

#[axum::debug_handler]
async fn cover_handler(
    Path((group_size, num_groups)): Path<(u32, u32)>,
    Json(payload): Json<InputData>,
) -> impl IntoResponse {
    println!("{CLEAR}Computing cover for group size {group_size} and {num_groups} groups");
    let start_time = std::time::Instant::now();
    let (usize_map, _, usize_to_string) =
        permutations::create_usize_map(&payload.map);
    let permutations: Vec<Vec<usize>> =
        permutations::find_valid_permutations(&usize_map, group_size as usize);
    let mut cover = ExactCover::new(permutations, num_groups as usize);

    
    let result: (Vec<Vec<usize>>, u64, u64) = task::spawn_blocking(move || {
        (
            cover.solve(&on_billion_attempts),
            cover.attempts_made,
            cover.times_backtracked,
        )
    })
    .await
    .unwrap();
    let end_time = std::time::Instant::now();

    cover_to_json(result.0, result.1, result.2, usize_to_string)
}

async fn version_route() -> impl IntoResponse {
    Json(VERSION)
}

async fn index() -> impl IntoResponse {
    Html(include_str!("../index.html"))
}

#[tokio::main]
async fn main() {
    let port = 3000;
    let app: Router = Router::new()
        .route("/compute/{group_size}/{num_groups}", post(cover_handler))
        .route("/version", get(version_route))
        .route("/", get(index));
    let listener: tokio::net::TcpListener = on_startup(port).await;
    axum::serve(listener, app).await.unwrap();
}
