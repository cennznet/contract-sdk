//!
//! Contract Runtime SDK
//! Defines a high level API over the contract runtime ABI
//!
#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate alloc;

mod index;
pub use crate::index::asset;
pub use crate::index::types;
pub mod runtime;
pub mod storage;
pub mod util;

/// Useful default imports for writing smart contracts
/// `use contract_sdk::prelude::*`
pub mod prelude {
    pub use alloc::vec;

    pub use crate::runtime::Context;
    pub use crate::runtime::ExecutionContext;
    pub use crate::runtime::Runtime;
    pub use crate::runtime::RuntimeABI;
    pub use crate::storage::Storage;
    pub use alloc::vec::Vec;
}

#[cfg(feature = "test")]
#[macro_use]
extern crate lazy_static;
