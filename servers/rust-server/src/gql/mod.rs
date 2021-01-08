pub mod http;
pub mod schema;

#[macro_use]
mod db;
#[macro_use]
mod result;

mod queries;

use result::{GqlError, GqlResult};
