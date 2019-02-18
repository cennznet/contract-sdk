//!
//! An index of CENNZnet reference types and constants
//!

/// Mappings to CENNZnet runtime types
pub mod types {
    use primitives::H256;

    // CENNZnet type mappings
    pub type AccountId = H256;
    pub type AssetId = u32;
    pub type Balance = u64;
    pub type Timestamp = u64;
}

/// Mappings to CENNZnet asset IDs
pub mod asset {
    pub const STAKING: u32 = 0;
    pub const SPEND: u32 = 10;
}
