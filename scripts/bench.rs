use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use futures::StreamExt;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let connections = env::connections();

    println!(
        "Running total {} requests in groups. {} concurrent requests per group.",
        TOTAL_REQUESTS, connections
    );

    let next_server_results = run(NextServer::url(), connections).await;
    interpret("Next server", next_server_results);

    let node_server_results = run(NodeServer::url(), connections).await;
    interpret("Node server", node_server_results);

    let rust_server_results = run(RustServer::url(), connections).await;
    interpret("Rust server", rust_server_results);
}

const TOTAL_REQUESTS: usize = 1000;

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

async fn run(url: String, connections: usize) -> Vec<Result<Duration, ()>> {
    let results = Arc::new(Mutex::new(vec![]));

    futures::stream::iter(vec![0; TOTAL_REQUESTS])
        .for_each_concurrent(connections, |_| {
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
    let request =
        reqwest::Client::new()
            .get(url)
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
            .header("Accept-Encoding", "gzip, deflate")
            .header("Accept-Language", "en-us")
            .header("Cookie", "WMF-Last-Access=08-Jan-2021; enwikimwuser-sessionId=041fb3ba99e057d356e5; WMF-Last-Access-Global=08-Jan-2021")
            .header( "User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Safari/605.1.15");

    let start = Instant::now();

    match request.send().await {
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

    pub fn connections() -> usize {
        let connections = env::var("CONNECTIONS").expect("[env] CONNECTIONS is not set");
        connections
            .parse::<usize>()
            .expect("Number of concurrent connections must be a positive integer")
    }
}
