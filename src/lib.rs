//!
//! Contract Runtime SDK
//! Defines a high level API over the contract runtime ABI
//!
#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate alloc;

pub mod index;

use alloc::vec::Vec;
use index::types::*;
use parity_codec::{Decode, Encode};

/// Transfer `asset_id`@`amount` from this contract's account to a given destination `account`
pub fn generic_asset_transfer(account: AccountId, asset_id: AssetId, amount: Balance) {
    let account_buf = AccountId::encode(&account);
    unsafe {
        cabi::ext_ga_transfer(
            asset_id,
            account_buf.as_ptr() as u32,
            account_buf.len() as u32,
            amount,
        );
    }
}

/// Return the most recent block timestamp
pub fn now() -> Result<Timestamp, &'static str> {
    unsafe {
        cabi::ext_now();
        let timestamp_buf = read_scratch_buffer();
        u64::decode(&mut &timestamp_buf[..]).ok_or("Failed to load timestamp value")
    }
}

// Return remaining gas balance
pub fn gas_left() -> Result<Balance, &'static str> {
    unsafe {
        cabi::ext_gas_left();
        let gas_buf = read_scratch_buffer();
        u64::decode(&mut &gas_buf[..]).ok_or("Failed to load gas value")
    }
}

/// Get an entropy seed from Substrate runtime
pub fn random_seed() -> Vec<u8> {
    unsafe {
        cabi::ext_random_seed();
        read_scratch_buffer()
    }
}

/// Store value under key in storage
pub fn set_storage(key: &StorageKey, value: Option<&[u8]>) {
    unsafe {
        let mut value_ptr = 0;
        let mut value_len = 0;
        let value_non_null = if let Some(v) = value {
            value_ptr = v.as_ptr() as u32;
            value_len = v.len() as u32;
            1
        } else {
            0
        };

        cabi::ext_set_storage(key.0.as_ptr() as u32, value_non_null, value_ptr, value_len);
    }
}

/// Load stored value at key, returns `None` if not found
pub fn get_storage(key: &StorageKey) -> Option<Vec<u8>> {
    const SUCCESS: u32 = 0;
    unsafe {
        let result = cabi::ext_get_storage(key.0.as_ptr() as u32);
        if result != SUCCESS {
            return None;
        }
        Some(read_scratch_buffer())
    }
}

/// Return user input as a buffer
pub fn input() -> Vec<u8> {
    unsafe {
        let input_len = cabi::ext_input_size();
        match input_len {
            len if len > 0 && len < isize::max_value() as u32 => {
                let mut value = vec![0; len as usize];
                cabi::ext_input_copy(value.as_mut_ptr() as u32, 0, len);
                value
            }
            _ => vec![],
        }
    }
}

/// Return the given `data` buffer to the contract caller
pub fn return_(data: &[u8]) -> ! {
    unsafe {
        cabi::ext_return(data.as_ptr() as u32, data.len() as u32);
    }
}

/// Read the contents of the scratch buffer
fn read_scratch_buffer() -> Vec<u8> {
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

/// Bindings to the contract runtime
mod cabi {
    extern "C" {
        pub fn ext_gas_left();
        pub fn ext_ga_transfer(asset_id: u32, account_ptr: u32, account_len: u32, value: u64);
        pub fn ext_now();
        pub fn ext_random_seed();
        pub fn ext_get_storage(key_ptr: u32) -> u32;
        pub fn ext_set_storage(key_ptr: u32, value_non_null: u32, value_ptr: u32, value_len: u32);
        pub fn ext_input_size() -> u32;
        pub fn ext_input_copy(dest_ptr: u32, offset: u32, len: u32);
        pub fn ext_scratch_size() -> u32;
        pub fn ext_scratch_copy(dest_ptr: u32, offset: u32, len: u32);
        pub fn ext_return(data_ptr: u32, data_len: u32) -> !;
    }
}
