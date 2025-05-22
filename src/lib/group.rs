use std::collections::HashMap;

use super::user::User;
use super::payement::Payement;

#[derive(Debug, Default)]
pub struct Group {
    users: Vec<User>,
    payements: Vec<Payement>,
}

impl Group {
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

impl Group {
    pub fn to_json(&self) {
        let mut total: f32 = 0.0;
        let mut plus: HashMap<&User, f32> = HashMap::new();
        let mut minus: HashMap<&User, f32> = HashMap::new();
        for user in self.users() {
            plus.insert(user, 0.0);
            minus.insert(user, 0.0);
        }
        for payement in self.payements.iter() {
            *plus.get_mut(payement.paid_by()).unwrap() += payement.price();
            for person in payement.beneficiaries() {
                if person.name() != payement.paid_by().name() {
                    *minus.get_mut(person).unwrap() -= payement.price();
                }
            }
            total += payement.price();
        }
        let share: f32 = total / self.users().len() as f32;
    }
}
