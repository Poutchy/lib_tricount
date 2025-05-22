use lib_tricount::lib::{
    group::Group,
    payement::Payement,
    reimbursement::Reimbursement,
    user::User
};

use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_compute_balances_for_user() {
        // Création des utilisateurs
        let alice = User::new("Alice".to_string());
        let bob = User::new("Bob".to_string());
        let charlie = User::new("Charlie".to_string());

        // Création des paiements
        // Alice a payé 30 pour Alice et Bob (15 chacun)
        let pay1 = Payement::new(
            "Dinner".to_string(),
            30.0,
            "Dinner at restaurant".to_string(),
            alice.clone(),
        );
        let mut pay1 = pay1;
        pay1.add_beneficiaries(alice.clone());
        pay1.add_beneficiaries(bob.clone());

        // Bob a payé 20 pour Bob et Charlie (10 chacun)
        let pay2 = Payement::new(
            "Taxi".to_string(),
            20.0,
            "Taxi ride".to_string(),
            bob.clone(),
        );
        let mut pay2 = pay2;
        pay2.add_beneficiaries(bob.clone());
        pay2.add_beneficiaries(charlie.clone());

        // Création du groupe
        let mut group = Group::default();

        group.add_user(alice.clone());
        group.add_user(bob.clone());
        group.add_user(charlie.clone());

        group.add_payement(pay1);
        group.add_payement(pay2);

        // Calcul des dettes pour Alice
        let result = group.compute_balances_for_user(alice.clone());

        // JSON attendu (Alice doit 15 à Bob, Bob doit 10 à Charlie)
        let expected = json!({
            "Bob": -15.0,
            "Charlie": 0.0
        });

        assert_eq!(result, expected);
        let result = group.compute_balances_for_user(bob.clone());

        // JSON attendu (Alice doit 15 à Bob, Bob doit 10 à Charlie)
        let expected = json!({
            "Alice": 15.0,
            "Charlie": -10.0
        });

        assert_eq!(result, expected);
        let result = group.compute_balances_for_user(charlie.clone());

        // JSON attendu (Alice doit 15 à Bob, Bob doit 10 à Charlie)
        let expected = json!({
            "Alice": 0.0,
            "Bob": 10.0
        });

        assert_eq!(result, expected);
    }

    #[test]
    fn test_compute_debt_for_user() {
        // Création des utilisateurs
        let alice = User::new("Alice".to_string());
        let bob = User::new("Bob".to_string());
        let charlie = User::new("Charlie".to_string());

        // Création des paiements
        // Alice a payé 30 pour Alice et Bob (15 chacun)
        let pay1 = Payement::new(
            "Dinner".to_string(),
            30.0,
            "Dinner at restaurant".to_string(),
            alice.clone(),
        );
        let mut pay1 = pay1;
        pay1.add_beneficiaries(alice.clone());
        pay1.add_beneficiaries(bob.clone());

        // Bob a payé 20 pour Bob et Charlie (10 chacun)
        let pay2 = Payement::new(
            "Taxi".to_string(),
            20.0,
            "Taxi ride".to_string(),
            bob.clone(),
        );
        let mut pay2 = pay2;
        pay2.add_beneficiaries(bob.clone());
        pay2.add_beneficiaries(charlie.clone());

        // Création du groupe
        let mut group = Group::default();

        group.add_user(alice.clone());
        group.add_user(bob.clone());
        group.add_user(charlie.clone());

        group.add_payement(pay1);
        group.add_payement(pay2);

        // Calcul des dettes pour Alice
        let result = group.compute_debt_for_user(alice.clone());
        assert_eq!(result, -15.0);

        let result = group.compute_debt_for_user(bob.clone());
        assert_eq!(result, 5.0);

        let result = group.compute_debt_for_user(charlie.clone());
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_compute_all_reimbursment() {
        // Création des utilisateurs
        let alice = User::new("Alice".to_string());
        let bob = User::new("Bob".to_string());
        let charlie = User::new("Charlie".to_string());

        // Création des paiements
        // Alice a payé 30 pour Alice et Bob (15 chacun)
        let pay1 = Payement::new(
            "Dinner".to_string(),
            30.0,
            "Dinner at restaurant".to_string(),
            alice.clone(),
        );
        let mut pay1 = pay1;
        pay1.add_beneficiaries(alice.clone());
        pay1.add_beneficiaries(bob.clone());

        // Bob a payé 20 pour Bob et Charlie (10 chacun)
        let pay2 = Payement::new(
            "Taxi".to_string(),
            20.0,
            "Taxi ride".to_string(),
            bob.clone(),
        );
        let mut pay2 = pay2;
        pay2.add_beneficiaries(bob.clone());
        pay2.add_beneficiaries(charlie.clone());

        // Création du groupe
        let mut group = Group::default();

        group.add_user(alice.clone());
        group.add_user(bob.clone());
        group.add_user(charlie.clone());

        group.add_payement(pay1);
        group.add_payement(pay2);

        let result: HashSet<_> = group
            .compute_all_reimbursements()
            .into_iter()
            .collect();

        let expected: HashSet<_> = [
            Reimbursement::new(
                charlie.clone(),
                alice.clone(),
                10.0,
            ),
            Reimbursement::new(
                bob.clone(),
                alice.clone(),
                5.0,
            ),
        ]
            .into_iter()
            .collect();

        assert_eq!(result, expected);
    }
}
