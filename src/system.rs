use super::types;
use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;
use std::ops::Add;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.

#[derive(Debug)]
///
pub struct Pallet<BlockNumber, AccountId, Nonce> {
    /// The current block number.
    block_number: BlockNumber,

    /// A map from an account to their nonce.
    nonce: BTreeMap<AccountId, Nonce>,
}

impl<BlockNumber, AccountId, Nonce> Pallet<BlockNumber, AccountId, Nonce>
where
    BlockNumber: Zero + CheckedSub + CheckedAdd + Copy + From<u32>,
    AccountId: Ord + Clone,
    Nonce: Zero + CheckedSub + CheckedAdd + Copy + From<u32> + Add<Output = Nonce>,
{
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::<AccountId, Nonce>::new(),
        }
    }

    /// Get the current block number.
    pub fn block_number(&self) -> BlockNumber {
        /* Return the current block number. */
        self.block_number
    }

    pub fn get_nonce(&self, who: &AccountId) -> Nonce {
        /* Return the current block number. */
        *self.nonce.get(who).unwrap_or(&Nonce::zero())
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        /* Increment the current block number by one. */
        self.block_number = self.block_number.checked_add(&1.into()).unwrap();

        // BlockNumber::checked_add(self.block_number, &1.into());
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &AccountId) {
        /* TODO: Get the current nonce of `who`, and increment it by one. */
        let zero = Nonce::zero();
        let old = self.nonce.get(who).unwrap_or(&zero);
        self.nonce.insert(who.clone(), old.add(1.into()));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_system() {
        /* TODO: Create a test which checks the following:
            - Increment the current block number.
            - Increment the nonce of `alice`.
            - Check the block number is what we expect.
            - Check the nonce of `alice` is what we expect.
        */

        let mut system = Pallet::<types::BlockNumber, types::AccountId, types::Nonce>::new();
        system.inc_block_number();
        system.inc_nonce(&"Alice".to_string());
        system.inc_nonce(&"Alice".to_string());
        system.inc_nonce(&"Alice".to_string());

        assert_eq!(system.block_number(), 1);
        assert_eq!(system.get_nonce(&"Alice".to_string()), 3);
    }
}
