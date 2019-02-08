//!
//! Contract Runtime SDK
//! Defines a high level API over the contract runtime ABI
//!
#![no_std]
#![feature(alloc)]
#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]

#[macro_use]
extern crate alloc;

mod index;
pub use crate::index::asset;
pub use crate::index::types;
pub mod runtime;
pub mod util;

// Use `wee_alloc` as the wasm memory manager
use wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

/// Useful default imports for writing smart contracts
/// `use contract_sdk::prelude::*`
pub mod prelude {
    pub use alloc::vec;

    pub use crate::runtime::Context;
    pub use crate::runtime::ExecutionContext;
    pub use crate::runtime::Runtime;
    pub use crate::runtime::RuntimeABI;
    pub use alloc::vec::Vec;

    // Stub some error handling functions needed when in a #[no_std] env
    #[panic_handler]
    #[no_mangle]
    pub fn panic(_info: &::core::panic::PanicInfo<'_>) -> ! {
        unsafe { ::core::intrinsics::abort() }
    }

    #[alloc_error_handler]
    pub fn oom(_: ::core::alloc::Layout) -> ! {
        unsafe { ::core::intrinsics::abort() }
    }
}
