// use super::types;
use num::traits::{One, Zero};
use std::collections::BTreeMap;
use std::ops::AddAssign;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + AddAssign + Copy;
    type Nonce: Zero + One + Copy + AddAssign;
    // and more if needed
}
#[derive(Debug)]
///
pub struct Pallet<T: Config> {
    /// The current block number.
    block_number: T::BlockNumber,

    /// A map from an account to their nonce.
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::<T::AccountId, T::Nonce>::new(),
        }
    }

    /// Get the current block number.
    pub fn block_number(&self) -> T::BlockNumber {
        /* Return the current block number. */
        self.block_number
    }

    pub fn get_nonce(&self, who: &T::AccountId) -> T::Nonce {
        /* Return the current block number. */
        *self.nonce.get(who).unwrap_or(&T::Nonce::zero())
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        /* Increment the current block number by one. */
        self.block_number = self.block_number + One::one();

        // BlockNumber::checked_add(self.block_number, &1.into());
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        /* TODO: Get the current nonce of `who`, and increment it by one. */
        let zero = T::Nonce::zero();
        let old = self.nonce.get(who).unwrap_or(&zero);
        self.nonce.insert(who.clone(), *old + One::one());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    struct testConfig;
    impl Config for testConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        /* TODO: Create a test which checks the following:
            - Increment the current block number.
            - Increment the nonce of `alice`.
            - Check the block number is what we expect.
            - Check the nonce of `alice` is what we expect.
        */
        // let testConfig = Config{AccountId, BlockNumber, Nonce};
        let mut system = Pallet::<testConfig>::new();
        system.inc_block_number();
        system.inc_nonce(&"Alice".to_string());
        system.inc_nonce(&"Alice".to_string());
        system.inc_nonce(&"Alice".to_string());

        assert_eq!(system.block_number(), 1);
        assert_eq!(system.get_nonce(&"Alice".to_string()), 3);
    }
}
