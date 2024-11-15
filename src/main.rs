mod balances;
mod system;

impl system::Config for Runtime {
    type AccountId = String;
    type BlockNumber = u32;
    type Nonce = u32;
}

impl balances::Config for Runtime {
    type Balance = u32;
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
}

fn main() {
    /* Create a mutable variable `runtime`, which is a new instance of `Runtime`. */
    let mut runtime = Runtime::new();
    let alice = &"Alice".to_string();
    let bob = &"Bob".to_string();
    let charlie = &"Charlie".to_string();

    /* Set the balance of `alice` to 100, allowing us to execute other transactions. */
    runtime.balances.set_balance(alice, 100);

    // start emulating a block
    /* Increment the block number in system. */
    runtime.system.inc_block_number();
    /* Assert the block number is what we expect. */
    assert!(runtime.system.block_number() == 1);

    // first transaction
    /* Increment the nonce of `alice`. */
    runtime.system.inc_nonce(alice);

    /*Execute a transfer from `alice` to `bob` for 30 tokens.
        - The transfer _could_ return an error. We should use `map_err` to print
          the error if there is one.
        - We should capture the result of the transfer in an unused variable like `_res`.
    */

    let _res = runtime
        .balances
        .transfer(alice, bob, 30)
        .map_err(|e| eprintln!("{}", e));

    // second transaction
    /* Increment the nonce of `alice` again. */
    runtime.system.inc_nonce(alice);
    /* Execute another balance transfer, this time from `alice` to `charlie` for 20. */
    let _res = runtime
        .balances
        .transfer(alice, charlie, 20)
        .map_err(|e| eprintln!("{}", e));

    println!("{:?}", runtime)
}

// mod test {
//     use super::*;
// }
