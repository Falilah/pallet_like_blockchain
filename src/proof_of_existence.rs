use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	/// The type which represents the content that can be claimed using this pallet.
	/// Could be the content directly as bytes, or better yet the hash of that content.
	/// We leave that decision to the runtime developer.
	type Content: Debug + Ord;
}


// A public enum which describes the calls we want to expose to the dispatcher.
// We should expect that the caller of each call will be provided by the dispatcher,
// and not included as a parameter of the call.
// pub enum Call<T: Config> {
// 	/*
// 		Remember that you only need to pass in the `claim` data, as `caller` information is passed
// 		in through the `dispatch` logic.
// 	*/

// 	CreateClaim {
//         content: T::Content,
//     },
// 	RevokeClaim {
//         content: T::Content,
//     },
// }

// /// Implementation of the dispatch logic, mapping from `POECall` to the appropriate underlying
// /// function we want to execute.
// impl<T: Config> crate::support::Dispatch for Pallet<T> {
// 	/*
// 		Implement `crate::support::Dispatch` for `Pallet<T>`.

// 		In your `dispatch` logic, match on `call` and forward the `caller` and `claim` data to the
// 		appropriate function.
// 	*/

// 	type Caller = T::AccountId;
//     type Call = Call<T>;

//     fn dispatch(
//         &mut self,
//         caller: Self::Caller,
//         call: Self::Call,
//     ) -> crate::support::DispatchResult {
//         /*use a `match` statement to route the `Call` to the appropriate pallet function. */
//         match call {
//             Call::CreateClaim { content } => {
// 				self.create_claim(caller, content)?
               
//             },
// 			Call::RevokeClaim {  content } =>{
// 				self.revoke_claim(caller, content)?
// 			}
//         }
//         Ok(())
//     }
// }


/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// A simple storage map from content to the owner of that content.
	/// Accounts can make multiple different claims, but each claim can only have one owner.
	// Add a field `claims` which is a `BTreeMap` fom `T::Content` to `T::AccountId`. */
    claims: BTreeMap<T::Content, T::AccountId>
}

#[macros::call]
impl<T: Config> Pallet<T> {
	
	/// Create a new claim on behalf of the `caller`.
	/// This function will return an error if someone already has claimed that content.
	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		/*Check that a `claim` does not already exist. If so, return an error. */
		if self.claims.contains_key(&claim) {
			return Err(&"this content is already claimed");
		}
		/* `insert` the claim on behalf of `caller`. */
		self.claims.insert(claim, caller);
		Ok(())
	}


	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		/* Get the owner of the `claim` to be revoked. */
		let caller_state = self.get_claim(&claim).ok_or("claim does not exist")?;
	
		
		/* Check that the `owner` matches the `caller`. */
		if caller != *caller_state {
			return Err(&"this content is owned by someone else");
		}
		/*If all checks pass, then `remove` the `claim`. */
		self.claims.remove(&claim);
		Ok(())
	}
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

		// let rev = std::panic::catch_unwind(|| {
		// 	poe.revoke_claim(charles, "claimBob")
		// });
		
		
		// assert_eq!(rev, Some("claim does not exist"));


	}
}