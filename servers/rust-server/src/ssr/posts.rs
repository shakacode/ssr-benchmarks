use actix_web::{http::header, web::Data, HttpRequest, HttpResponse};
use ssr::{JsRenderer, Ssr};

use crate::{
    etag,
    pg::{queries as db, PgPool},
};

pub async fn endpoint(pg: Data<PgPool>, ssr: Data<Ssr>, req: HttpRequest) -> HttpResponse {
    let uri = req.uri();
    let res = db::get_posts::exec(&pg).await;

    match res {
        Ok(data) => {
            let data = json!({ "posts": data });
            match ssr.render(uri, &data, JsRenderer::Global).await {
                Ok(html) => HttpResponse::Ok()
                    .content_type("text/html; charset=utf-8")
                    .header(header::ETAG, etag::new(&html))
                    .header(
                        header::CACHE_CONTROL,
                        "private, no-cache, no-store, max-age=0, must-revalidate",
                    )
                    .body(html),
                Err(error) => {
                    error!("{}", error);
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(error) => {
            error!("{}", error);
            HttpResponse::InternalServerError().finish()
        }
    }
}
