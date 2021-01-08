pub type GqlResult<T> = Result<T, GqlError>;

#[derive(graphql::SimpleObject)]
#[graphql(name = "Ok")]
pub struct GqlOk {
    ok: bool,
}

const INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

pub enum GqlError {
    InternalServerError,
}

impl Into<graphql::Error> for GqlError {
    fn into(self) -> graphql::Error {
        match self {
            GqlError::InternalServerError => graphql::Error::new(INTERNAL_SERVER_ERROR),
        }
    }
}

#[macro_export]
macro_rules! gql_error {
    ($error:item) => {
        #[derive(serde::Serialize, Copy, Clone)]
        #[serde(
            tag = "reason",
            content = "payload",
            rename_all = "SCREAMING_SNAKE_CASE"
        )]
        $error
    };
}
