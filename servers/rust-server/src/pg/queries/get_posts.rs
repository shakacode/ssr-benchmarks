use crate::{
    models::post::{Post, PostId},
    pg::PgPool,
};

pub async fn exec(db: &PgPool) -> sqlx::Result<Vec<Post>> {
    sqlx::query_file_as!(Post, "src/pg/queries/get_posts.sql")
        .fetch_all(db)
        .await
}
