//!
//! An index of CENNZnet reference types and constants
//!

/// Mappings to CENNZnet runtime types
pub mod types {
    // CENNZnet type mappings
    use ink_core::env::{ContractEnv, DefaultSrmlTypes, EnvTypes};

    pub type AccountId = <ContractEnv<DefaultSrmlTypes> as EnvTypes>::AccountId;
    pub type AssetId = u32;
    pub type Balance = u128;
    pub type Timestamp = u64;
}

/// Mappings to CENNZnet asset IDs
pub mod asset {
    pub const CENNZ: u32 = 16_000;
    pub const CENTRAPAY: u32 = 16_001;
}
