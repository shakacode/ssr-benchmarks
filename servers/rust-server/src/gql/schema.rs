use graphql::{Context, EmptyMutation, EmptySubscription, FieldResult, Schema};

use crate::{
    gql::{queries, GqlError},
    models::post::Post,
};

pub type GqlSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn new() -> GqlSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription).finish()
}

pub struct Query;

#[graphql::Object]
impl Query {
    async fn posts(&self, ctx: &Context<'_>) -> FieldResult<Vec<Post>> {
        queries::posts::exec(ctx).await.map_err(GqlError::into)
    }
}
