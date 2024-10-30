use std::collections::BTreeMap;

#[warn(dead_code)]

pub struct Pallet {
    balance: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balance: BTreeMap::new(),
        }
    }

    /// Set the balance of an account `who` to some `amount`.
    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balance.insert(who.clone(), amount);
    }

    /// Get the balance of an account `who`.
    /// If the account has no stored balance, we return zero.
    pub fn balance(&self, who: &String) -> u128 {
        *self.balance.get(who).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        from: &String,
        to: &String,
        amount: u128,
    ) -> Result<(), &'static str> {
        let from_balance = self.balance(&from);
        let to_balance = self.balance(&to);

        let from_new_balance = from_balance
            .checked_sub(amount)
            .ok_or("Not enough balance")?;

        let to_new_balance = to_balance
            .checked_add(amount)
            .ok_or("Overflow when adding to balance")?;

        self.set_balance(from, from_new_balance);
        self.set_balance(to, to_new_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn init_balance() {
        let mut balance = super::Pallet::new();

        assert_eq!(balance.balance(&"alice".to_string()), 0);
        balance.set_balance(&"alice".to_string(), 100);
        assert_eq!(balance.balance(&"alice".to_string()), 100);
        assert_eq!(balance.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        let mut balance = super::Pallet::new();

        balance.set_balance(&"alice".to_string(), 100);

        balance.transfer(&alice.clone(), &bob, 90);

        assert_eq!(balance.balance(&alice), 10);
        assert_eq!(balance.balance(&bob), 90);
    }

    #[test]
    fn transfer_balance_overflow() {
        let bob = "bob".to_string();
        let alice = "alice".to_string();

        let mut balance = super::Pallet::new();

        balance.set_balance(&"alice".to_string(), 100);
        balance.set_balance(&bob, u128::MAX);

        let result = balance.transfer(&alice.clone(), &bob.clone(), 1);

        assert_eq!(result, Err("Overflow when adding to balance"));
        assert_eq!(balance.balance(&"alice".to_string()), 100);
        assert_eq!(balance.balance(&bob), u128::MAX);
    }
}
