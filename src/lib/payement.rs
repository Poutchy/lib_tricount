use serde::Serialize;

use super::user::User;

#[derive(Debug, Serialize)]
pub struct Payement {
    name: String,
    price: f32,
    description: String,
    paid_by: User,
    beneficiaries: Vec<User>    
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

    pub fn price(&self) -> &f32 {
        &self.price
    }

    pub fn paid_by(&self) -> &User {
        &self.paid_by
    }

    pub fn beneficiaries(&self) -> &[User] {
        &self.beneficiaries
    }
}
