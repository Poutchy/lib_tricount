
use super::user::User;
use super::payement::Payement;

#[derive(Debug)]
pub struct Group {
    users: Vec<User>,
    payements: Vec<Payement>,
}

impl Group {
    pub fn new() -> Group {
        Group {
            users: Vec::<User>::new(),
            payements: Vec::<Payement>::new(),
        }
    }

    pub fn users(&self) -> &[User] {
        &self.users
    }

    pub fn payements(&self) -> &[Payement] {
        &self.payements
    }

    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn add_payement(&mut self, payement: Payement) {
        self.payements.push(payement);
    }
}
