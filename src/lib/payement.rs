use serde::Serialize;

use super::user::User;

#[derive(Debug, Serialize)]
pub struct Payement {
    pub name: String,
    pub price: f32,
    pub description: String,
    pub paid_by: User,
    pub beneficiaries: Vec<User>    
}

impl Payement {
    pub fn new(name: String, price: f32, description: String, paid_by: User) -> Self {
        assert!(price > 0.0, "Price < 0");
        Self {
            name,
            price,
            description,
            paid_by,
            beneficiaries: Vec::<User>::new(),
        }
    }

    pub fn add_beneficiaries(&mut self, user: User) {
        self.beneficiaries.push(user);
    }
}
