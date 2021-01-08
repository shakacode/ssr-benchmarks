pub mod posts;

use std::path::PathBuf;

use ssr::{JsWorkerLog, Ssr, SsrConfig};

pub async fn new() -> Ssr {
    Ssr::new(SsrConfig {
        port: 9000,
        js_worker: PathBuf::from("./servers/rust-server/node_modules/ssr-rs/worker.js"),
        js_worker_log: JsWorkerLog::Minimal,
        global_js_renderer: Some(PathBuf::from("./servers/rust-server/assets/ssr.js")),
    })
    .await
    .expect("Failed to initialize node renderer")
}
