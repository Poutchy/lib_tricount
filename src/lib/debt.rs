use serde_json::Value::Object;

use super::{
    group::Group,
    user::User
};

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

