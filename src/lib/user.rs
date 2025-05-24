use serde::Serialize;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize)]
pub struct User {
    pub name: String
}

impl User {
    pub fn new(name: String) -> Self{
       Self { name } 
    }
}
