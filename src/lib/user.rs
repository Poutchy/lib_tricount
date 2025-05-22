use serde::Serialize;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize)]
pub struct User {
    name: String
}

impl User {
    pub fn new(name: String) -> Self{
       Self { name } 
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
