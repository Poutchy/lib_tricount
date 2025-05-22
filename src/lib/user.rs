use serde::Serialize;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct User {
    name: String
}

impl User {
    pub fn new(name: String) -> User {
        User{name}
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
