use serde_json::{json, Value};
use std::collections::HashMap;

use super::{
    group::Group,
    user::User,
};

impl Group {
    pub fn compute_balances_for_user(&self, user: User) -> serde_json::Value {
        let mut balances: HashMap<String, f32> = HashMap::new();

        for other_user in self.users() {
            if other_user.name() != user.name() {
                balances.insert(other_user.name().to_string(), 0.0);
            }
        }

        for payement in self.payements() {
            let total_amount = payement.price();
            let payer = &payement.paid_by();
            let beneficiaries = payement.beneficiaries();
            let share = total_amount / beneficiaries.len() as f32;

            let user_is_beneficiaries = beneficiaries.iter().any(|b| b.name() == user.name());

            if user_is_beneficiaries {
                for beneficiary in beneficiaries {
                    if beneficiary.name() != user.name() && payer.name() == user.name() {
                        *balances.get_mut(beneficiary.name()).unwrap() -= share;
                    }

                    if beneficiary.name() == user.name() && payer.name() != user.name() {
                        *balances.get_mut(payer.name()).unwrap() += share;
                    }
                    
                }
            } else if payer.name() == user.name() {
                for beneficiary in beneficiaries {
                    *balances.get_mut(beneficiary.name()).unwrap() -= share;
                }
            }
        }

        let res = balances.into_iter()
            .map(|(name, amount)| (name, json!(amount)))
            .collect::<serde_json::Map<_, _>>();

        Value::Object(res)
    }
}

