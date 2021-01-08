use actix_files::Files;
use actix_web::{guard, web, App, HttpServer};

use crate::{env, gql, pg, ssr};

pub async fn run() -> std::io::Result<()> {
    let pg = pg::pool::new().await;
    let gql = gql::schema::new();
    let ssr = ssr::new().await;

    HttpServer::new(move || {
        App::new()
            .data(pg.clone())
            .data(gql.clone())
            .data(ssr.clone())
            .route("/", web::get().to(ssr::posts::endpoint))
            .service(Files::new("/assets", "./servers/rust-server/assets/"))
            .service(
                web::resource(env::graphql_path())
                    .guard(guard::Post())
                    .to(gql::http::api::endpoint),
            )
            .service(
                web::resource("/gql")
                    .guard(guard::Get())
                    .to(gql::http::playground::endpoint),
            )
    })
    .bind(&format!("127.0.0.1:{}", env::app_port()))?
    .run()
    .await
}
