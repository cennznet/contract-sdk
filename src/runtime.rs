//!
//! The contract runtime API
//!
use crate::index::types::*;
use alloc::vec::Vec;
use parity_codec::{Decode, Encode};

/// An interface for contract runtime functionality
pub trait RuntimeABI {
    /// Deposit an event on chain
    fn deposit_event(event: &[u8]);
    /// Log a UTF-8 message to the node stdout
    fn log(message: &str);
    /// Returns a data buffer to the caller, terminates immediately.
    fn return_with(data: &[u8]) -> !;
    /// Call stored code at the given `callee` account with `gas` and `input` payload.
    /// It also transfers some `value` to the account
    fn call(callee: AccountId, gas: u64, value: Balance, input: &[u8]);
}

/// An interface over read-only runtime data
/// Note: these calls still incur gas costs, but provide a nicer API presented as a `context`
pub trait ExecutionContext {
    /// Get the caller's account address
    fn caller() -> Result<AccountId, &'static str>;
    /// Get the remaining gas balance for contract execution
    fn gas() -> Result<Balance, &'static str>;
    /// Get the input buffer (payload) from the caller
    fn input() -> Option<Vec<u8>>;
    /// Get the current block's timestamp
    fn now() -> Result<Timestamp, &'static str>;
    /// Get the current block's random seed
    fn random_seed() -> Vec<u8>;
}

/// Provides contextual data of a contract's execution environment
pub struct Context;

impl ExecutionContext for Context {
    /// Get the current block timestamp
    fn now() -> Result<Timestamp, &'static str> {
        unsafe {
            cabi::ext_now();
            let timestamp_buf = read_scratch_buffer();
            u64::decode(&mut &timestamp_buf[..]).ok_or("Failed to load timestamp value")
        }
    }

    /// Get the address of the contract caller
    fn caller() -> Result<AccountId, &'static str> {
        unsafe {
            cabi::ext_caller();
            let caller_buf = read_scratch_buffer();
            Decode::decode(&mut &caller_buf[..]).ok_or("Failed to load caller value")
        }
    }

    /// Get input data from the caller
    fn input() -> Option<Vec<u8>> {
        unsafe {
            let input_len = cabi::ext_input_size();
            match input_len {
                len if len > 0 && len < isize::max_value() as u32 => {
                    let mut value = vec![0; len as usize];
                    cabi::ext_input_copy(value.as_mut_ptr() as u32, 0, len);
                    Some(value)
                }
                _ => None,
            }
        }
    }

    /// Get remaining gas balance
    fn gas() -> Result<Balance, &'static str> {
        unsafe {
            cabi::ext_gas_left();
            let gas_buf = read_scratch_buffer();
            u64::decode(&mut &gas_buf[..]).ok_or("Failed to load gas value")
        }
    }

    /// Get an entropy seed from the Substrate runtime
    fn random_seed() -> Vec<u8> {
        unsafe {
            cabi::ext_random_seed();
            read_scratch_buffer()
        }
    }
}

/// The default RuntimeAPI implementation
#[derive(Default)]
pub struct Runtime;

impl RuntimeABI for Runtime {
    /// Call code as the given `callee` account with initial `gas`, `input` payload,
    /// and transfer some `value`
    fn call(callee: AccountId, gas: u64, value: Balance, input: &[u8]) {
        let callee_buf = Encode::encode(&callee);
        let value_buf = Encode::encode(&value);
        unsafe {
            // TODO: expose exit code
            let _ = cabi::ext_call(
                callee_buf.as_ptr() as u32,
                callee_buf.len() as u32,
                gas,
                value_buf.as_ptr() as u32,
                value_buf.len() as u32,
                input.as_ptr() as u32,
                input.len() as u32,
            );
        }
    }

    /// Deposit an event on chain
    fn deposit_event(event: &[u8]) {
        unsafe {
            cabi::ext_deposit_event(event.as_ptr() as u32, event.len() as u32);
        }
    }

    /// Log the given UTF-8 message to node stdout
    /// Only supported on `--dev` chains, NOOP otherwise
    fn log(message: &str) {
        unsafe {
            cabi::ext_println(
                message.as_bytes().as_ptr() as u32,
                message.as_bytes().len() as u32,
            );
        }
    }

    /// Return the given `data` buffer to the caller
    fn return_with(data: &[u8]) -> ! {
        unsafe {
            cabi::ext_return(data.as_ptr() as u32, data.len() as u32);
        }
    }
}

/// Read the contents of the scratch buffer
pub(crate) fn read_scratch_buffer() -> Vec<u8> {
    unsafe {
        match cabi::ext_scratch_size() {
            len if len > 0 && len < isize::max_value() as u32 => {
                let mut value = vec![0; len as usize];
                cabi::ext_scratch_copy(value.as_mut_ptr() as u32, 0, len);
                value
            }
            _ => vec![],
        }
    }
}

/// Bindings to the Substrate contract runtime
pub(crate) mod cabi {
    extern "C" {
        pub fn ext_caller();
        pub fn ext_gas_left();
        pub fn ext_now();
        pub fn ext_call(
            callee_ptr: u32,
            callee_len: u32,
            gas: u64,
            value_ptr: u32,
            value_len: u32,
            input_data_ptr: u32,
            input_data_len: u32,
        ) -> u32;
        pub fn ext_random_seed();
        pub fn ext_get_storage(key_ptr: u32) -> u32;
        pub fn ext_set_storage(key_ptr: u32, value_non_null: u32, value_ptr: u32, value_len: u32);
        pub fn ext_input_size() -> u32;
        pub fn ext_input_copy(dest_ptr: u32, offset: u32, len: u32);
        pub fn ext_scratch_size() -> u32;
        pub fn ext_scratch_copy(dest_ptr: u32, offset: u32, len: u32);
        pub fn ext_return(data_ptr: u32, data_len: u32) -> !;
        pub fn ext_deposit_event(data_ptr: u32, data_len: u32);
        pub fn ext_println(message_ptr: u32, message_len: u32);
    }
}
