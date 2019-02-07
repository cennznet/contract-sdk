//!
//! Contract Runtime SDK
//! Defines a high level API over the contract runtime ABI
//!
#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate alloc;

pub mod index;
pub mod runtime;
pub mod util;

