use actix_web::http::header::EntityTag;
use blake2::{Blake2b, Digest};

pub fn new(res: &String) -> EntityTag {
    let mut hasher = Blake2b::new();
    hasher.update(res);
    let hash = hasher.finalize();
    let tag = base64::encode(hash.as_slice());

    EntityTag::strong(tag)
}
