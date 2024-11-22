mod balances;
mod support;
mod system;

use balances::Call;

use crate::support::Dispatch;
mod types {
    use crate::support;

    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
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

}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
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
}

impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet.
    fn new() -> Self {
        /*Create a new `Runtime` by creating new instances of `system` and `balances`. */
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
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
    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice,
                call: RuntimeCall::BalancesTransfer {
                    to: bob,
                    amount: 69,
                },
            },
            support::Extrinsic {
                caller: "Bob".to_string(),
                call: RuntimeCall::BalancesTransfer {
                    to: "Charlie".to_string(),
                    amount: 30,
                },
            },
            support::Extrinsic {
                caller: "alex".to_string(),
                call: RuntimeCall::BalancesTransfer {
                    to: "Charlie".to_string(),
                    amount: 30,
                },
            },
            support::Extrinsic {
                caller: "Alice".to_string(),
                call: RuntimeCall::BalancesTransfer {
                    to: "alex".to_string(),
                    amount: 10,
                },
            },
        ],
    };

    runtime.execute_block(block_1).expect("invalid block");

    // start emulating a block
    /* Increment the block number in system. */
    // runtime.system.inc_block_number();
    // /* Assert the block number is what we expect. */
    // assert!(runtime.system.block_number() == 1);

    // first transaction
    // /* Increment the nonce of `alice`. */
    // runtime.system.inc_nonce(alice);

    // /*Execute a transfer from `alice` to `bob` for 30 tokens.
    //     - The transfer _could_ return an error. We should use `map_err` to print
    //       the error if there is one.
    //     - We should capture the result of the transfer in an unused variable like `_res`.
    // */
    // let _res = runtime
    //     .balances
    //     .transfer(alice, bob, 30)
    //     .map_err(|e| eprintln!("{}", e));

    // // second transaction
    // /* Increment the nonce of `alice` again. */
    // runtime.system.inc_nonce(alice);
    // /* Execute another balance transfer, this time from `alice` to `charlie` for 20. */
    // let _res = runtime
    //     .balances
    //     .transfer(alice, charlie, 20)
    // .map_err(|e| eprintln!("{}", e));

    println!("{:?}", runtime)
}

// mod test {
//     use super::*;
// }
