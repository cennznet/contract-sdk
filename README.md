# Contract SDK
A low level API over the CENNZnet contract runtime.  
Contract developers should use this SDK to access behaviour and data from the underlying blockchain.  
Project goals are to provide CENNZnet specific functionality and mappings.  
It operates at a similar layer to Ink core.  

## Development
Intended to compile with `#![no_std]`  
Run: `cargo build`  

## A Minimal Contract
The smallest viable contract can be written with this SDK in ~5 lines of code:
```rust
#![no_std]
use bootstrap_env;
use contract_sdk::prelude::*;

pub extern "C" fn deploy(){}
pub extern "C" fn call(){}
```

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

---
# Contract 101
Included here for posterity, most of this will be abstracted by ink.

## The Prelude
Contracts execute in a sandbox. An ABI is given to provide them with access
to the underlying blockchain storage and other runtime functionality provided by the chain e.g. attestation and transfers.

The ABI itself is quite low-level so we hide it behind a high-level SDK. Since every contract should probably use this external functionality, it's all available for import via the contract SDK prelude.
```rust
use contract_sdk::prelude::*;
```
The prelude includes, among others:
`Runtime` - API to runtime functionality
`Storage` - API for blockchain storage
`Context` - The contract's execution context .e.g timestamp, address, and input payload

## Call and Deploy
Every contract must export two functions named `call` and `deploy`.

The contract VM uses these as entrypoints for contract execution and initialization respectivley.

They should take no arguments and return no parameters, otherwise the
contract will be considered invalid.

```rust
// Called when the contract is invoked
#[no_mangle]
pub extern "C" fn call() {}

// Called once when the contract is instantiated (one time only!)
#[no_mangle]
pub extern "C" fn deploy() {}
```

