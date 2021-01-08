use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use futures::StreamExt;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let concurrent_reqs = get_number_of_concurrent_reqs();

    println!(
        "Running total {} requests in groups. {} concurrent requests per group.",
        TOTAL_REQUESTS, concurrent_reqs
    );

    let next_server_results = run(NextServer::url(), concurrent_reqs).await;
    interpret("Next server", next_server_results);

    let node_server_results = run(NodeServer::url(), concurrent_reqs).await;
    interpret("Node server", node_server_results);

    let rust_server_results = run(RustServer::url(), concurrent_reqs).await;
    interpret("Rust server", rust_server_results);
}

const TOTAL_REQUESTS: usize = 1000;

fn get_number_of_concurrent_reqs() -> usize {
    let args = std::env::args().collect::<Vec<String>>();

    match args.as_slice() {
        [_, n] => n.parse::<usize>().expect("Failed to parse provided argument: make sure you provided a valid number of concurrent requests"),
        [_] => panic!("Number of concurrent requests is not provided."),
        _ => panic!(
            "Unexpected set of arguments. You can only provide a number of concurrent requests."
        ),
    }
}

struct RustServer;

impl RustServer {
    fn url() -> String {
        format!("http://127.0.0.1:{}/", env::rust_server_port())
    }
}

struct NodeServer;

impl NodeServer {
    fn url() -> String {
        format!("http://127.0.0.1:{}/", env::node_server_port())
    }
}

struct NextServer;

impl NextServer {
    fn url() -> String {
        format!("http://127.0.0.1:{}/", env::next_server_port())
    }
}

async fn run(url: String, concurrent_reqs: usize) -> Vec<Result<Duration, ()>> {
    let results = Arc::new(Mutex::new(vec![]));

    futures::stream::iter(vec![0; TOTAL_REQUESTS])
        .for_each_concurrent(concurrent_reqs, |_| {
            let url = url.as_str();
            let results = results.clone();
            async move {
                let res = get(url).await;
                let mut lock = results.lock().await;
                lock.push(res);
            }
        })
        .await;

    Arc::try_unwrap(results).unwrap().into_inner()
}

async fn get(url: &str) -> Result<Duration, ()> {
    let start = Instant::now();

    match reqwest::get(url).await {
        Ok(res) => {
            if res.status().is_success() {
                Ok(start.elapsed())
            } else {
                Err(())
            }
        }
        Err(_) => Err(()),
    }
}

fn interpret(server: &str, results: Vec<Result<Duration, ()>>) {
    let (total_oks, total_errs, mut resp_times) =
        results
            .iter()
            .fold((0, 0, vec![]), |(oks, errs, mut times), res| match res {
                Ok(x) => {
                    times.push(*x);
                    (oks + 1, errs, times)
                }
                Err(_) => (oks, errs + 1, times),
            });

    println!("\n{}:", server);

    match resp_times.as_slice() {
        [] => println!("  All requests failed"),
        _ => {
            println!("  Total oks: {}", total_oks);
            println!("  Total errs: {}", total_errs);
            println!("  Average response time: {:?}", average(&resp_times));
            println!("  Median response time: {:?}", median(&mut resp_times));
            // println!("  All response times: {:#?}", resp_times);
        }
    }
}

fn average(xs: &Vec<Duration>) -> Duration {
    Duration::from_secs_f32(xs.iter().sum::<Duration>().as_secs_f32() / xs.len() as f32)
}

fn median(xs: &mut Vec<Duration>) -> Duration {
    xs.sort();
    let mid = xs.len() / 2;
    xs[mid]
}

mod env {
    use std::env;

    pub fn rust_server_port() -> String {
        env::var("RUST_SERVER_PORT").expect("[env] RUST_SERVER_PORT is not set")
    }

    pub fn node_server_port() -> String {
        env::var("NODE_SERVER_PORT").expect("[env] NODE_SERVER_PORT is not set")
    }

    pub fn next_server_port() -> String {
        env::var("NEXT_SERVER_PORT").expect("[env] NEXT_SERVER_PORT is not set")
    }
}
