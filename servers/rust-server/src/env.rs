use std::env;

pub fn app_port() -> String {
    env::var("RUST_SERVER_PORT").expect("RUST_SERVER_PORT is not set")
}

pub fn graphql_path() -> String {
    env::var("GRAPHQL_PATH").expect("GRAPHQL_PATH is not set")
}

pub fn pg_host() -> String {
    env::var("PG_HOST").expect("PG_HOST is not set")
}

pub fn pg_port() -> String {
    env::var("PG_PORT").expect("PG_PORT is not set")
}

pub fn pg_user() -> String {
    env::var("PG_USER").expect("PG_USER is not set")
}

pub fn pg_password() -> String {
    env::var("PG_PASSWORD").expect("PG_PASSWORD is not set")
}

pub fn pg_database() -> String {
    env::var("PG_DATABASE").expect("PG_DATABASE is not set")
}
