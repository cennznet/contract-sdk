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

// Vendor ink versions
pub use ink_core;
pub use ink_lang;
pub use ink_model;

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

    // Required for macro namespacing
    pub use ink_core::{self};
    pub use ink_lang::{self};
    pub use ink_model::{self};
}
