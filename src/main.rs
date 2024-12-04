mod balances;
mod support;
mod system;
mod proof_of_existence;

use crate::support::Dispatch;
mod types {
    use crate::support;

    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Content = &'static str;

    /*Define a concrete `Extrinsic` type using `AccountId` and `RuntimeCall`. */
    pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
    /*Define a concrete `Header` type using `BlockNumber`. */
    pub type Header = support::Header<BlockNumber>;
    /* Define a concrete `Block` type using `Header` and `Extrinsic`. */
    pub type Block = support::Block<Header, Extrinsic>;
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    ProofOfExistence(proof_of_existence::Call<Runtime>),

}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}
impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;
    // Dispatch a call on behalf of a caller. Increments the caller's nonce.
    //
    // Dispatch allows us to identify which underlying module call we want to execute.
    // Note that we extract the `caller` from the extrinsic, and use that information
    // to determine who we are executing the call on behalf of.
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
        match runtime_call {
            RuntimeCall::Balances(call) =>{
                self.balances.dispatch(caller, call)?;
            },
            RuntimeCall::ProofOfExistence(call) =>{
                self.proof_of_existence.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
    /* create a field `system` which is of type `system::Pallet`. */
    system: system::Pallet<Self>,

    /* create a field `balances` which is of type `balances::Pallet`. */
    balances: balances::Pallet<Self>,
    proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet.
    fn new() -> Self {
        /*Create a new `Runtime` by creating new instances of `system` and `balances`. */
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            proof_of_existence: proof_of_existence::Pallet::new()
        }
    }

    // Execute a block of extrinsics. Increments the block number.
    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.system.inc_block_number();
        if self.system.block_number() != block.header.block_number {
            return Err("Block number mismatched");
        }
        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller);

            let _ = self.dispatch(caller, call).map_err(|e| {
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                    block.header.block_number, i, e
                )
            });
            // do stuff with `caller` and `call`
        }

        Ok(())
    }
}

fn main() {
    /* Create a mutable variable `runtime`, which is a new instance of `Runtime`. */
    let mut runtime = Runtime::new();
    let alice = "Alice".to_string();
    let bob = "Bob".to_string();
    let charlie = &"Charlie".to_string();

    /* Set the balance of `alice` to 100, allowing us to execute other transactions. */
    runtime.balances.set_balance(&alice, 100);
    let call = balances::Call::transfer { to: (bob), amount: (69) };
    let call2 = balances::Call::transfer { to: ("Charlie".to_string()), amount: (30) };
    let call3 = balances::Call::transfer { to: ("Charlie".to_string()), amount: (30) };
    let call4 = balances::Call::transfer { to: ("alex".to_string()), amount: (10) };

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice,
                call: RuntimeCall::Balances(call)
            },
            support::Extrinsic {
                caller: "Bob".to_string(),
                call: RuntimeCall::Balances(call2)
            },
            support::Extrinsic {
                caller: "alex".to_string(),
                call: RuntimeCall::Balances(call3)
            },
            support::Extrinsic {
                caller: "Alice".to_string(),
                call: RuntimeCall::Balances(call4)
            },
        ],
    };

    runtime.execute_block(block_1).expect("invalid block");

    let claim1 = proof_of_existence::Call::create_claim { claim:"hash of bob: hello! this is for bob" };
    let claim2 = proof_of_existence::Call::create_claim { claim: "hash of alice: hello! this is for Alice" };
    let claim3 = proof_of_existence::Call::revoke_claim  { claim: "No hash for this claim" };
    let claim4 = proof_of_existence::Call::create_claim { claim: "No hash for this claim" };
    let claim5 = proof_of_existence::Call::revoke_claim  { claim: "No hash for this claim" };
    let claim6 = proof_of_existence::Call::create_claim { claim: "hash of alice: hello! this is for Alice" };

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: "Bob".to_string(),
                call: RuntimeCall::ProofOfExistence(claim1)
            },

            support::Extrinsic {
                caller: "Alice".to_string(),
                call: RuntimeCall::ProofOfExistence(claim2)
            },
            support::Extrinsic {
                caller: "Alice".to_string(),
                call: RuntimeCall::ProofOfExistence(claim3)
            },
            support::Extrinsic {
                caller: "Alice".to_string(),
                call: RuntimeCall::ProofOfExistence(claim4)
            },
             support::Extrinsic {
                caller: "Alice".to_string(),
                call: RuntimeCall::ProofOfExistence(claim5)
            },
            support::Extrinsic {
                caller: "Bob".to_string(),
                call: RuntimeCall::ProofOfExistence(claim6)
            },
        ],
    };

    runtime.execute_block(block_2).expect("invalid block");

   

    println!("{:?}", runtime)
}


