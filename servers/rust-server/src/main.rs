#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate log;

mod env;
mod gql;
mod models;
mod pg;
mod server;
mod ssr;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    server::run().await
}
