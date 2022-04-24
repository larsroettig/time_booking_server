use crate::schema::projects;


#[derive(Queryable)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub abbreviation: String,
    pub description: Option<String>,
}

#[juniper::object(description = "Project")]
impl Project {
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn abbreviation(&self) -> &str {
        self.abbreviation.as_str()
    }
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
}
