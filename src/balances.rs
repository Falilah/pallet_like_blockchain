use super::types;
use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::{collections::BTreeMap, fmt::Debug};

#[derive(Debug)]

pub struct Pallet<AccountId, Balance> {
    balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet<AccountId, Balance>
where
    AccountId: Ord + Clone,
    Balance: Zero + CheckedSub + CheckedAdd + Copy + Debug,
{
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::<AccountId, Balance>::new(),
        }
    }

    pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &AccountId) -> Balance {
        *self.balances.get(who).unwrap_or(&Balance::zero())
    }

    pub fn transfer(
        &mut self,
        caller: &AccountId,
        to: &AccountId,
        amount: Balance,
    ) -> Result<(), &'static str> {
        let caller_bal = self.balance(caller);
        let to_bal = self.balance(to);
        println!("balance before {:?}", caller_bal);

        let new_caller_bal = caller_bal.checked_sub(&amount).ok_or("Not enough funds.")?;
        println!("balance before {:?}", new_caller_bal);

        let new_to_bal = to_bal
            .checked_add(&amount)
            .ok_or("over/underflow occured.")?;

        self.set_balance(caller, new_caller_bal);
        self.set_balance(to, new_to_bal);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_balances() {
        let mut balances = Pallet::<types::AccountId, types::Balance>::new();
        assert_eq!(balances.balance(&"alice".to_string()), 0);
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut transfer = Pallet::<types::AccountId, types::Balance>::new();

        // - That `alice` cannot transfer funds she does not have.
        let result = transfer.transfer(&"Alice".to_string(), &"Bob".to_string(), 100);
        assert_eq!(result, Err("Not enough funds."));

        // set Alice balance to 150
        transfer.set_balance(&"Alice".to_string(), 150);

        // - That `alice` can successfully transfer funds to `bob`.
        let result = transfer.transfer(&"Alice".to_string(), &"Bob".to_string(), 100);
        assert_eq!(result, Ok(()));

        // - That the balance of `alice` and `bob` is correctly updated.
        let alice_new_bal = transfer.balance(&"Alice".to_string());
        assert_eq!(alice_new_bal, 50);

        let bob_new_bal = transfer.balance(&"Bob".to_string());
        assert_eq!(bob_new_bal, 100);
    }
}
