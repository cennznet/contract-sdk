//!
//! Contract bootstrap boiler plate
//! Should be imported into every Substrate contract
//!
#![no_std]
#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]

// Use `wee_alloc` as the wasm memory manager
use wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

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