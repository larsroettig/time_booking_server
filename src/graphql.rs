use super::context::GraphQLContext;
use juniper::{FieldResult,RootNode,EmptyMutation};

use crate::graphql_model::project::Project;
use super::resource_model::project_rs::ProjectRs;

// The root GraphQL query
pub struct Query;

// The root Query struct relies on GraphQLContext to provide the connection pool
// needed to execute actual Postgres queries.
#[juniper::object(Context = GraphQLContext)]
impl Query {
    pub fn projects(context: &GraphQLContext, page: Option<i32>, page_size: Option<i32>) -> FieldResult<Vec<Project>> {
        ProjectRs::all_projects(context, page, page_size)
    }
}

// And finally the root schema that pulls the query and mutation together. Perhaps someday
// you'll see a Subscription struct here as well.
pub type Schema = RootNode<'static, Query, EmptyMutation<GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(Query, EmptyMutation::new())
}
