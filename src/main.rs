use crate::colors::{CLEAR, CLEAR_CONSOLE, GREEN};
use crate::version::VERSION;
use axum::response::Html;
use axum::{
    Router,
    extract::{Json, Path, Query},
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tokio::task;
use types::{ExactCover, InputData, OutputData, QueryInfo};

mod colors;
mod exact_cover;
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
    let (usize_map, string_to_usize, usize_to_string) =
        permutations::create_usize_map(&payload.map);
    let permutations: Vec<Vec<usize>> =
        permutations::find_valid_permutations(&usize_map, group_size as usize);
    let mut cover = ExactCover::new(permutations, num_groups as usize);
    let result: (Vec<Vec<usize>>, u64, u64) = task::spawn_blocking(move || {
        (
            cover.solve(&|attempts| {
                if attempts % 1_073_741_824 == 0 {
                    println!("{} billion attempts made", attempts / 1_000_000_000);
                }
            }),
            cover.attempts_made,
            cover.times_backtracked,
        )
    })
    .await
    .unwrap();
    let end_time = std::time::Instant::now();
    println!(
        "{CLEAR}Cover computed in {} seconds, with {} attempts made and {} times backtracked",
        (end_time - start_time).as_secs_f32(),
        result.1,
        result.2
    );

    let mut output = vec![];
    for group in result.0 {
        let mut group_str = vec![];
        for student in group {
            group_str.push(usize_to_string.get(&student).unwrap().clone());
        }
        output.push(group_str);
    }
    let output = OutputData {
        cover: output,
        attempts_made: result.1,
        times_backtracked: result.2,
    };
    Json(output)
}

async fn version_route() -> impl IntoResponse {
    Json(VERSION)
}

async fn index() -> impl IntoResponse {
    Html(include_str!("../index.html"))
}

#[tokio::main]
async fn main() {
    let app: Router = Router::new()
        .route("/compute/{group_size}/{num_groups}", post(cover_handler))
        .route("/version", get(version_route))
        .route("/", get(index));
    println!("{CLEAR_CONSOLE}{GREEN}Group Organizer {VERSION} ready!{CLEAR}");
    let listener: tokio::net::TcpListener =
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    webbrowser::open(format!("{}{}", "http://localhost:", 3000).as_str()).unwrap();
    axum::serve(listener, app).await.unwrap();
}
