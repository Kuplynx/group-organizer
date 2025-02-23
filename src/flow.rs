use std::collections::HashMap;

use crate::{colors, version, OutputData};

pub async fn on_startup(port: u16) -> tokio::net::TcpListener {
    let border: &str = "========================================";
    println!("{}{}", colors::CLEAR_CONSOLE, colors::GREEN);
    println!("{}", border);
    println!("\n  Starting server version {}          ", version::VERSION);
    println!("  Listening on http://{}              \n", port);
    println!("{}", border);
    println!("{}", colors::CLEAR);

    webbrowser::open(&format!("http://localhost:{}", port)).unwrap();

    tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap()
}

pub fn cover_to_json(
    cover: Vec<Vec<usize>>,
    attempts_made: u64,
    times_backtracked: u64,
    usize_to_string: HashMap<usize, String>,
) -> axum::Json<OutputData> {
    let mut output = vec![];
    for group in cover {
        let mut group_str = vec![];
        for student in group {
            group_str.push(usize_to_string.get(&student).unwrap().clone());
        }
        output.push(group_str);
    }
    let output = OutputData {
        cover: output,
        attempts_made,
        times_backtracked,
    };
    axum::Json(output)
}

pub fn on_billion_attempts(attempts: u64) -> bool {
    if attempts >= (1_073_741_824 * 32) {
        println!("{}Stopping after 34,359,738,368 attempts{}", colors::RED, colors::CLEAR);
        return true;
    }
    false
}