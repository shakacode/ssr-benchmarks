use rand::Rng;
use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() {
    let pool = pool().await;

    for _ in 1..100 {
        let title = lipsum::lipsum_title();
        let content = lipsum::lipsum_words(rand::thread_rng().gen_range(80..200));
        insert_post(title, content, &pool).await;
    }
}

#[derive(Debug)]
pub struct PostInput {
    pub title: String,
    pub content: String,
}

pub async fn insert_post(title: String, content: String, db: &PgPool) {
    sqlx::query("INSERT INTO posts (title, content) VALUES ($1, $2)")
        .bind(&title)
        .bind(&content)
        .execute(db)
        .await
        .unwrap();
}

async fn pool() -> PgPool {
    let addr = format!(
        "postgres://{user}:{password}@{host}:{port}/{db}",
        user = env::pg_user(),
        password = env::pg_password(),
        host = env::pg_host(),
        port = env::pg_port(),
        db = env::pg_database()
    );

    PgPool::connect(&addr)
        .await
        .expect("Failed to initialize DB pool")
}

mod env {
    use std::env;

    pub fn pg_host() -> String {
        env::var("PG_HOST").expect("[env] PG_HOST is not set")
    }

    pub fn pg_port() -> String {
        env::var("PG_PORT").expect("[env] PG_PORT is not set")
    }

    pub fn pg_user() -> String {
        env::var("PG_USER").expect("[env] PG_USER is not set")
    }

    pub fn pg_password() -> String {
        env::var("PG_PASSWORD").expect("[env] PG_PASSWORD is not set")
    }

    pub fn pg_database() -> String {
        env::var("PG_DATABASE").expect("[env] PG_DATABASE is not set")
    }
}
