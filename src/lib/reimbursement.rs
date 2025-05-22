use std::collections::HashMap;

use ordered_float::OrderedFloat;
use serde::Serialize;

use super::{
    group::Group,
    user::User
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct Reimbursement {
    from: User,
    to: User,
    amount: OrderedFloat<f32>,
}

impl Reimbursement {
    pub fn new(from: User, to: User, amount: f32) -> Self {
        Reimbursement { from, to, amount: OrderedFloat(amount) }
    }
}

impl Group {
    pub fn compute_all_reimbursements(&self) -> Vec<Reimbursement> {
        // 1. Calculer la balance nette pour chaque utilisateur (ce qu'il doit ou lui doit)
        // Par exemple, balance > 0 = le groupe lui doit de l'argent (créancier)
        // balance < 0 = il doit de l'argent au groupe (débiteur)

        let mut balances: HashMap<&User, f32> = HashMap::new();
        for user in self.users() {
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
        while !debtors.is_empty() && !creditors.is_empty() {
            let (debtor, mut debt_amount) = debtors.pop().unwrap();
            let (creditor, mut credit_amount) = creditors.pop().unwrap();

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
