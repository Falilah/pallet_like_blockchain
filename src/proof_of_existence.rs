use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	/// The type which represents the content that can be claimed using this pallet.
	/// Could be the content directly as bytes, or better yet the hash of that content.
	/// We leave that decision to the runtime developer.
	type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// A simple storage map from content to the owner of that content.
	/// Accounts can make multiple different claims, but each claim can only have one owner.
	// Add a field `claims` which is a `BTreeMap` fom `T::Content` to `T::AccountId`. */
    claims: BTreeMap<T::Content, T::AccountId>
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
		/* Return a new instance of the `Pallet` struct. */
        
            Self {
                claims: BTreeMap::<T::Content, T::AccountId>::new(),
            }
        
	}

	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		//`get` the `claim` */
		
		self.claims.get(claim)
	}

	/// Create a new claim on behalf of the `caller`.
	/// This function will return an error if someone already has claimed that content.
	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		/*Check that a `claim` does not already exist. If so, return an error. */
		if self.claims.contains_key(&claim) {
			return Err(&"this content is already claimed");
		}
		/* TODO: `insert` the claim on behalf of `caller`. */
		self.claims.insert(claim, caller);
		Ok(())
	}


	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		/* TODO: Get the owner of the `claim` to be revoked. */
		let caller_state = self.get_claim(&claim).expect("claim does not exist");
	
		
		/* TODO: Check that the `owner` matches the `caller`. */
		assert!(&caller == caller_state);
		/* TODO: If all checks pass, then `remove` the `claim`. */
		self.claims.remove_entry(&claim);
		Ok(())
	}
}


  
#[cfg(test)]
mod test {
    use super::Pallet;

	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		/*
			TODO:
			Create an end to end test verifying the basic functionality of this pallet.
				- Check the initial state is as you expect.
				- Check that all functions work successfully.
				- Check that all error conditions error as expected.
		*/

		let alice = "Alice".to_string();
		let bob = "Bob".to_string();
		let charles = "Charles".to_string();



		let mut poe = Pallet::<TestConfig>::new();
		assert_eq!(poe.get_claim(&"hash of a private id"), None);
		let result: Result<(), &str> = poe.create_claim(alice, &"claim");
		assert_eq!(result, Ok(()));

		let result: Result<(), &str> = poe.create_claim(bob, &"claimBob");
		assert_eq!(result, Ok(()));

		let result: Result<(), &str> = poe.create_claim(charles, &"claimBob");
		assert_eq!(result, Err("this content is already claimed"));

		let bob = "Bob".to_string();

		let rev = poe.revoke_claim(bob, "claimBob");
		assert_eq!(rev, Ok(()));

		let charles = "Charles".to_string();

		// let rev = poe.revoke_claim(charles, "claimBob").err();
		// // assert_eq!(rev, Some("claim does not exist"));


	}
}