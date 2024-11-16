use num::traits::{CheckedAdd, CheckedSub, One, Zero};
use std::ops::AddAssign;
use std::{collections::BTreeMap, fmt::Debug};

pub trait Config: crate::system::Config {
    type Balance: Zero + One + AddAssign + Copy + CheckedSub + CheckedAdd;
    // and more if needed
}

#[derive(Debug)]

pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::<T::AccountId, T::Balance>::new(),
        }
    }

    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }

    pub fn transfer(
        &mut self,
        caller: &T::AccountId,
        to: &T::AccountId,
        amount: T::Balance,
    ) -> crate::support::DispatchResult{
        let caller_bal = self.balance(caller);
        let to_bal = self.balance(to);

        let new_caller_bal = caller_bal.checked_sub(&amount).ok_or("Not enough funds.")?;

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

    struct testConfig;
    impl crate::system::Config for testConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }
    impl Config for testConfig {
        type Balance = u32;
    }

    #[test]
    fn init_balances() {
        let mut balances = Pallet::<testConfig>::new();
        assert_eq!(balances.balance(&"alice".to_string()), 0);
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut transfer = Pallet::<testConfig>::new();

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
