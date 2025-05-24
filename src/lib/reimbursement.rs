use ordered_float::OrderedFloat;
use serde::Serialize;

use super::user::User;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct Reimbursement {
    pub from: User,
    pub to: User,
    pub amount: OrderedFloat<f32>,
}

impl Reimbursement {
    pub fn new(from: User, to: User, amount: f32) -> Self {
        Reimbursement { from, to, amount: OrderedFloat(amount) }
    }
}

