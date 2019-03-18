# Contract SDK
A high level API over the CENNZnet contract runtime.
Contract developers should use this SDK to access behaviour and data from the underlying blockchain.

Project goals are to provide a stable integration point for changes from Substrate
and Parity's contract eDSL (https://github.com/Robbepop/pdsl).

## Development
Intended to compile with `#![no_std]`  
Run: `cargo build`  

## Gas Costs
Associated gas costs can be found [here](https://github.com/paritytech/substrate/blob/master/srml/contract/COMPLEXITY.md)  

## Usage
```rust
#![no_std]
use bootstrap; // Bootstraps contract env
use contract_sdk::prelude::*;

#[no_mangle]
pub extern "C" fn call() {
    // Say hello!
    if let Ok(address) = Context::caller() {
        let mut message = b"Hello ".to_vec();
        message.extend_from_slice(address);
        Runtime::log(&message);
    } else {
        Runtime::log(b"Hello nobody...");
    }
}
```
