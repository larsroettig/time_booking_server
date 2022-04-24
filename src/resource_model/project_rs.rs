use diesel::{QueryDsl, RunQueryDsl};
use juniper::{FieldError, FieldResult};
use crate::context::GraphQLContext;
use crate::graphql_model::project::Project;

pub struct ProjectRs;

impl ProjectRs {

    pub fn all_projects(context: &GraphQLContext, page: Option<i32>, page_size: Option<i32>) -> FieldResult<Vec<Project>> {
        use crate::schema::projects::dsl::*;

        let mut size = page_size.unwrap_or(10);
        let mut offset = (page.unwrap_or(1) - 1) * size;

        if offset < 0 { offset = 0; }
        if size < 0 { size = 0; }

        let result = projects
            .limit(size as i64)
            .offset(offset as i64)
            .load::<Project>(&context.pool.get().unwrap());

        graphql_translate(result)
    }
}

fn graphql_translate<T>(res: Result<T, diesel::result::Error>) -> FieldResult<T> {
    match res {
        Ok(t) => Ok(t),
        Err(e) => FieldResult::Err(FieldError::from(e)),
    }
}