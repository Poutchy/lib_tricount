use serde_json::{json, Value, Value::Object};
use std::collections::HashMap;

use super::payement::Payement;
use super::reimbursement::Reimbursement;
use super::user::User;

#[derive(Debug, Default)]
pub struct Group {
    pub users: Vec<User>,
    pub payements: Vec<Payement>,
}

impl Group {
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
        for user in &self.users {
            plus.insert(user, 0.0);
            minus.insert(user, 0.0);
        }
        for payement in self.payements.iter() {
            if let Some(entry) = plus.get_mut(&payement.paid_by) {
                *entry += payement.price;
            }
            for person in &payement.beneficiaries {
                if person.name != payement.paid_by.name {
                    if let Some(entry) = minus.get_mut(&person) {
                        *entry -= payement.price;
                    }
                }
            }
            total += payement.price;
        }
        let share: f32 = total / self.users.len() as f32;
    }
}

impl Group {
    pub fn compute_debt_for_user(&self, user: User) -> f32 {
        let mut res = 0.0;
        let temp = self.compute_balances_for_user(user);

        if let Object(map) = &temp {
            for (_, value) in map.iter() {
                if let Some(f) = value.as_f64(){
                    res += f as f32;
                }
            }
        }
        res
    }
}


impl Group {
    pub fn compute_balances_for_user(&self, user: User) -> serde_json::Value {
        let mut balances: HashMap<String, f32> = HashMap::new();

        for other_user in &self.users {
            if other_user.name != user.name {
                balances.insert(other_user.name.to_string(), 0.0);
            }
        }

        for payement in &self.payements {
            let total_amount = payement.price;
            let payer = &payement.paid_by;
            let beneficiaries = &payement.beneficiaries;
            let share = total_amount / beneficiaries.len() as f32;

            let user_is_beneficiaries = beneficiaries.iter().any(|b| b.name == user.name);

            if user_is_beneficiaries {
                for beneficiary in beneficiaries {
                    if beneficiary.name != user.name && payer.name == user.name {
                        if let Some(entry) = balances.get_mut(&beneficiary.name) {
                            *entry -= share;
                        }
                    }

                    if beneficiary.name == user.name && payer.name != user.name {
                        if let Some(entry) = balances.get_mut(&payer.name) { 
                            *entry += share;
                        }
                    }
                    
                }
            } else if payer.name == user.name {
                for beneficiary in beneficiaries {
                    if let Some(entry) = balances.get_mut(&beneficiary.name) {
                        *entry -= share;
                    }
                }
            }
        }

        let res = balances.into_iter()
            .map(|(name, amount)| (name, json!(amount)))
            .collect::<serde_json::Map<_, _>>();

        Value::Object(res)
    }
}


impl Group {
    pub fn compute_all_reimbursements(&self) -> Vec<Reimbursement> {
        // 1. Calculer la balance nette pour chaque utilisateur (ce qu'il doit ou lui doit)
        // Par exemple, balance > 0 = le groupe lui doit de l'argent (créancier)
        // balance < 0 = il doit de l'argent au groupe (débiteur)

        let mut balances: HashMap<&User, f32> = HashMap::new();
        for user in &self.users {
            balances.insert(user, self.compute_debt_for_user(user.clone())); 
            // positive = dette utilisateur envers groupe, negative = crédit
        }

        // 2. Séparer les débiteurs et les créanciers
        let mut debtors: Vec<(&User, f32)> = balances.iter()
            .filter(|&(_, balance)| *balance > 0.0)   // negative = detenteur d'argent
            .map(|(user, &balance)| (*user, balance)) // montant positif à payer
            .collect();

        let mut creditors: Vec<(&User, f32)> = balances.iter()
            .filter(|&(_, balance)| *balance < 0.0)  // positive = doit payer
            .map(|(user, &balance)| (*user, -balance))
            .collect();
        // 3. Calculer les remboursements
        let mut reimbursements = Vec::new();

        // tant qu'il y a des dettes et des crédits
        while let (Some((debtor, mut debt_amount)), Some((creditor, mut credit_amount))) = (debtors.pop(), creditors.pop()) {
            // montant remboursé est le minimum entre dette et crédit
            let payment = debt_amount.min(credit_amount);

            reimbursements.push(Reimbursement::new(
                debtor.clone(),
                creditor.clone(),
                payment,
            ));

            // réduire les soldes restants
            debt_amount -= payment;
            credit_amount -= payment;

            // réinsérer si solde restant > 0
            if debt_amount > 0.0 {
                debtors.push((debtor, debt_amount));
            }
            if credit_amount > 0.0 {
                creditors.push((creditor, credit_amount));
            }
        }

        reimbursements
    }
}
